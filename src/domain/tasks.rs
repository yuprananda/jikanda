// Copyright (C) 2026 Yu Prananda
// SPDX-License-Identifier: GPL-3.0-only

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tasks {
    pub entries: Vec<Task>,
    pub active_id: Option<u64>,
    pub next_id: u64,
}

impl Default for Tasks {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            active_id: None,
            next_id: 1,
        }
    }
}

impl Tasks {
    pub fn add(&mut self, title: &str) -> Result<(), String> {
        let title = title.trim();
        if title.is_empty() || title.chars().count() > 120 {
            return Err("Tulis tugas sepanjang 1–120 karakter.".into());
        }
        if self.entries.len() >= 32 || self.next_id >= u64::MAX - 1 {
            return Err("Maksimal 32 tugas. Hapus tugas yang tidak diperlukan.".into());
        }
        let id = self.next_id;
        self.next_id += 1;
        self.entries.push(Task {
            id,
            title: title.into(),
            completed: false,
        });
        if self.active_id.is_none() {
            self.active_id = Some(id);
        }
        Ok(())
    }

    pub fn toggle(&mut self, id: u64) {
        if let Some(task) = self.entries.iter_mut().find(|task| task.id == id) {
            task.completed = !task.completed;
            if task.completed && self.active_id == Some(id) {
                self.active_id = None;
            }
        }
    }

    pub fn select(&mut self, id: u64) {
        if self
            .entries
            .iter()
            .any(|task| task.id == id && !task.completed)
        {
            self.active_id = if self.active_id == Some(id) {
                None
            } else {
                Some(id)
            };
        }
    }

    pub fn delete(&mut self, id: u64) {
        self.entries.retain(|task| task.id != id);
        if self.active_id == Some(id) {
            self.active_id = None;
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        let mut ids = std::collections::HashSet::new();
        if self.entries.len() > 32
            || self.next_id == 0
            || self.next_id == u64::MAX
            || self.entries.iter().any(|task| {
                task.id == 0
                    || task.id >= self.next_id
                    || !ids.insert(task.id)
                    || task.title.trim().is_empty()
                    || task.title.chars().count() > 120
            })
            || self.active_id.is_some_and(|id| {
                !self
                    .entries
                    .iter()
                    .any(|task| task.id == id && !task.completed)
            })
        {
            return Err("Daftar tugas lokal tidak valid.".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn selection_completion_and_deletion_stay_consistent() {
        let mut tasks = Tasks::default();
        tasks.add("  Write a page  ").unwrap();
        tasks.add("Read Rust").unwrap();
        assert_eq!(tasks.entries[0].title, "Write a page");
        assert_eq!(tasks.active_id, Some(1));
        tasks.select(2);
        tasks.toggle(2);
        assert_eq!(tasks.active_id, None);
        tasks.select(2);
        assert_eq!(tasks.active_id, None);
        tasks.toggle(2);
        tasks.select(2);
        tasks.delete(2);
        assert_eq!(tasks.active_id, None);
        tasks.validate().unwrap();
        let restored: Tasks =
            serde_json::from_str(&serde_json::to_string(&tasks).unwrap()).unwrap();
        assert_eq!(tasks, restored);
    }
    #[test]
    fn limits_and_invalid_references_are_rejected() {
        let mut tasks = Tasks::default();
        assert!(tasks.add("  ").is_err());
        assert!(tasks.add(&"a".repeat(121)).is_err());
        for _ in 0..32 {
            tasks.add("Task").unwrap();
        }
        assert!(tasks.add("Extra").is_err());
        tasks.active_id = Some(999);
        assert!(tasks.validate().is_err());
    }
}
