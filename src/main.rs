use std::env;

use rustyline::DefaultEditor;

use jisho::{Entry, lookup};

fn print_results(results: Vec<&Entry>) {
    for entry in results.iter() {
        if entry.kanji.is_empty() {
            println!(
                "{} - {}",
                entry.reading,
                entry
                    .meanings
                    .iter()
                    .map(|s| s.glosses.join(", "))
                    .collect::<Vec<_>>()
                    .join("; ")
            );
        } else {
            println!(
                "{}【{}】- {}",
                entry.kanji,
                entry.reading,
                entry
                    .meanings
                    .iter()
                    .map(|s| s.glosses.join(", "))
                    .collect::<Vec<_>>()
                    .join("; ")
            );
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"-v".to_string()) || args.contains(&"--version".to_string()) {
        let name = env!("CARGO_PKG_NAME");
        let version = env!("CARGO_PKG_VERSION");
        let jmdict_version = include_str!(concat!(env!("OUT_DIR"), "/jmdict_version"));
        println!("{} {} (JMdict {})", name, version, jmdict_version);
        std::process::exit(0);
    }
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        println!("Usage: jisho [OPTIONS] [QUERY]");
        println!();
        println!("A bidirectional Japanese-English dictionary.");
        println!("Without a query, starts an interactive REPL.");
        println!();
        println!("Options:");
        println!("  -h, --help     Show this help");
        println!("  -v, --version  Show version");
        println!();
        println!("Search modifiers:");
        println!("  word*    Prefix match (entries starting with word)");
        println!("  *word    Postfix match (entries ending with word)");
        println!("  w?rd     Wildcard match (? matches any single character)");
        println!("  =word    Exact match only");
        std::process::exit(0);
    }
    if args.len() < 2 {
        let mut rl = DefaultEditor::new().unwrap();
        loop {
            let readline = rl.readline("> ");
            match readline {
                Ok(line) => {
                    if line.is_empty() {
                        continue;
                    }
                    let _ = rl.add_history_entry(line.as_str());
                    let input = line.trim().to_lowercase();
                    let results = lookup(&input);
                    print_results(results);
                }
                Err(_) => break,
            }
        }
    } else {
        let input = &args[1..].join(" ").trim().to_lowercase();
        let results = lookup(input);
        print_results(results);
    }
}
