use std::iter::Peekable;
use std::fmt;
use std::char;

pub enum Token {
    And,
    Or,
    Equal,
    NotEqual,
    LessEqual,
    GreaterEqual,
    AssignPlus,
    AssignMinus,
    AssignMultiply,
    AssignDivide,
    AssignMod,
    Increment,
    Decrement,
    Arrow,
    Ident(String),
    Binary{v: String, t: String},
    Octal{v: String, t: String},
    Decimal{v: String, t: String},
    Hexadecimal{v: String, t: String},
    Float{v: String, t: String},
    StringLiteral(String),
    Char(char),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Token::And => write!(f, "<And>"),
            &Token::Or => write!(f, "<Or>"),
            &Token::Equal => write!(f, "<Equal>"),
            &Token::NotEqual => write!(f, "<NotEqual>"),
            &Token::LessEqual => write!(f, "<LessEqual>"),
            &Token::GreaterEqual => write!(f, "<GreaterEqual>"),
            &Token::AssignPlus => write!(f, "<AssignPlus>"),
            &Token::AssignMinus => write!(f, "<AssignMinus>"),
            &Token::AssignMultiply => write!(f, "<AssignMultiply>"),
            &Token::AssignDivide => write!(f, "<AssignDivide>"),
            &Token::AssignMod => write!(f, "<AssignMod>"),
            &Token::Increment => write!(f, "<Increment>"),
            &Token::Decrement => write!(f, "<Decrement>"),
            &Token::Arrow => write!(f, "<Arrow>"),
            &Token::Ident(ref s) => write!(f, "<Ident=\"{}\">", s),
            &Token::Binary{ref v, ref t} => write!(f, "<Binary={} trail=\"{}\">", v, t),
            &Token::Octal{ref v, ref t} => write!(f, "<Octal={} trail=\"{}\">", v, t),
            &Token::Decimal{ref v, ref t} => write!(f, "<Decimal={} trail=\"{}\">", v, t),
            &Token::Hexadecimal{ref v, ref t} => write!(f, "<Hexadecimal={} trail=\"{}\">", v, t),
            &Token::Float{ref v, ref t} => write!(f, "<Float={} trail=\"{}\">", v, t),
            &Token::StringLiteral(ref s) => write!(f, "<String=\"{}\">", s),
            &Token::Char(c) => write!(f, "<{}>", c),
        }
    }
}

pub struct Lex<I: Iterator<Item=char>> {
    iter: Peekable<I>,
    saved_char: Option<char>,
}

impl<I: Iterator<Item=char>> Lex<I> {
    pub fn new(iter: I) -> Lex<I> {
        Lex { iter: iter.peekable(), saved_char: None }
    }

