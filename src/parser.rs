use std::iter::Peekable;
use std::collections::HashMap;
use super::lexer::*;
use super::token::*;

pub struct Parser<I: Iterator<Item=char>> {
    lexer: Peekable<Lexer<I>>,
    sym: Token,
    types: HashMap<Token, &'static str>,
}

impl<I: Iterator<Item=char>> Parser<I> {
    pub fn new(lexer: Lexer<I>) -> Parser<I> {
        let mut l = lexer.peekable();
        let t = l.next().unwrap();

        let mut ty = HashMap::new();

        ty.insert(Token::U8, "let u8");
        ty.insert(Token::U16, "let u16");
        ty.insert(Token::U32, "let u32");
        ty.insert(Token::U64, "let u64");
        ty.insert(Token::Usize, "let usize");
        ty.insert(Token::I8, "let i8");
        ty.insert(Token::I16, "let i16");
        ty.insert(Token::I32, "let i32");
        ty.insert(Token::I64, "let i64");
        ty.insert(Token::Usize, "let isize");

        Parser { lexer: l, sym: t, types: ty }
    }

    /*fn accept(&mut self, t: Token) -> bool {
        if *self.lexer.peek().unwrap() == t {
            self.lexer.next();
            true
        } else {
            false
        }
    }*/

    pub fn parse(&mut self) {
        self.translation_unit()
    }

    fn translation_unit(&mut self) {
        self.global_declaration()
    }

    fn global_declaration(&mut self) {
        self.variable_declaration()
    }

    fn variable_declaration(&mut self) {
        self.type_specifier();
    }

    fn type_specifier(&mut self) {
        match self.types.get(&self.sym) {
            Some(s) => print!("{} ", s),
            None => panic!("Expected type; got {:?}", self.sym),
        };
        self.sym = self.lexer.next().unwrap();
        match self.sym {
            Token::Ident(ref s) => print!("{} ", s),
            _ => panic!("Expected ident; got {:?}", self.sym),
        };
        self.sym = self.lexer.next().unwrap();
        match self.sym {
            Token::Term('=') => print!("equal "),
            _ => panic!("Expected '='; got {:?}", self.sym),
        };
        self.sym = self.lexer.next().unwrap();
        match self.sym {
            Token::Binary{ref v, ..} |
            Token::Octal{ref v, ..} |
            Token::Decimal{ref v, ..} |
            Token::Hexadecimal{ref v, ..} |
            Token::Float{ref v, ..} => println!("{}", v),
            _ => panic!("Expected constant; got {:?}", self.sym),
        };
        self.sym = self.lexer.next().unwrap();
        match self.sym {
            Token::Term(';') => { },
            _ => panic!("Expected ';'; got {:?}", self.sym),
        };
    }
}
