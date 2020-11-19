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
            jisho::lookup(input.trim());
        }
    } else {
        let input = &args[1];
        jisho::lookup(input.trim());
    }
}
