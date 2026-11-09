// Copyright (C) 2026 Yu Prananda
// SPDX-License-Identifier: GPL-3.0-only

mod catalog;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    #[default]
    English,
    Indonesian,
}

pub fn bind(global: crate::I18n<'_>, language: Language) {
    global.on_english(|text| english(&text).into());
    global.set_indonesian(language == Language::Indonesian);
}

pub fn translate(language: Language, text: &str) -> String {
    match language {
        Language::English => english(text),
        Language::Indonesian => text.to_owned(),
    }
}

pub fn english(text: &str) -> String {
    translate_message(text, 0)
}

fn translate_message(text: &str, depth: u8) -> String {
    if let Some((_, translated)) = catalog::MESSAGES.iter().find(|(source, _)| *source == text) {
        return (*translated).to_owned();
    }
    if depth < 4 {
        // Prefer a whole hours/minutes message over its shorter minutes suffix.
        let matched = catalog::MESSAGES
            .iter()
            .filter_map(|&(source, translated)| {
                if !source.contains('{') {
                    return None;
                }
                match_template(source, text)
                    .map(|values| (literal_weight(source), source, translated, values))
            })
            .max_by_key(|(weight, _, _, _)| *weight);
        if let Some((_, source, translated, values)) = matched {
            let mut output = String::new();
            let mut rest = translated;
            let mut index = 0;
            while let Some(start) = rest.find('{') {
                let Some(end) = rest[start..].find('}') else {
                    break;
                };
                output.push_str(&rest[..start]);
                if source == "PNG tersimpan: {}" {
                    output.push_str(values[index]);
                } else {
                    output.push_str(&translate_message(values[index], depth + 1));
                }
                rest = &rest[start + end + 1..];
                if values[index] == "1" {
                    for plural in [" seconds", " minutes", " hours", " sessions"] {
                        if rest.starts_with(plural) {
                            output.push_str(&plural[..plural.len() - 1]);
                            rest = &rest[plural.len()..];
                            break;
                        }
                    }
                }
                index += 1;
            }
            output.push_str(rest);
            return output;
        }
    }
    text.to_owned()
}

fn literal_weight(template: &str) -> usize {
    let mut placeholder = false;
    template
        .chars()
        .filter(|c| match c {
            '{' => {
                placeholder = true;
                false
            }
            '}' => {
                placeholder = false;
                false
            }
            _ => !placeholder,
        })
        .count()
}

// Match a complete message, never replace words inside user text or filenames.
// Translated templates preserve the source placeholder order.
fn match_template<'a>(template: &str, text: &'a str) -> Option<Vec<&'a str>> {
    let mut remaining = text;
    let mut pattern = template;
    let mut values = Vec::new();
    while let Some(start) = pattern.find('{') {
        remaining = remaining.strip_prefix(&pattern[..start])?;
        let end = pattern[start..].find('}')? + start;
        pattern = &pattern[end + 1..];
        let literal_end = pattern.find('{').unwrap_or(pattern.len());
        let delimiter = &pattern[..literal_end];
        if delimiter.is_empty() {
            if !pattern.is_empty() {
                return None;
            }
            values.push(remaining);
            remaining = "";
        } else {
            let boundary = remaining.find(delimiter)?;
            values.push(&remaining[..boundary]);
            remaining = &remaining[boundary..];
        }
    }
    (remaining == pattern).then_some(values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_and_legacy_settings_are_english() {
        let mut value = serde_json::to_value(crate::domain::progress::Settings::default()).unwrap();
        value.as_object_mut().unwrap().remove("language");
        let settings: crate::domain::progress::Settings = serde_json::from_value(value).unwrap();
        assert_eq!(settings.language, Language::English);
    }

    #[test]
    fn translates_dynamic_messages_and_preserves_unknown_text() {
        assert_eq!(
            english("Siklus 2 dari 4 · Istirahat"),
            "Cycle 2 of 4 · Break"
        );
        assert_eq!(english("2 jam 15 menit"), "2 hours 15 minutes");
        assert_eq!(english("1 jam 1 menit"), "1 hour 1 minute");
        assert_eq!(english("1 sesi"), "1 session");
        assert_eq!(
            english("PNG tersimpan: C:\\Fokus\\catatan.png"),
            "PNG saved: C:\\Fokus\\catatan.png"
        );
        assert_eq!(english("my personal note"), "my personal note");
        assert_eq!(english("PNG tersimpan: Fokus"), "PNG saved: Fokus");
        assert_eq!(
            translate(Language::Indonesian, "Mulai fokus"),
            "Mulai fokus"
        );
    }

    #[test]
    fn catalog_is_unique_and_placeholders_match() {
        let mut keys = std::collections::HashSet::new();
        for &(source, translated) in catalog::MESSAGES {
            assert!(keys.insert(source), "duplicate message: {source}");
            assert_eq!(
                source.matches('{').count(),
                translated.matches('{').count(),
                "{source}"
            );
        }
    }
}
