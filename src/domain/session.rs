// Copyright (C) 2026 Yu Prananda
// SPDX-License-Identifier: GPL-3.0-only

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Interval,
    #[default]
    DeepFocus,
    Flow,
}

impl Mode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Interval => "Pomodoro",
            Self::DeepFocus => "Deep Focus",
            Self::Flow => "Flow",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Focus,
    Break,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Ready,
    Running,
    Paused,
    Interrupted,
    Completed,
}

/// Times are monotonic offsets supplied by the application, never UI tick counts.
pub struct Session {
    pub state: State,
    target: Duration,
    accumulated: Duration,
    started: Duration,
    last_trusted: Duration,
    pub uncertain: bool,
    pub mode: Mode,
    pub phase: Phase,
    pub cycle: u32,
    pub cycles: u32,
    pub awaiting_next: bool,
    focus_target: Duration,
    break_target: Duration,
    focus_total: Duration,
}

impl Session {
    #[cfg(test)]
    pub fn new(target: Duration) -> Result<Self, &'static str> {
        Self::configured(Mode::DeepFocus, target, Duration::from_secs(300), 1)
    }

    pub fn configured(
        mode: Mode,
        target: Duration,
        break_target: Duration,
        cycles: u32,
    ) -> Result<Self, &'static str> {
        if target.is_zero() {
            return Err("Duration must be greater than zero");
        }
        if target > Duration::from_secs(86400)
            || break_target.is_zero()
            || break_target > Duration::from_secs(7200)
            || !(1..=12).contains(&cycles)
        {
            return Err("Invalid session configuration");
        }
        Ok(Self {
            state: State::Ready,
            target,
            accumulated: Duration::ZERO,
            started: Duration::ZERO,
            last_trusted: Duration::ZERO,
            uncertain: false,
            mode,
            phase: Phase::Focus,
            cycle: 1,
            cycles,
            awaiting_next: false,
            focus_target: target,
            break_target,
            focus_total: Duration::ZERO,
        })
    }

    pub fn start_or_resume(&mut self, now: Duration) {
        if self.state == State::Running {
            return;
        }
        if matches!(self.state, State::Completed | State::Interrupted) {
            self.accumulated = Duration::ZERO;
            self.focus_total = Duration::ZERO;
            self.phase = Phase::Focus;
            self.cycle = 1;
            self.target = self.focus_target;
        }
        if self.awaiting_next {
            self.accumulated = Duration::ZERO;
            self.awaiting_next = false;
        }
        self.started = now;
        self.last_trusted = now;
        self.uncertain = false;
        self.state = State::Running;
    }

    pub fn sample(&mut self, now: Duration) {
        if self.state != State::Running {
            return;
        }
        // A missed interval may be sleep, a stalled event loop, or scheduling delay.
        // Discard the unverified tail rather than crediting it as focus time.
        if now.saturating_sub(self.last_trusted) > Duration::from_secs(3) {
            self.pause(self.last_trusted);
            self.uncertain = true;
            return;
        }
        self.last_trusted = now;
        if self.mode != Mode::Flow && self.elapsed(now) >= self.target {
            self.advance();
        }
    }

    pub fn pause(&mut self, now: Duration) {
        if self.state == State::Running {
            self.accumulated = self.elapsed(now);
            if self.mode != Mode::Flow && self.accumulated >= self.target {
                self.advance();
            } else {
                self.state = State::Paused;
            }
        }
    }

    pub fn suspend(&mut self) {
        if self.state == State::Running {
            self.pause(self.last_trusted);
            self.uncertain = true;
        }
    }

    pub fn stop(&mut self, now: Duration) {
        self.sample(now);
        if matches!(self.state, State::Running | State::Paused) {
            self.pause(now);
            if self.state != State::Completed {
                self.state = State::Interrupted;
                self.uncertain = false;
            }
        }
    }

    pub fn remaining(&self, now: Duration) -> Duration {
        self.target.saturating_sub(self.elapsed(now))
    }

    pub fn focus_elapsed(&self, now: Duration) -> Duration {
        self.focus_total
            + if self.phase == Phase::Focus {
                if self.mode == Mode::Flow {
                    self.elapsed(now)
                } else {
                    self.elapsed(now).min(self.target)
                }
            } else {
                Duration::ZERO
            }
    }

    pub fn finish_flow(&mut self, now: Duration) {
        self.sample(now);
        if self.mode == Mode::Flow && matches!(self.state, State::Running | State::Paused) {
            self.pause(now);
            self.state = State::Completed;
            self.uncertain = false;
        }
    }

    fn advance(&mut self) {
        self.accumulated = self.target;
        if self.mode != Mode::Interval || (self.phase == Phase::Focus && self.cycle == self.cycles)
        {
            self.state = State::Completed;
            return;
        }
        if self.phase == Phase::Focus {
            self.focus_total += self.target;
            self.phase = Phase::Break;
            self.target = self.break_target;
        } else {
            self.phase = Phase::Focus;
            self.cycle += 1;
            self.target = self.focus_target;
        }
        self.accumulated = Duration::ZERO;
        self.awaiting_next = true;
        self.state = State::Paused;
    }

    pub fn target(&self) -> Duration {
        self.target
    }
    pub fn focus_target(&self) -> Duration {
        self.focus_target
    }

    fn elapsed(&self, now: Duration) -> Duration {
        self.accumulated
            + if self.state == State::Running {
                now.saturating_sub(self.started)
            } else {
                Duration::ZERO
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn s(n: u64) -> Duration {
        Duration::from_secs(n)
    }

    #[test]
    fn rejects_zero() {
        assert!(Session::new(s(0)).is_err());
    }

    #[test]
    fn rapid_pause_resume_does_not_double_count() {
        let mut timer = Session::new(s(10)).unwrap();
        timer.start_or_resume(s(0));
        timer.pause(s(2));
        timer.pause(s(8));
        timer.start_or_resume(s(9));
        timer.start_or_resume(s(10));
        timer.sample(s(11));
        assert_eq!(timer.remaining(s(11)), s(6));
    }

    #[test]
    fn completion_is_terminal_until_explicit_restart() {
        let mut timer = Session::new(s(2)).unwrap();
        timer.start_or_resume(s(0));
        timer.sample(s(2));
        timer.stop(s(3));
        timer.sample(s(9));
        assert_eq!(timer.state, State::Completed);
        assert_eq!(timer.remaining(s(9)), s(0));
    }

    #[test]
    fn delayed_callback_uses_time_not_tick_count() {
        let mut timer = Session::new(s(10)).unwrap();
        timer.start_or_resume(s(0));
        timer.sample(s(2));
        assert_eq!(timer.remaining(s(2)), s(8));
    }

    #[test]
    fn uncertain_gap_excludes_unverified_time() {
        let mut timer = Session::new(s(10)).unwrap();
        timer.start_or_resume(s(0));
        timer.sample(s(1));
        timer.sample(s(100));
        assert_eq!(timer.state, State::Paused);
        assert_eq!(timer.remaining(s(100)), s(9));
        assert!(timer.uncertain);
    }

    #[test]
    fn suspend_requires_manual_resume() {
        let mut timer = Session::new(s(10)).unwrap();
        timer.start_or_resume(s(0));
        timer.sample(s(2));
        timer.suspend();
        timer.sample(s(200));
        assert_eq!(timer.remaining(s(200)), s(8));
        timer.start_or_resume(s(200));
        timer.sample(s(201));
        assert_eq!(timer.remaining(s(201)), s(7));
    }

    #[test]
    fn stop_is_interruption_and_restart_is_fresh() {
        let mut timer = Session::new(s(10)).unwrap();
        timer.start_or_resume(s(0));
        timer.stop(s(2));
        assert_eq!(timer.state, State::Interrupted);
        timer.start_or_resume(s(5));
        assert_eq!(timer.remaining(s(5)), s(10));
    }

    #[test]
    fn interval_waits_for_next_phase_and_excludes_breaks() {
        let mut timer = Session::configured(Mode::Interval, s(2), s(1), 2).unwrap();
        timer.start_or_resume(s(0));
        timer.sample(s(1));
        timer.sample(s(2));
        assert_eq!(timer.phase, Phase::Break);
        assert_eq!(timer.state, State::Paused);
        timer.sample(s(99));
        assert_eq!(timer.focus_elapsed(s(99)), s(2));
        timer.start_or_resume(s(100));
        timer.sample(s(101));
        assert_eq!(timer.phase, Phase::Focus);
        assert_eq!(timer.cycle, 2);
        assert_eq!(timer.focus_elapsed(s(101)), s(2));
        timer.start_or_resume(s(102));
        timer.sample(s(103));
        timer.sample(s(104));
        assert_eq!(timer.state, State::Completed);
        assert_eq!(timer.focus_elapsed(s(1000)), s(4));
        timer.stop(s(1001));
        assert_eq!(timer.focus_elapsed(s(1001)), s(4));
    }

    #[test]
    fn interruption_during_break_preserves_only_focus() {
        let mut timer = Session::configured(Mode::Interval, s(2), s(10), 2).unwrap();
        timer.start_or_resume(s(0));
        timer.sample(s(2));
        timer.start_or_resume(s(3));
        timer.sample(s(4));
        timer.stop(s(5));
        assert_eq!(timer.state, State::Interrupted);
        assert_eq!(timer.focus_elapsed(s(5)), s(2));
        timer.start_or_resume(s(10));
        assert_eq!(timer.phase, Phase::Focus);
        assert_eq!(timer.focus_elapsed(s(10)), s(0));
    }

    #[test]
    fn flow_counts_past_target_and_requires_explicit_completion() {
        let mut timer = Session::configured(Mode::Flow, s(1), s(1), 1).unwrap();
        timer.start_or_resume(s(0));
        for now in 1..=4 {
            timer.sample(s(now));
        }
        assert_eq!(timer.state, State::Running);
        assert_eq!(timer.focus_elapsed(s(4)), s(4));
        timer.pause(s(4));
        timer.start_or_resume(s(20));
        timer.sample(s(21));
        timer.finish_flow(s(22));
        assert_eq!(timer.state, State::Completed);
        assert_eq!(timer.focus_elapsed(s(100)), s(6));
    }

    #[test]
    fn invalid_interval_configuration_is_rejected() {
        assert!(Session::configured(Mode::Interval, s(1), s(0), 2).is_err());
        assert!(Session::configured(Mode::Interval, s(1), s(1), 0).is_err());
        assert!(Session::configured(Mode::Interval, s(1), s(1), 13).is_err());
    }
}
