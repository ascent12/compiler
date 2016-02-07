use std::char;
use std::collections::HashMap;
use super::token::*;
use super::token::Token::*;

struct Save<I>
where I: Iterator<Item=char> {
    iter: I,
    curr: Option<char>,
    save: bool,
}

impl<I> Save<I>
where I: Iterator<Item=char> {
    fn new(iter: I) -> Save<I> {
        Save { iter: iter, curr: None, save: false }
    }

    fn save(&mut self) {
        self.save = true
    }
}

impl<I> Iterator for Save<I>
where I: Iterator<Item=char> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.save {
            self.save = false;
        } else {
            self.curr = self.iter.next();
        }

        self.curr
    }
}

pub struct Lexer<I>
where I: Iterator<Item=char> {
    iter: Save<I>,
    eof: Option<Token>,

    keywords: HashMap<String, Token>,
}

impl<I: Iterator<Item=char>> Lexer<I> {
    pub fn new(iter: I) -> Lexer<I> {
        let keywords = hash![
            "if"    => If,
            "else"  => Else,
            "do"    => Do,
            "while" => While,
            "for"   => For,

            "u8"    => U8,
            "u16"   => U16,
            "u32"   => U32,
            "u64"   => U64,
            "usize" => Usize,
            "i8"    => I8,
            "i16"   => I16,
            "i32"   => I32,
            "i64"   => I64,
            "isize" => Isize,
        ];

        Lexer {
            iter: Save::new(iter),
            eof: Some(EndOfFile),
            keywords: keywords,
        }
    }

    fn skip_whitespace(&mut self) {
        for c in self.iter.by_ref() {
            match c {
                ' '|'\t'|'\r' => { },
                '\n' => { },
                _ => break,
            };
        }

        self.iter.save();
    }

    fn lex_ident(&mut self, c: char) -> Option<Token> {
        let mut val = String::with_capacity(20);
        val.push(c);

        for c in self.iter.by_ref() {
            match c {
                '_' |
                '0'...'9' |
                'a'...'z' |
                'A'...'Z' => val.push(c),
                _ => break,
            };
        }

        self.iter.save();

        Some(match self.keywords.get(&val).cloned() {
            Some(t) => t,
            None => Ident(val),
        })
    }

    fn lex_constant(&mut self, t: char) -> Option<Token> {
        #[derive(PartialEq)]
        enum State {
            Int,
            Trail,
        }

        let mut val = String::with_capacity(20);
        let mut trail = String::with_capacity(5);

        let mut state = State::Int;

        for c in self.iter.by_ref() {
            state = match state {
                State::Int => match c {
                    '0'...'1' if t == 'b' => State::Int,
                    '0'...'7' if t == 'o' => State::Int,
                    '0'...'9' |
                    'a'...'f' |
                    'A'...'F' if t == 'x' => State::Int,
                    '_' |
                    'a'...'z' |
                    'A'...'Z' => State::Trail,
                    _         => break,
                },
                State::Trail => match c {
                    '_' |
                    '0'...'9' |
                    'a'...'z' |
                    'A'...'Z' => State::Trail,
                    _         => break,
                },
            };

            match state {
                State::Int => val.push(c),
                State::Trail => trail.push(c),
            };
        }

        self.iter.save();

        Some(match t {
            'b' => Token::Binary { v: val, t: trail },
            'o' => Token::Octal { v: val, t: trail },
            'x' => Token::Hexadecimal { v: val, t: trail },
            _ => unreachable!(),
        })
    }

    fn lex_decimal(&mut self, c: char) -> Option<Token> {
        enum State {
            Int,
            Dec,
            ExpSign,
            Exp,
            IntTrail,
            FloatTrail,
        }

        let mut val = String::with_capacity(20);
        let mut trail = String::with_capacity(5);
        val.push(c);

        let mut state = State::Int;

        for c in self.iter.by_ref() {
            state = match state {
                State::Int => match c {
                    '0'...'9' => State::Int,
                    '.'       => State::Dec,
                    'e'|'E'   => State::ExpSign,
                    'a'...'z' |
                    'A'...'Z' => State::IntTrail,
                    _         => break,
                },
                State::Dec => match c {
                    '0'...'9' => State::Dec,
                    'e'|'E'   => State::ExpSign,
                    'a'...'z' |
                    'A'...'Z' => State::FloatTrail,
                    _         => break,
                },
                State::ExpSign => match c {
                    '+'|'-' |
                    '0'...'9' => State::Exp,
                    _         => {
                        println!("Warning: empty exponent");
                        val.pop();
                        break;
                    },
                },
                State::Exp => match c {
                    '0'...'9' => State::Exp,
                    'a'...'z' |
                    'A'...'Z' => State::FloatTrail,
                    _         => break,
                },
                State::IntTrail => match c {
                    '0'...'9' |
                    'a'...'z' |
                    'A'...'Z' => State::IntTrail,
                    _         => break,
                },
                State::FloatTrail => match c {
                    '0'...'9' |
                    'a'...'z' |
                    'A'...'Z' => State::FloatTrail,
                    _         => break,
                },
            };

            match state {
                State::IntTrail |
                State::FloatTrail => trail.push(c),
                _ => val.push(c),
            }
        }

        self.iter.save();

        Some(match state {
            State::Int |
            State::IntTrail => Token::Decimal { v: val, t: trail },
            _               => Token::Float{ v: val, t: trail },
        })
    }

