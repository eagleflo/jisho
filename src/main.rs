use std::env;
use std::io::{self, Write};

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
            let results = jisho::lookup(input.trim());
            for entry in results.iter() {
                println!("{}【{}】- {}", entry.keb, entry.reb, entry.gloss);
            }
        }
    } else {
        let input = &args[1];
        let results = jisho::lookup(input.trim());
        for entry in results.iter() {
            println!("{}【{}】- {}", entry.keb, entry.reb, entry.gloss);
        }
    }
}
