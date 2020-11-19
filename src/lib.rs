use std::collections::HashMap;

fn read_dictionary() -> (
    HashMap<String, (String, String)>,
    HashMap<String, (String, String)>,
) {
    let mut j2e = HashMap::new();
    let mut e2j = HashMap::new();
    let xml = include_str!("../JMdict_e.xml");
    let doc = match roxmltree::Document::parse(&xml) {
        Ok(doc) => doc,
        Err(e) => {
            println!("Error: {}", e);
            return (j2e, e2j);
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
        }
    }

    return (j2e, e2j);
}

fn is_japanese(c: &char) -> bool {
    (*c >= '\u{4e00}' && *c <= '\u{9fff}') || // CJK Unified Ideographs
        (*c >= '\u{3040}' && *c <= '\u{309f}') || // Hiragana
        (*c >= '\u{30a0}' && *c <= '\u{30ff}') || // Katakana
        (*c >= '\u{f900}' && *c <= '\u{faff}') // CJK Compatibility Ideographs
}

pub fn lookup(input: &String) {
    // FIXME: Use lazy_static macro?
    let (j2e, e2j) = read_dictionary();
    let first = input.chars().next().unwrap();

    if is_japanese(&first) {
        if j2e.contains_key(input) {
            let (reb, gloss) = j2e.get(input).unwrap();
            println!("{}【{}】- {}", input, reb, gloss);
        } else {
            for key in j2e.keys() {
                if key.starts_with(input) {
                    let (reb, gloss) = j2e.get(key).unwrap();
                    println!("{}【{}】- {}", input, reb, gloss);
                }
            }
        }
    } else {
        if e2j.contains_key(input) {
            let (keb, reb) = e2j.get(input).unwrap();
            println!("{}【{}】- {}", keb, reb, input);
        } else {
            for key in e2j.keys() {
                if key.starts_with(input) {
                    let (keb, reb) = e2j.get(key).unwrap();
                    println!("{}【{}】- {}", keb, reb, key);
                }
            }
        }
    }
}
