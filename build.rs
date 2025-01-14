use bitcode::Encode;
use flate2::read::GzDecoder;
use std::{collections::HashMap, env, fs, io::Read, path::Path};

type Dictionary = HashMap<String, Vec<Entry>>;

#[derive(Clone, PartialEq, Encode)]
pub struct Entry {
    pub kanji: String,
    pub reading: String,
    pub meanings: Vec<String>,
    pub frequency: i32,
}

fn upsert(dictionary: &mut Dictionary, key: String, entry: &Entry) {
    if let Some(entries) = dictionary.get_mut(&key) {
        if !entries.contains(entry) {
            entries.push(entry.clone());
        }
    } else {
        dictionary.insert(key, vec![entry.clone()]);
    }
}

// JMdict often adds parenthetical explanations to its English meanings. We need
// to cut them down to bare headwords.
// FIXME: There are a number of meanings in JMdict that *begin* with parentheses.
// These get lost via this process.
fn trim_explanation(meaning: &str) -> &str {
    if meaning.ends_with(')') {
        if let Some(open_parenthesis) = meaning.find('(') {
            return meaning[..open_parenthesis].trim();
        }
    }
    meaning
}

fn read_dictionary() -> (Dictionary, Dictionary, Dictionary, String) {
    let mut j2e = HashMap::new();
    let mut e2j = HashMap::new();
    let mut reading = HashMap::new();
    let mut version = String::from("unknown");
    let mut gz = GzDecoder::new(fs::File::open("./JMdict_e.gz").unwrap());
    let mut xml = String::new();
    gz.read_to_string(&mut xml).unwrap();
    let opt = roxmltree::ParsingOptions {
        allow_dtd: true,
        ..roxmltree::ParsingOptions::default()
    };
    let doc = match roxmltree::Document::parse_with_options(&xml, opt) {
        Ok(doc) => doc,
        Err(e) => {
            println!("Error: {}", e);
            return (j2e, e2j, reading, version);
        }
    };

    if let Some(comment_node) = doc
        .descendants()
        .find(|n| n.is_comment() && n.text().unwrap().starts_with(" JMdict created: "))
    {
        // Version comment has the format "JMdict created: 2024-07-15"
        let comment = comment_node.text().unwrap();
        if let Some((_, version_str)) = comment.split_once(": ") {
            version = version_str.trim().to_string();
        }
    }

    for node in doc.descendants().filter(|n| n.has_tag_name("entry")) {
        let keb = match node.descendants().find(|n| n.has_tag_name("keb")) {
            Some(e) => e.text().unwrap(),
            None => "",
        };
        let reb = match node.descendants().find(|n| n.has_tag_name("reb")) {
            Some(e) => e.text().unwrap(),
            None => continue,
        };
        let nf = match node
            .descendants()
            .find(|n| n.has_tag_name("re_pri") && n.text().unwrap().starts_with("nf"))
        {
            Some(e) => e.text().unwrap(),
            None => "",
        };

        let glosses = node
            .descendants()
            .filter(|n| n.has_tag_name("gloss"))
            .map(|n| n.text().unwrap().to_string())
            .collect();

        let entry = Entry {
            kanji: keb.to_string(),
            reading: reb.to_string(),
            meanings: glosses,
            frequency: if !nf.is_empty() {
                nf[2..].parse().unwrap_or(999)
            } else {
                999
            },
        };

        if !keb.is_empty() {
            upsert(&mut j2e, keb.to_string(), &entry);
        }
        for meaning in &entry.meanings {
            let headword = trim_explanation(meaning).to_lowercase();
            upsert(&mut e2j, headword, &entry);
        }
        upsert(&mut reading, reb.to_string(), &entry);
    }

    (j2e, e2j, reading, version)
}

fn main() {
    let (j2e, e2j, reading, version) = read_dictionary();

    let out_dir = env::var_os("OUT_DIR").unwrap();

    let j2e_path = Path::new(&out_dir).join("j2e.bitcode");
    let j2e_bitcode = bitcode::encode(&j2e);
    fs::write(j2e_path, j2e_bitcode).unwrap();

    let e2j_path = Path::new(&out_dir).join("e2j.bitcode");
    let e2j_bitcode = bitcode::encode(&e2j);
    fs::write(e2j_path, e2j_bitcode).unwrap();

    let reading_path = Path::new(&out_dir).join("reading.bitcode");
    let reading_bitcode = bitcode::encode(&reading);
    fs::write(reading_path, reading_bitcode).unwrap();

    let version_path = Path::new(&out_dir).join("jmdict_version");
    fs::write(version_path, version).unwrap();

    println!("cargo:rerun-if-changed=JMdict_e.gz");
}
