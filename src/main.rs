use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1].trim().to_string();
    jisho::lookup(input);
}
