#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Ident(String),
    // Single character terminals
    Term(char),

    // Binary Operators
    And,
    Or,
    Equal,
    NotEqual,
    LessEqual,
    GreaterEqual,

    // Assignment operators
    AssignPlus,
    AssignMinus,
    AssignMultiply,
    AssignDivide,
    AssignMod,

    // Unary operators
    Increment,
    Decrement,
    Arrow,

    // Constants
    Binary{v: String, t: String},
    Octal{v: String, t: String},
    Decimal{v: String, t: String},
    Hexadecimal{v: String, t: String},
    Float{v: String, t: String},
    StringLiteral(String),

    // Keywords
    If,
    Else,
    Do,
    While,
    For,
    // Types
    U8,
    U16,
    U32,
    U64,
    Usize,
    I8,
    I16,
    I32,
    I64,
    Isize,
}
