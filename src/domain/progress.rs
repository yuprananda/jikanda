// Copyright (C) 2026 Yu Prananda
// SPDX-License-Identifier: GPL-3.0-only

use super::session::Mode;
use chrono::{Days, Local, NaiveDate};
use serde::{Deserialize, Serialize};

fn alarm_enabled() -> bool {
    true
}

fn full_opacity() -> u8 {
    100
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    #[serde(default)]
    pub language: crate::i18n::Language,
    pub deep_seconds: u64,
    pub interval_seconds: u64,
    pub break_seconds: u64,
    pub cycles: u32,
    pub last_mode: Mode,
    pub dark: bool,
    pub notifications: bool,
    #[serde(default = "alarm_enabled")]
    pub alarm_enabled: bool,
    #[serde(default)]
    pub alarm_sound: super::alarm::AlarmSound,
    pub always_on_top: bool,
    pub floating_decorated: bool,
    #[serde(default = "full_opacity")]
    pub floating_opacity: u8,
    pub main_decorated: bool,
    pub show_scene: bool,
    pub export_directory: String,
    #[serde(default)]
    pub display_name: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: crate::i18n::Language::English,
            deep_seconds: 3000,
            interval_seconds: 1500,
            break_seconds: 300,
            cycles: 4,
            last_mode: Mode::DeepFocus,
            dark: true,
            notifications: true,
            alarm_enabled: true,
            alarm_sound: super::alarm::AlarmSound::default(),
            always_on_top: true,
            floating_decorated: false,
            floating_opacity: 100,
            main_decorated: false,
            show_scene: true,
            export_directory: String::new(),
            display_name: String::new(),
        }
    }
}

