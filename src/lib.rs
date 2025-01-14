use bitcode::Decode;
use flate2::read::ZlibDecoder;
use lazy_static::lazy_static;
use rustc_hash::FxHashMap;
use std::io::Read;

type Dictionary = FxHashMap<String, Vec<Entry>>;

#[derive(Clone, Debug, PartialEq, Decode)]
pub struct Entry {
    pub kanji: String,
    pub reading: String,
    pub meanings: Vec<String>,
    pub frequency: i32,
}

lazy_static! {
    static ref J2E: Dictionary = {
        let b = include_bytes!(concat!(env!("OUT_DIR"), "/j2e.bitcode.zlib"));
        let mut d = ZlibDecoder::new(b.as_slice());
        let mut v = Vec::new();
        d.read_to_end(&mut v).unwrap();
        bitcode::decode(&v).unwrap()
    };
    static ref E2J: Dictionary = {
        let b = include_bytes!(concat!(env!("OUT_DIR"), "/e2j.bitcode.zlib"));
        let mut d = ZlibDecoder::new(b.as_slice());
        let mut v = Vec::new();
        d.read_to_end(&mut v).unwrap();
        bitcode::decode(&v).unwrap()
    };
    static ref READING: Dictionary = {
        let b = include_bytes!(concat!(env!("OUT_DIR"), "/reading.bitcode.zlib"));
        let mut d = ZlibDecoder::new(b.as_slice());
        let mut v = Vec::new();
        d.read_to_end(&mut v).unwrap();
        bitcode::decode(&v).unwrap()
    };
}

fn strip_first(input: &str) -> &str {
    let mut chars = input.chars();
    chars.next();
    chars.as_str()
}

fn strip_last(input: &str) -> &str {
    let mut chars = input.chars();
    chars.next_back();
    chars.as_str()
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
            for entry in entries {
                if !results.contains(&entry) {
                    results.push(entry);
                }
            }
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
            for entry in entries {
                if !results.contains(&entry) {
                    results.push(entry);
                }
            }
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

enum Mode {
    Default,
    Exact,
    Prefix,
    Postfix,
}

pub fn lookup(input_raw: &str) -> Vec<&Entry> {
    let mut mode: Mode = Mode::Default;
    let mut input = input_raw;
    if input_raw.starts_with('=') {
        mode = Mode::Exact;
        input = strip_first(input_raw);
    } else if input_raw.ends_with(['*', '＊']) {
        mode = Mode::Prefix;
        input = strip_last(input_raw);
    } else if input_raw.starts_with(['*', '＊']) {
        mode = Mode::Postfix;
        input = strip_first(input_raw);
    }

    if input.chars().any(|c| is_kanji(&c)) {
        match mode {
            Mode::Default => collect_results(&J2E, input),
            Mode::Exact => collect_exact_results(&J2E, input),
            Mode::Prefix => collect_prefix_results(&J2E, input),
            Mode::Postfix => collect_postfix_results(&J2E, input),
        }
    } else if input.chars().all(|c| is_hiragana(&c) || is_katakana(&c)) {
        match mode {
            Mode::Default => collect_results(&READING, input),
            Mode::Exact => collect_exact_results(&READING, input),
            Mode::Prefix => collect_prefix_results(&READING, input),
            Mode::Postfix => collect_postfix_results(&READING, input),
        }
    } else {
        match mode {
            Mode::Default => collect_results(&E2J, input),
            Mode::Exact => collect_exact_results(&E2J, input),
            Mode::Prefix => collect_prefix_results(&E2J, input),
            Mode::Postfix => collect_postfix_results(&E2J, input),
        }
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
        let mut results = lookup("green");
        assert!(results.contains(&&entry()));

        let results_len = results.len();
        results.dedup();
        assert!(results_len == results.len())
    }
}
