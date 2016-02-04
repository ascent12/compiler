use std::io::prelude::*;
use std::fs::File;
//use std::str::Chars;

mod lex;
use self::lex::*;

fn main() {
    let mut f = File::open("test.G").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    for t in Lex::new(s.chars()) {
        println!("{}", t);
    }
}
