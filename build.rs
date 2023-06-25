use rusqlite::{Connection, Result};
use std::{env, fs, path::Path};

#[derive(Clone)]
pub struct Entry {
    pub ent_seq: u32,
    pub kanji: String,
    pub reading: String,
    pub meanings: Vec<String>,
    pub frequency: i32,
}

fn read_dictionary() -> Vec<Entry> {
    let mut entries = Vec::new();
    let xml = fs::read_to_string("./JMdict_e.xml").unwrap();
    let opt = roxmltree::ParsingOptions {
        allow_dtd: true,
        ..roxmltree::ParsingOptions::default()
    };
    let doc = match roxmltree::Document::parse_with_options(&xml, opt) {
        Ok(doc) => doc,
        Err(e) => {
            println!("Error: {}", e);
            return entries;
        }
    };

    for node in doc.descendants().filter(|n| n.has_tag_name("entry")) {
        let ent_seq = match node.descendants().find(|n| n.has_tag_name("ent_seq")) {
            Some(e) => e.text().unwrap(),
            None => continue,
        };
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
            ent_seq: ent_seq.parse().unwrap(),
            kanji: keb.to_string(),
            reading: reb.to_string(),
            meanings: glosses,
            frequency: if !nf.is_empty() {
                nf[2..].parse().unwrap_or(999)
            } else {
                999
            },
        };

        entries.push(entry)
    }

    entries
}

fn write_database(entries: Vec<Entry>) -> Result<()> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let db_path = Path::new(&out_dir).join("jisho.db");
    let db = Connection::open(db_path.as_path())?;
    db.execute("DROP TABLE IF EXISTS entries", ())?;
    db.execute(
        "CREATE TABLE entries (
            ent_seq INTEGER PRIMARY KEY,
            kanji TEXT NOT NULL,
            reading TEXT NOT NULL,
            meanings TEXT NOT NULL,
            frequency INTEGER NOT NULL
        )",
        (),
    )?;

    db.execute("BEGIN", ())?;
    for entry in entries {
        db.execute(
            "INSERT INTO entries (ent_seq, kanji, reading, meanings, frequency) VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                &entry.ent_seq,
                &entry.kanji,
                &entry.reading,
                &entry.meanings.join(", "),
                &entry.frequency,
            ),
        )?;
    }
    db.execute("END", ())?;

    Ok(())
}

fn main() {
    let entries = read_dictionary();
    write_database(entries);
    println!("cargo:rerun-if-changed=JMdict_e.xml");
}
