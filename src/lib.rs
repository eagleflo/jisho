use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;

type Dictionary = HashMap<String, Vec<Entry>>;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Entry {
    pub kanji: String,
    pub reading: String,
    pub meanings: Vec<String>,
    pub frequency: i32,
}

lazy_static! {
    static ref J2E: Dictionary =
        serde_json::from_str(include_str!(concat!(env!("OUT_DIR"), "/j2e.json"))).unwrap();
    static ref E2J: Dictionary =
        serde_json::from_str(include_str!(concat!(env!("OUT_DIR"), "/e2j.json"))).unwrap();
    static ref READING: Dictionary =
        serde_json::from_str(include_str!(concat!(env!("OUT_DIR"), "/reading.json"))).unwrap();
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

fn collect_exact_results(dictionary: &'static Dictionary, input: &str) -> Vec<&'static Entry> {
    let mut results = Vec::new();
    if dictionary.contains_key(input) {
        let entries = dictionary.get(input).unwrap();
        results.extend(entries);
    }
    results.sort_by_key(|e| e.frequency);
    results
}

fn collect_prefix_results(dictionary: &'static Dictionary, input: &str) -> Vec<&'static Entry> {
    let mut results = Vec::new();
    for key in dictionary.keys() {
        if key.starts_with(input) {
            let entries = dictionary.get(key).unwrap();
            results.extend(entries);
        }
    }
    results.sort_by_key(|e| e.frequency);
    results
}

fn collect_postfix_results(dictionary: &'static Dictionary, input: &str) -> Vec<&'static Entry> {
    let mut results = Vec::new();
    for key in dictionary.keys() {
        if key.ends_with(input) {
            let entries = dictionary.get(key).unwrap();
            results.extend(entries);
        }
    }
    results.sort_by_key(|e| e.frequency);
    results
}

fn collect_results(dictionary: &'static Dictionary, input: &str) -> Vec<&'static Entry> {
    let mut results = collect_exact_results(dictionary, input);
    if results.is_empty() {
        results.extend(collect_prefix_results(dictionary, input));
    }
    if results.is_empty() {
        results.extend(collect_postfix_results(dictionary, input));
    }
    results
}

pub fn lookup(input: &str) -> Vec<&Entry> {
    if input.chars().any(|c| is_kanji(&c)) {
        collect_results(&J2E, input)
    } else if input.chars().all(|c| is_hiragana(&c) || is_katakana(&c)) {
        collect_results(&READING, input)
    } else {
        collect_prefix_results(&E2J, input)
    }
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
