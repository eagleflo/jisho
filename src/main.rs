use jisho::{lookup, Entry};
use std::env;
use std::io::{self, Write};

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
    if args.len() < 2 {
        loop {
            let mut input = String::new();
            print!("> ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            if input == "\n" {
                continue;
            }
            let results = lookup(input.trim());
            print_results(results);
        }
    } else {
        let input = &args[1..].join(" ");
        let results = lookup(input.trim());
        print_results(results);
    }
}
