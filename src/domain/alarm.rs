// Copyright (C) 2026 Yu Prananda
// SPDX-License-Identifier: GPL-3.0-only

use super::session::{Mode, Phase, Session, State};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlarmSound {
    #[default]
    SoftBell,
    DoubleChime,
    BrightTone,
}

impl AlarmSound {
    pub fn index(self) -> i32 {
        match self {
            Self::SoftBell => 0,
            Self::DoubleChime => 1,
            Self::BrightTone => 2,
        }
    }

    pub fn from_index(index: i32) -> Result<Self, String> {
        match index {
            0 => Ok(Self::SoftBell),
            1 => Ok(Self::DoubleChime),
            2 => Ok(Self::BrightTone),
            _ => Err("Pilihan alarm tidak valid.".into()),
        }
    }
}

/// Consume completion cues even when muted, so enabling sound later never replays
/// an old cue. Independent from persistence success and UI refresh frequency.
#[derive(Default)]
pub struct AlarmEvents(Option<(State, Phase, u32)>);

impl AlarmEvents {
    pub fn poll(&mut self, session: &Session) -> bool {
        if session.mode == Mode::Flow
            || !(session.state == State::Completed
                || (session.state == State::Paused && session.awaiting_next))
        {
            return false;
        }
        let cue = (session.state, session.phase, session.cycle);
        if self.0 == Some(cue) {
            return false;
        }
        self.0 = Some(cue);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    fn s(value: u64) -> Duration {
        Duration::from_secs(value)
    }

    #[test]
    fn countdown_and_every_interval_boundary_emit_once() {
        let mut timer = Session::configured(Mode::Interval, s(1), s(1), 2).unwrap();
        let mut events = AlarmEvents::default();
        assert!(!events.poll(&timer));
        timer.start_or_resume(s(0));
        timer.sample(s(1));
        assert!(events.poll(&timer));
        assert!(!events.poll(&timer));
        timer.start_or_resume(s(2));
        timer.sample(s(3));
        assert!(events.poll(&timer));
        assert!(!events.poll(&timer));
        timer.start_or_resume(s(4));
        timer.sample(s(5));
        assert!(events.poll(&timer));
        assert!(!events.poll(&timer));
        let mut deep = Session::new(s(1)).unwrap();
        let mut events = AlarmEvents::default();
        deep.start_or_resume(s(0));
        deep.sample(s(1));
        assert!(events.poll(&deep));
        assert!(!events.poll(&deep));
    }

    #[test]
    fn muted_cues_are_consumed_and_stopping_at_boundary_does_not_ring_again() {
        let mut timer = Session::configured(Mode::Interval, s(1), s(1), 2).unwrap();
        let mut events = AlarmEvents::default();
        timer.start_or_resume(s(0));
        timer.sample(s(1));
        let muted = true;
        let play = events.poll(&timer) && !muted;
        assert!(!play);
        assert!(
            !events.poll(&timer),
            "Unmuting must not replay an old event"
        );
        timer.stop(s(2));
        assert!(!events.poll(&timer));
    }

    #[test]
    fn stop_pause_sleep_and_manual_flow_completion_are_silent() {
        let mut timer = Session::new(s(10)).unwrap();
        let mut events = AlarmEvents::default();
        timer.start_or_resume(s(0));
        timer.pause(s(1));
        assert!(!events.poll(&timer));
        timer.start_or_resume(s(2));
        timer.suspend();
        assert!(!events.poll(&timer));
        timer.stop(s(3));
        assert!(!events.poll(&timer));
        let mut flow = Session::configured(Mode::Flow, s(1), s(1), 1).unwrap();
        flow.start_or_resume(s(0));
        flow.finish_flow(s(1));
        assert!(!events.poll(&flow));
    }
}
