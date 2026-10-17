use std::io::{self, Read};

fn main() {
    let mut text = String::new();
    io::stdin().read_to_string(&mut text).expect("failed to read stdin");

    let lines = text.lines().count();
    let words = text.split_whitespace().count();
    let chars = text.chars().count();

    println!("Lines:      {}", lines);
    println!("Words:      {}", words);
    println!("Characters: {}", chars);
}
