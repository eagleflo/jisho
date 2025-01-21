use bitcode::Encode;
use flate2::read::GzDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use rustc_hash::FxHashMap;
use std::{
    env,
    error::Error,
    fs,
    io::{Read, Write},
    path::Path,
};

type Dictionary = FxHashMap<String, Vec<Entry>>;

#[derive(Clone, PartialEq, Encode)]
pub struct Entry {
    pub kanji: String,
    pub reading: String,
    pub meanings: Vec<Sense>,
    pub frequency: i32,
}

#[derive(Clone, PartialEq, Encode)]
pub struct Sense {
    pub glosses: Vec<String>,
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

// JMdict often adds parenthetical explanations to its English glosses. We need
// to cut them down to bare headwords.
// FIXME: There are a number of meanings in JMdict that *begin* with parentheses.
// These get lost via this process.
fn trim_explanation(gloss: &str) -> &str {
    if gloss.ends_with(')') {
        if let Some(open_parenthesis) = gloss.find('(') {
            return gloss[..open_parenthesis].trim();
        }
    }
    gloss
}

fn read_dictionary() -> (Dictionary, Dictionary, Dictionary, String) {
    let mut j2e = FxHashMap::default();
    let mut e2j = FxHashMap::default();
    let mut reading = FxHashMap::default();
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
        let kebs: Vec<_> = node
            .descendants()
            .filter(|n| n.has_tag_name("keb"))
            .map(|n| n.text().unwrap().to_string())
            .collect();

        let rebs: Vec<_> = node
            .descendants()
            .filter(|n| n.has_tag_name("reb"))
            .map(|n| n.text().unwrap().to_string())
            .collect();

        let nf = match node
            .descendants()
            .find(|n| n.has_tag_name("re_pri") && n.text().unwrap().starts_with("nf"))
        {
            Some(e) => e.text().unwrap(),
            None => "",
        };

        let senses = node
            .descendants()
            .filter(|n| n.has_tag_name("sense"))
            .map(|n| Sense {
                glosses: n
                    .descendants()
                    .filter(|n| n.has_tag_name("gloss"))
                    .map(|n| n.text().unwrap().to_string())
                    .collect(),
            })
            .collect();

        let entry = Entry {
            kanji: kebs.first().map_or("", |v| v).to_string(),
            reading: rebs.first().map_or("", |v| v).to_string(),
            meanings: senses,
            frequency: if !nf.is_empty() {
                nf[2..].parse().unwrap_or(999)
            } else {
                999
            },
        };

        for keb in kebs {
            upsert(&mut j2e, keb, &entry);
        }

        for meaning in &entry.meanings {
            for gloss in &meaning.glosses {
                let headword = trim_explanation(gloss).to_lowercase();
                upsert(&mut e2j, headword, &entry);
            }
        }

        for reb in rebs {
            upsert(&mut reading, reb, &entry);
        }
    }

    (j2e, e2j, reading, version)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (j2e, e2j, reading, version) = read_dictionary();

    let out_dir = env::var_os("OUT_DIR").unwrap();

    let j2e_path = Path::new(&out_dir).join("j2e");
    let j2e_bitcode = bitcode::encode(&j2e);
    let j2e_bytes: Vec<u8>;
    if cfg!(feature = "compression") {
        let mut j2e_compressed = ZlibEncoder::new(Vec::new(), Compression::best());
        j2e_compressed.write_all(&j2e_bitcode)?;
        j2e_bytes = j2e_compressed.finish()?;
    } else {
        j2e_bytes = j2e_bitcode;
    }
    fs::write(j2e_path, j2e_bytes).unwrap();

    let e2j_path = Path::new(&out_dir).join("e2j");
    let e2j_bitcode = bitcode::encode(&e2j);
    let e2j_bytes: Vec<u8>;
    if cfg!(feature = "compression") {
        let mut e2j_compressed = ZlibEncoder::new(Vec::new(), Compression::best());
        e2j_compressed.write_all(&e2j_bitcode)?;
        e2j_bytes = e2j_compressed.finish()?;
    } else {
        e2j_bytes = e2j_bitcode;
    }
    fs::write(e2j_path, e2j_bytes).unwrap();

    let reading_path = Path::new(&out_dir).join("reading");
    let reading_bitcode = bitcode::encode(&reading);
    let reading_bytes: Vec<u8>;
    if cfg!(feature = "compression") {
        let mut reading_compressed = ZlibEncoder::new(Vec::new(), Compression::best());
        reading_compressed.write_all(&reading_bitcode)?;
        reading_bytes = reading_compressed.finish()?;
    } else {
        reading_bytes = reading_bitcode;
    }
    fs::write(reading_path, reading_bytes).unwrap();

    let version_path = Path::new(&out_dir).join("jmdict_version");
    fs::write(version_path, version).unwrap();

    println!("cargo:rerun-if-changed=JMdict_e.gz");
    Ok(())
}
