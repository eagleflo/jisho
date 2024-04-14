use std::env;

use rustyline::DefaultEditor;

use jisho::{lookup, Entry};

fn print_results(results: Vec<&Entry>) {
    for entry in results.iter() {
        if entry.kanji.is_empty() {
            println!("{} - {}", entry.reading, entry.meanings.join(", "));
        } else {
            println!(
                "{}【{}】- {}",
                entry.kanji,
                entry.reading,
                entry.meanings.join(", ")
            );
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"-v".to_string()) || args.contains(&"--version".to_string()) {
        let name = env!("CARGO_PKG_NAME");
        let version = env!("CARGO_PKG_VERSION");
        println!("{} {}", name, version);
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
