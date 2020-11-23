use lazy_static::lazy_static;
use std::collections::HashMap;

type Dictionary = HashMap<String, Vec<Entry>>;

#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub keb: String,
    pub reb: String,
    pub gloss: String,
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

    for node in doc.descendants() {
        if node.has_tag_name("entry") {
            let keb = match node.descendants().find(|n| n.has_tag_name("keb")) {
                Some(e) => e.text().unwrap(),
                None => continue,
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

                if let Some(entries) = j2e.get_mut(&keb.to_string()) {
                    entries.push(entry.clone());
                } else {
                    j2e.insert(keb.to_string(), vec![entry.clone()]);
                }
                if let Some(entries) = e2j.get_mut(&gloss.to_string()) {
                    entries.push(entry.clone());
                } else {
                    e2j.insert(gloss.to_string(), vec![entry.clone()]);
                }
                if let Some(entries) = reading.get_mut(&reb.to_string()) {
                    entries.push(entry.clone());
                } else {
                    reading.insert(reb.to_string(), vec![entry.clone()]);
                }
            }
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

pub fn lookup(input: &str) -> Vec<&Entry> {
    let j2e = &DICTIONARIES.0;
    let e2j = &DICTIONARIES.1;
    let reading = &DICTIONARIES.2;
    let first = input.chars().next().unwrap();
    let mut results = Vec::new();

    if is_kanji(&first) {
        if j2e.contains_key(input) {
            let entries = j2e.get(input).unwrap();
            results.extend(entries);
        } else {
            for key in j2e.keys() {
                if key.starts_with(input) {
                    let entries = j2e.get(key).unwrap();
                    results.extend(entries);
                }
            }
        }
    } else if is_hiragana(&first) || is_katakana(&first) {
        if reading.contains_key(input) {
            let entries = reading.get(input).unwrap();
            results.extend(entries);
        } else {
            for key in reading.keys() {
                if key.starts_with(input) {
                    let entries = j2e.get(key).unwrap();
                    results.extend(entries);
                }
            }
        }
    } else {
        if e2j.contains_key(input) {
            let entries = e2j.get(input).unwrap();
            results.extend(entries);
        } else {
            for key in e2j.keys() {
                if key.starts_with(input) {
                    let entries = e2j.get(key).unwrap();
                    results.extend(entries);
                }
            }
        }
    }

    return results;
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
