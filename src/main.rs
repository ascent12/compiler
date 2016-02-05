use std::io::prelude::*;
use std::fs::File;

mod token;
mod lexer;
use self::lexer::*;

fn main() {
    let mut f = File::open("test.G").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    for t in Lexer::new(s.chars()) {
        println!("{:?}", t);
    }
}