    fn skip_whitespace(&mut self) -> Option<char> {
        for c in self.saved_char.take().iter().cloned().chain(self.iter.by_ref()) {
            match c {
                ' '|'\t'|'\r' => { },
                '\n' => { },
                _ => return Some(c),
            };
        }

        None
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
                _ => {
                    self.saved_char = Some(c);
                    break;
                },
            };
        }

        Some(Token::Ident(val))
    }

    fn lex_constant(&mut self, t: char) -> Option<Token> {
        enum State {
            Int,
            Trail,
        }

        let mut val = String::with_capacity(20);
        let mut trail = String::with_capacity(5);

        let mut state = State::Int;

        for c in self.iter.by_ref().skip(1) {
            state = match state {
                State::Int => match c {
                    '0'...'1' if t == 'b' => State::Int,
                    '0'...'7' if t == 'o' => State::Int,
                    '0'...'9' |
                    'a'...'f' |
                    'A'...'F' if t == 'x' || t == 'X' => State::Int,
                    '_' |
                    'a'...'z' |
                    'A'...'Z' => State::Trail,
                    _         => {self.saved_char = Some(c); break;},
                },
                State::Trail => match c {
                    '_' |
                    '0'...'9' |
                    'a'...'z' |
                    'A'...'Z' => State::Trail,
                    _         => {self.saved_char = Some(c); break;},
                },
            };

            match state {
                State::Int => val.push(c),
                State::Trail => trail.push(c),
            };
        }

        Some(match t {
            'b' => Token::Binary { v: val, t: trail },
            'o' => Token::Octal { v: val, t: trail },
            _   => Token::Hexadecimal { v: val, t: trail },
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
                    _         => {self.saved_char = Some(c); break;},
                },
                State::Dec => match c {
                    '0'...'9' => State::Dec,
                    'e'|'E'   => State::ExpSign,
                    'a'...'z' |
                    'A'...'Z' => State::FloatTrail,
                    _         => {self.saved_char = Some(c); break;},
                },
                State::ExpSign => match c {
                    '+'|'-' |
                    '0'...'9' => State::Exp,
                    _         => {
                        println!("Warning: empty exponent");
                        val.pop();
                        self.saved_char = Some(c);
                        break;
                    },
                },
                State::Exp => match c {
                    '0'...'9' => State::Exp,
                    'a'...'z' |
                    'A'...'Z' => State::FloatTrail,
                    _         => {self.saved_char = Some(c); break;},
                },
                State::IntTrail => match c {
                    '0'...'9' |
                    'a'...'z' |
                    'A'...'Z' => State::IntTrail,
                    _         => {self.saved_char = Some(c); break;},
                },
                State::FloatTrail => match c {
                    '0'...'9' |
                    'a'...'z' |
                    'A'...'Z' => State::FloatTrail,
                    _         => {self.saved_char = Some(c); break;},
                },
            };

            match state {
                State::IntTrail |
                State::FloatTrail => trail.push(c),
                _ => val.push(c),
            }
        }
       
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

        for c in self.iter.by_ref().skip(1) {
            state = match state {
                State::Other => match c {
                    '/' => State::Slash,
                    '*' => State::Star,
                    _ => State::Other,
                },
                State::Slash => match c {
                    '*' => {depth = depth + 1; State::Other}
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

impl<I: Iterator<Item=char>> Iterator for Lex<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        macro_rules! next {
            ( $($ty:ident)::+ ) => {{
                self.iter.next();
                Some($($ty)::*)
            }};
        }

        let c = match self.skip_whitespace() {
            Some(c) => c,
            None => return None,
        };

        match (c, self.iter.peek().cloned()) {
            ('/', Some('/')) => {self.single_line_comment(); self.next()},
            ('/', Some('*')) => {self.multi_line_comment(); self.next()},
            ('&', Some('&')) => next!(Token::And),
            ('|', Some('|')) => next!(Token::Or),
            ('=', Some('=')) => next!(Token::Equal),
            ('!', Some('=')) => next!(Token::NotEqual),
            ('<', Some('=')) => next!(Token::LessEqual),
            ('>', Some('=')) => next!(Token::GreaterEqual),
            ('+', Some('+')) => next!(Token::Increment),
            ('-', Some('-')) => next!(Token::Decrement),
            ('+', Some('=')) => next!(Token::AssignPlus),
            ('-', Some('=')) => next!(Token::AssignMinus),
            ('*', Some('=')) => next!(Token::AssignMultiply),
            ('/', Some('=')) => next!(Token::AssignDivide),
            ('%', Some('=')) => next!(Token::AssignMod),
            ('-', Some('>')) => next!(Token::Arrow),
            ('_', _) |
            ('a'...'z', _) |
            ('A'...'Z', _) => self.lex_ident(c),
            ('0', Some(t @ 'b')) |
            ('0', Some(t @ 'o')) |
            ('0', Some(t @ 'x')) |
            ('0', Some(t @ 'X')) => self.lex_constant(t),
            ('.', Some('0'...'9')) |
            ('0'...'9', _) => self.lex_decimal(c),
            ('"', _) => self.lex_str_literal(),
            (_, _) => Some(Token::Char(c)),
        }
    }
}
