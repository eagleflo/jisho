use lazy_static::lazy_static;
use rusqlite::{Connection, Result};
use serde::Deserialize;
use std::collections::HashMap;

type Dictionary = HashMap<String, Vec<Entry>>;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Entry {
    pub ent_seq: u32,
    pub kanji: String,
    pub reading: String,
    pub meanings: Vec<String>,
    pub frequency: i32,
}

const DB_FILE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/jisho.db"));
static DB: Connection = {
    let mut tmp_file = tempfile::tempfile();
    tmp_file.write_all(DB_FILE);
}

fn is_kanji(c: &char) -> bool {
    (*c >= '\u{4e00}' && *c <= '\u{9fff}') || // CJK Unified Ideographs
        (*c >= '\u{f900}' && *c <= '\u{faff}') // CJK Compatibility Ideographs
}

fn is_hiragana(c: &char) -> bool {
    *c >= '\u{3040}' && *c <= '\u{309f}'
}

fn is_katakana(c: &char) -> bool {
    *c >= '\u{30a0}' && *c <= '\u{30ff}'
}

fn collect_results(input: &str) -> Vec<&'static Entry> {
    let mut results = Vec::new();

    if dictionary.contains_key(input) {
        let entries = dictionary.get(input).unwrap();
        results.extend(entries);
    } else {
        for key in dictionary.keys() {
            if key.starts_with(input) {
                let entries = dictionary.get(key).unwrap();
                results.extend(entries);
            }
        }
    }
    results.sort_by_key(|e| e.frequency);
    results
}

pub fn lookup(input: &str) -> Vec<&Entry> {
    let mut results = Vec::new();

    let first = input.chars().next().unwrap();
    if is_kanji(&first) {
        let mut stmt = db.prepare(
            "SELECT kanji, reading, meanings, frequency FROM entries WHERE kanji = (?1)",
            &input,
        )?;
        // J2E, look up from kanji
    } else if is_hiragana(&first) || is_katakana(&first) {
        // READING, look up from reading
    } else {
        // E2J, look up from meanings
    }

    results.sort_by_key(|e| e.frequency);
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry() -> Entry {
        Entry {
            kanji: "緑".to_string(),
            reading: "みどり".to_string(),
            meanings: vec![
                "green".to_string(),
                "greenery".to_string(),
                "verdure".to_string(),
            ],
            frequency: 3,
        }
    }

    #[test]
    fn kanji_lookup() {
        let results = lookup("緑");
        assert_eq!(results.first().unwrap(), &&entry())
    }

    #[test]
    fn reading_lookup() {
        let results = lookup("みどり");
        assert_eq!(results.first().unwrap(), &&entry())
    }

    #[test]
    fn meaning_lookup() {
        let results = lookup("green");
        assert!(results.contains(&&entry()))
    }
}
