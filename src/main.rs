use std::io::prelude::*;
use std::fs::File;

mod token;
mod lexer;
mod parser;
use self::lexer::*;
use self::parser::*;

fn main() {
    let mut f = File::open("test.G").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let mut p = Parser::new(Lexer::new(s.chars()));
    p.parse();
}