impl Settings {
    /// Product durations are fixed; legacy JSON fields remain readable.
    pub fn use_fixed_durations(&mut self) {
        self.deep_seconds = 3000;
        self.interval_seconds = 1500;
        self.break_seconds = 300;
        self.cycles = 4;
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.display_name.chars().count() > 60 {
            return Err("Nama tampilan maksimal 60 karakter.".into());
        }
        if !(50..=100).contains(&self.floating_opacity) {
            return Err("Opacity timer mini harus 50–100%.".into());
        }
        if !(1..=86400).contains(&self.deep_seconds)
            || !(1..=86400).contains(&self.interval_seconds)
            || !(1..=7200).contains(&self.break_seconds)
            || !(1..=12).contains(&self.cycles)
        {
            return Err("Durasi fokus 1–86400 detik, istirahat 1–7200 detik, siklus 1–12.".into());
        }
        if !self.export_directory.is_empty()
            && !std::path::Path::new(&self.export_directory).is_absolute()
        {
            return Err("Folder ekspor harus berupa path absolut.".into());
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    Completed,
    Interrupted,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Record {
    pub id: u64,
    pub mode: Mode,
    pub started_ms: i64,
    pub ended_ms: i64,
    pub local_day: NaiveDate,
    pub focus_ms: u64,
    pub outcome: Outcome,
    pub note: String,
    #[serde(default)]
    pub session_name: String,
    #[serde(default)]
    pub tasks: super::tasks::Tasks,
}

impl Record {
    /// Only completed task titles are eligible for the explicit share opt-in.
    pub fn share_tasks(&self) -> String {
        self.tasks
            .entries
            .iter()
            .filter(|task| task.completed)
            .map(|task| format!("\u{2713} {}", task.title))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn validate(&self) -> Result<(), String> {
        self.tasks.validate()?;
        if self.id == 0
            || self.focus_ms == 0
            || self.focus_ms > 366 * 86400 * 1000
            || self.note.chars().count() > 160
            || self.session_name.chars().count() > 120
            || chrono::DateTime::from_timestamp_millis(self.started_ms).is_none()
            || chrono::DateTime::from_timestamp_millis(self.ended_ms).is_none()
        {
            return Err("Entri riwayat tidak valid.".into());
        }
        // Wall-clock endpoints may move backwards; duration remains monotonic.
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Data {
    pub version: u32,
    pub next_id: u64,
    pub settings: Settings,
    pub records: Vec<Record>,
    #[serde(default)]
    pub tasks: super::tasks::Tasks,
    #[serde(default)]
    pub session_name: String,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            version: 1,
            next_id: 1,
            settings: Settings::default(),
            records: Vec::new(),
            tasks: super::tasks::Tasks::default(),
            session_name: String::new(),
        }
    }
}

impl Data {
    pub fn new_session(&mut self) {
        self.session_name.clear();
        self.tasks = super::tasks::Tasks::default();
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.version != 1 {
            return Err("Versi data belum didukung; data tidak diubah.".into());
        }
        self.settings.validate()?;
        self.tasks.validate()?;
        if self.session_name.chars().count() > 120 {
            return Err("Nama sesi maksimal 120 karakter.".into());
        }
        if self.records.len() > 10000 {
            return Err(
                "Batas 10000 entri tercapai; ekspor/cadangkan data sebelum melanjutkan.".into(),
            );
        }
        let mut ids = std::collections::HashSet::new();
        for record in &self.records {
            record.validate()?;
            if !ids.insert(record.id) || record.id >= self.next_id {
                return Err("ID riwayat tidak valid.".into());
            }
        }
        if self.next_id == 0 || self.next_id == u64::MAX {
            return Err("ID berikutnya tidak valid.".into());
        }
        Ok(())
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Summary {
    pub focus_ms: u64,
    pub sessions: usize,
    pub completed: usize,
}

pub fn summarize(records: &[Record], start: NaiveDate, end: NaiveDate) -> Summary {
    let mut result = Summary::default();
    for r in records
        .iter()
        .filter(|r| (start..=end).contains(&r.local_day))
    {
        result.focus_ms += r.focus_ms;
        result.sessions += 1;
        result.completed += usize::from(r.outcome == Outcome::Completed);
    }
    result
}

pub fn today() -> NaiveDate {
    Local::now().date_naive()
}
pub fn week_start(end: NaiveDate) -> NaiveDate {
    end.checked_sub_days(Days::new(6)).unwrap_or(end)
}
pub fn duration_label(ms: u64) -> String {
    let seconds = ms / 1000;
    if ms > 0 && seconds == 0 {
        return "<1 detik".into();
    }
    if seconds < 60 {
        format!("{seconds} detik")
    } else if seconds < 3600 {
        format!("{} menit", seconds / 60)
    } else {
        format!("{} jam {} menit", seconds / 3600, seconds % 3600 / 60)
    }
}

/// Compact display for share cards; never rounds recorded time upwards.
pub fn share_duration(ms: u64) -> String {
    let seconds = ms / 1000;
    if ms > 0 && seconds == 0 {
        return "<1s".into();
    }
    if seconds < 60 {
        return format!("{seconds}s");
    }
    let minutes = seconds / 60;
    if minutes < 60 {
        return if seconds.is_multiple_of(60) {
            format!("{minutes}m")
        } else {
            format!("{minutes}m {}s", seconds % 60)
        };
    }
    if minutes.is_multiple_of(60) {
        format!("{}h", minutes / 60)
    } else {
        format!("{}h {}m", minutes / 60, minutes % 60)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn named_sessions_keep_independent_task_snapshots() {
        let mut data = Data {
            session_name: "Learn Rust".into(),
            ..Data::default()
        };
        data.tasks.add("Read chapter one").unwrap();
        data.records.push(Record {
            id: 1,
            mode: Mode::DeepFocus,
            started_ms: 0,
            ended_ms: 1000,
            local_day: NaiveDate::from_ymd_opt(2026, 11, 7).unwrap(),
            focus_ms: 1000,
            outcome: Outcome::Completed,
            note: String::new(),
            session_name: data.session_name.clone(),
            tasks: data.tasks.clone(),
        });
        let mut share_record = data.records[0].clone();
        assert!(share_record.share_tasks().is_empty());
        share_record.tasks.toggle(1);
        share_record.tasks.add("Unfinished private task").unwrap();
        assert!(share_record.share_tasks().contains("Read chapter one"));
        assert!(
            !share_record
                .share_tasks()
                .contains("Unfinished private task")
        );
        data.next_id = 2;
        data.tasks.toggle(1);
        assert!(!data.records[0].tasks.entries[0].completed);
        data.new_session();
        assert_eq!(data.session_name, "");
        assert!(data.tasks.entries.is_empty());
        data.session_name = "Write a blog".into();
        data.tasks.add("Outline").unwrap();
        assert_eq!(data.records[0].session_name, "Learn Rust");
        assert_eq!(data.records[0].tasks.entries[0].title, "Read chapter one");
        let copy: Data = serde_json::from_str(&serde_json::to_string(&data).unwrap()).unwrap();
        assert_eq!(copy, data);
        copy.validate().unwrap();
        let mut legacy = serde_json::to_value(&data.records[0]).unwrap();
        let fields = legacy.as_object_mut().unwrap();
        fields.remove("session_name");
        fields.remove("tasks");
        fields.insert("note".into(), serde_json::json!("An old note"));
        let legacy: Record = serde_json::from_value(legacy).unwrap();
        assert_eq!(legacy.note, "An old note");
        assert!(legacy.session_name.is_empty());
        assert!(legacy.tasks.entries.is_empty());
        legacy.validate().unwrap();
    }
    #[test]
    fn legacy_alarm_defaults_and_muted_sound_choice_roundtrip() {
        let mut json = serde_json::to_value(Settings::default()).unwrap();
        json.as_object_mut().unwrap().remove("alarm_enabled");
        json.as_object_mut().unwrap().remove("alarm_sound");
        let mut settings: Settings = serde_json::from_value(json).unwrap();
        assert!(settings.alarm_enabled);
        assert_eq!(
            settings.alarm_sound,
            super::super::alarm::AlarmSound::SoftBell
        );
        settings.alarm_enabled = false;
        settings.notifications = true;
        settings.alarm_sound = super::super::alarm::AlarmSound::DoubleChime;
        let loaded: Settings =
            serde_json::from_str(&serde_json::to_string(&settings).unwrap()).unwrap();
        assert!(!loaded.alarm_enabled);
        assert!(loaded.notifications);
        assert_eq!(
            loaded.alarm_sound,
            super::super::alarm::AlarmSound::DoubleChime
        );
        assert!(super::super::alarm::AlarmSound::from_index(3).is_err());
    }

    #[test]
    fn display_name_is_optional_for_legacy_settings_and_validates_unicode_length() {
        let mut json = serde_json::to_value(Settings::default()).unwrap();
        json.as_object_mut().unwrap().remove("display_name");
        let mut settings: Settings = serde_json::from_value(json).unwrap();
        assert!(settings.display_name.is_empty());
        settings.display_name = "\u{540d}".repeat(60);
        settings.validate().unwrap();
        let restored: Settings =
            serde_json::from_str(&serde_json::to_string(&settings).unwrap()).unwrap();
        assert_eq!(restored.display_name, settings.display_name);
        settings.display_name.push('x');
        assert!(settings.validate().is_err());
    }

    #[test]
    fn opacity_defaults_for_old_settings_and_rejects_invisible_values() {
        let mut json = serde_json::to_value(Settings::default()).unwrap();
        json.as_object_mut().unwrap().remove("floating_opacity");
        let mut settings: Settings = serde_json::from_value(json).unwrap();
        assert_eq!(settings.floating_opacity, 100);
        for value in [0, 49, 101] {
            settings.floating_opacity = value;
            assert!(settings.validate().is_err());
        }
        for value in [50, 100] {
            settings.floating_opacity = value;
            settings.validate().unwrap();
        }
    }
    #[test]
    fn share_duration_preserves_short_sessions_and_never_rounds_up() {
        assert_eq!(share_duration(500), "<1s");
        assert_eq!(share_duration(8000), "8s");
        assert_eq!(share_duration(90000), "1m 30s");
        assert_eq!(share_duration(9_300_999), "2h 35m");
        assert_eq!(share_duration(3_600_000), "1h");
    }
    #[test]
    fn legacy_data_without_tasks_loads_an_empty_list() {
        let mut json = serde_json::to_value(Data::default()).unwrap();
        json.as_object_mut().unwrap().remove("tasks");
        json.as_object_mut().unwrap().remove("session_name");
        let loaded: Data = serde_json::from_value(json).unwrap();
        assert!(loaded.tasks.entries.is_empty());
        assert!(loaded.session_name.is_empty());
        loaded.validate().unwrap();
    }
    #[test]
    fn fixed_durations_replace_legacy_preferences_without_changing_history() {
        let mut settings = Settings {
            deep_seconds: 600,
            interval_seconds: 120,
            break_seconds: 60,
            cycles: 2,
            ..Settings::default()
        };
        settings.use_fixed_durations();
        assert_eq!(
            (
                settings.deep_seconds,
                settings.interval_seconds,
                settings.break_seconds,
                settings.cycles
            ),
            (3000, 1500, 300, 4)
        );
        let legacy: Mode = serde_json::from_str("\"Interval\"").unwrap();
        assert_eq!(legacy.label(), "Pomodoro");
        assert_eq!(serde_json::to_string(&legacy).unwrap(), "\"Interval\"");
    }

    #[test]
    fn seven_days_crosses_month_and_year() {
        let end = NaiveDate::from_ymd_opt(2027, 1, 3).unwrap();
        assert_eq!(
            week_start(end),
            NaiveDate::from_ymd_opt(2026, 12, 28).unwrap()
        );
    }
    #[test]
    fn explicit_interruption_counts_but_outside_dates_does_not() {
        let day = NaiveDate::from_ymd_opt(2026, 11, 6).unwrap();
        let mut data = Data {
            next_id: 3,
            ..Data::default()
        };
        for (id, date) in [(1, day), (2, day.succ_opt().unwrap())] {
            data.records.push(Record {
                id,
                mode: Mode::Flow,
                started_ms: 0,
                ended_ms: 10000,
                local_day: date,
                focus_ms: 5000,
                outcome: Outcome::Interrupted,
                note: String::new(),
                session_name: String::new(),
                tasks: super::super::tasks::Tasks::default(),
            });
        }
        assert_eq!(
            summarize(&data.records, day, day),
            Summary {
                focus_ms: 5000,
                sessions: 1,
                completed: 0
            }
        );
        data.records[1].id = 1;
        assert!(data.validate().is_err());
    }
    #[test]
    fn future_schema_and_invalid_settings_are_rejected() {
        let mut data = Data {
            version: 2,
            ..Data::default()
        };
        assert!(data.validate().is_err());
        data.version = 1;
        data.settings.cycles = 0;
        assert!(data.validate().is_err());
    }
}
