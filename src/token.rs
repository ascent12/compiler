use std::fmt;

#[derive(PartialEq)]
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
