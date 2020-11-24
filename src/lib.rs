use lazy_static::lazy_static;
use std::collections::HashMap;

type Dictionary = HashMap<String, Vec<Entry>>;

#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub keb: String,
    pub reb: String,
    pub gloss: String,
}

fn upsert(dictionary: &mut Dictionary, key: String, entry: &Entry) {
    if let Some(entries) = dictionary.get_mut(&key) {
        entries.push(entry.clone());
    } else {
        dictionary.insert(key, vec![entry.clone()]);
    }
}

fn read_dictionary() -> (Dictionary, Dictionary, Dictionary) {
    let mut j2e = HashMap::new();
    let mut e2j = HashMap::new();
    let mut reading = HashMap::new();
    let xml = include_str!("../JMdict_e.xml");
    let doc = match roxmltree::Document::parse(&xml) {
        Ok(doc) => doc,
        Err(e) => {
            println!("Error: {}", e);
            return (j2e, e2j, reading);
        }
    };

    for node in doc.descendants().filter(|n| n.has_tag_name("entry")) {
        let keb = match node.descendants().find(|n| n.has_tag_name("keb")) {
            Some(e) => e.text().unwrap(),
            None => "",
        };
        let reb = match node.descendants().find(|n| n.has_tag_name("reb")) {
            Some(e) => e.text().unwrap(),
            None => continue,
        };
        let glosses = node
            .descendants()
            .filter(|n| n.has_tag_name("gloss"))
            .map(|n| n.text().unwrap());

        for gloss in glosses {
            let entry = Entry {
                keb: keb.to_string(),
                reb: reb.to_string(),
                gloss: gloss.to_string(),
            };

            if !keb.is_empty() {
                upsert(&mut j2e, keb.to_string(), &entry);
            }
            upsert(&mut e2j, gloss.to_string(), &entry);
            upsert(&mut reading, reb.to_string(), &entry);
        }
    }

    return (j2e, e2j, reading);
}

lazy_static! {
    static ref DICTIONARIES: (Dictionary, Dictionary, Dictionary) = read_dictionary();
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

fn collect_results(dictionary: &'static Dictionary, input: &str) -> Vec<&'static Entry> {
    let mut results = Vec::new();
    if dictionary.contains_key(input) {
        let entries = dictionary.get(input).unwrap();
        results.extend(entries);
    } else {
        for key in dictionary.keys() {
            if key.starts_with(input) {
                let entries = dictionary.get(input).unwrap();
                results.extend(entries);
            }
        }
    }
    return results;
}

pub fn lookup(input: &str) -> Vec<&Entry> {
    let j2e = &DICTIONARIES.0;
    let e2j = &DICTIONARIES.1;
    let reading = &DICTIONARIES.2;
    let first = input.chars().next().unwrap();
    return if is_kanji(&first) {
        collect_results(j2e, input)
    } else if is_hiragana(&first) || is_katakana(&first) {
        collect_results(reading, input)
    } else {
        collect_results(e2j, input)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kanji_lookup() {
        let results = lookup("緑");
        let entry = Entry {
            keb: "緑".to_string(),
            reb: "みどり".to_string(),
            gloss: "green".to_string(),
        };
        assert_eq!(results.first().unwrap(), &&entry)
    }

    #[test]
    fn reading_lookup() {
        let results = lookup("みどり");
        let entry = Entry {
            keb: "緑".to_string(),
            reb: "みどり".to_string(),
            gloss: "green".to_string(),
        };
        assert_eq!(results.first().unwrap(), &&entry)
    }

    #[test]
    fn meaning_lookup() {
        let results = lookup("green");
        let entry = Entry {
            keb: "緑".to_string(),
            reb: "みどり".to_string(),
            gloss: "green".to_string(),
        };
        assert!(results.contains(&&entry))
    }
}
