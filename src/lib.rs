use lazy_static::lazy_static;
use std::collections::HashMap;

type Dictionary = HashMap<String, (String, String)>;

#[derive(PartialEq, Debug)]
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
            let gloss = match node.descendants().find(|n| n.has_tag_name("gloss")) {
                Some(e) => e.text().unwrap(),
                None => continue,
            };
            j2e.insert(keb.to_string(), (reb.to_string(), gloss.to_string()));
            e2j.insert(gloss.to_string(), (keb.to_string(), reb.to_string()));
            reading.insert(reb.to_string(), (keb.to_string(), gloss.to_string()));
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

pub fn lookup(input: &str) -> Vec<Entry> {
    let j2e = &DICTIONARIES.0;
    let e2j = &DICTIONARIES.1;
    let reading = &DICTIONARIES.2;
    let first = input.chars().next().unwrap();
    let mut results = Vec::new();

    if is_kanji(&first) {
        if j2e.contains_key(input) {
            let (reb, gloss) = j2e.get(input).unwrap();
            let entry = Entry {
                keb: input.to_string(),
                reb: reb.clone(),
                gloss: gloss.clone(),
            };
            results.push(entry);
        } else {
            for key in j2e.keys() {
                if key.starts_with(input) {
                    let (reb, gloss) = j2e.get(key).unwrap();
                    let entry = Entry {
                        keb: key.to_string(),
                        reb: reb.clone(),
                        gloss: gloss.clone(),
                    };
                    results.push(entry);
                }
            }
        }
    } else if is_hiragana(&first) || is_katakana(&first) {
        if reading.contains_key(input) {
            let (keb, gloss) = reading.get(input).unwrap();
            let entry = Entry {
                keb: keb.clone(),
                reb: input.to_string(),
                gloss: gloss.clone(),
            };
            results.push(entry);
        } else {
            for key in reading.keys() {
                if key.starts_with(input) {
                    let (keb, gloss) = j2e.get(key).unwrap();
                    let entry = Entry {
                        keb: keb.clone(),
                        reb: key.to_string(),
                        gloss: gloss.clone(),
                    };
                    results.push(entry);
                }
            }
        }
    } else {
        if e2j.contains_key(input) {
            let (keb, reb) = e2j.get(input).unwrap();
            let entry = Entry {
                keb: keb.clone(),
                reb: reb.clone(),
                gloss: input.to_string(),
            };
            results.push(entry);
        } else {
            for key in e2j.keys() {
                if key.starts_with(input) {
                    let (keb, reb) = e2j.get(key).unwrap();
                    let entry = Entry {
                        keb: keb.clone(),
                        reb: reb.clone(),
                        gloss: key.to_string(),
                    };
                    results.push(entry);
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
        assert_eq!(
            results.first().unwrap(),
            &Entry {
                keb: "緑".to_string(),
                reb: "みどり".to_string(),
                gloss: "green".to_string()
            }
        )
    }

    #[test]
    fn reading_lookup() {
        let results = lookup("みどり");
        assert_eq!(
            results.first().unwrap(),
            &Entry {
                keb: "緑".to_string(),
                reb: "みどり".to_string(),
                gloss: "green".to_string()
            }
        )
    }

    #[test]
    fn meaning_lookup() {
        let results = lookup("green");
        assert_eq!(
            results.first().unwrap(),
            &Entry {
                keb: "緑".to_string(),
                reb: "みどり".to_string(),
                gloss: "green".to_string()
            }
        )
    }
}