    fn lex_str_literal(&mut self) -> Option<Token> {
        #[derive(PartialEq)]
        enum State {
            Str,
            Escape,
            Hex,
        }

        let mut val = String::with_capacity(32);
        let mut state = State::Str;

        let mut tmp: u32 = 0;
        let mut rem: u32 = 0;

        for mut c in self.iter.by_ref() {
            state = match state {
                State::Str => match c {
                    '\\' => State::Escape,
                    '"' => break,
                    _ => State::Str,
                },
                State::Escape => match c {
                    'a' => { c = '\x07'; State::Str },
                    'b' => { c = '\x08'; State::Str },
                    'f' => { c = '\x0C'; State::Str },
                    'n' => { c = '\n'; State::Str },
                    'r' => { c = '\x0D'; State::Str },
                    't' => { c = '\t'; State::Str },
                    'v' => { c = '\x0B'; State::Str },
                    '\\' |
                    '\'' |
                    '"' |
                    '?' => State::Str,
                    'x' => { tmp = 0; rem = 2; State::Hex }
                    'u' => { tmp = 0; rem = 4; State::Hex },
                    'U' => { tmp = 0; rem = 8; State::Hex },
                    _ => panic!("Invalid escape sequence"),
                },
                State::Hex => match c {
                    '0'...'9' |
                    'a'...'f' |
                    'A'...'F' => {
                        tmp = (tmp << 4) | c.to_digit(16).unwrap();
                        rem = rem - 1;

                        if rem == 0 {
                            c = match char::from_u32(tmp) {
                                Some(c) => c,
                                None => panic!("Invalid Unicode escape"),
                            };
                            State::Str
                        } else {
                            State::Hex
                        }
                    },
                    _ => panic!("Invalid Unicode escape"),
                },
            };

            if state == State::Str {
                val.push(c);
            }
        }
        
        Some(Token::StringLiteral(val))
    }

    fn single_line_comment(&mut self) {
        for c in self.iter.by_ref() {
            if c == '\n' {
                return;
            }
        }
    }

    fn multi_line_comment(&mut self) {
        enum State {
            Other,
            Slash,
            Star,
        }

        let mut state = State::Other;
        let mut depth = 0;

        for c in self.iter.by_ref() {
            state = match state {
                State::Other => match c {
                    '/' => State::Slash,
                    '*' => State::Star,
                    _ => State::Other,
                },
                State::Slash => match c {
                    '*' => {depth += 1; State::Other}
                    _ => State::Other,
                },
                State::Star => match c {
                    '/' if depth == 0 => return,
                    '/' => {depth = depth - 1; State::Other},
                    _ => State::Other,
                },
            };
        }

        panic!("Unclosed comment");
    }
}

impl<I> Iterator for Lexer<I>
where I: Iterator<Item=char> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let c = match self.iter.next() {
            Some(c) => c,
            None => return self.eof.take(),
        };

        match (c, self.iter.next()) {
            ('/', Some('/')) => {self.single_line_comment(); self.next()},
            ('/', Some('*')) => {self.multi_line_comment(); self.next()},
            ('&', Some('&')) => Some(And),
            ('|', Some('|')) => Some(Or),
            ('=', Some('=')) => Some(Equal),
            ('!', Some('=')) => Some(NotEqual),
            ('<', Some('=')) => Some(LessEqual),
            ('>', Some('=')) => Some(GreaterEqual),
            ('+', Some('+')) => Some(Increment),
            ('-', Some('-')) => Some(Decrement),
            ('+', Some('=')) => Some(AssignPlus),
            ('-', Some('=')) => Some(AssignMinus),
            ('*', Some('=')) => Some(AssignMultiply),
            ('/', Some('=')) => Some(AssignDivide),
            ('%', Some('=')) => Some(AssignMod),
            ('-', Some('>')) => Some(Arrow),
            ('0', Some('b')) => self.lex_constant('b'),
            ('0', Some('o')) => self.lex_constant('o'),
            ('0', Some('x')) |
            ('0', Some('X')) => self.lex_constant('x'),
            (_, _) => {
                self.iter.save();
                match c {
                    '0'...'9' => self.lex_decimal(c),
                    'a'...'z' |
                    'A'...'Z' |
                    '_' => self.lex_ident(c),
                    '"' => self.lex_str_literal(),
                    _ => Some(Token::Term(c)),
                }
            },
        }
    }
}
