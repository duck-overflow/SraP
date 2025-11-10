#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // Steuer
    Eof,
    Unknown,

    // Literale
    Int(i64),
    Ident(String),

    // Schlüsselwörter
    Set,
    True,
    False,

    // Operationen & Zeichen
    Plus,    //+
    Minus,   //-
    Star,    //*
    Slash,   // /
    Percent, // %
    Assign,  // =
    EqEq,    // ==
    NotEq,   // !=
    Lt,      // <
    Lte,     // <=
    Gt,      // >
    Gte,     // >=

    LParen,    // (
    RParen,    // )
    Semicolon, // ;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, col: usize) -> Self {
        Self { kind, line, col }
    }
}

pub fn keyword_or_ident(s: &str) -> TokenKind {
    match s {
        "set" => TokenKind::Set,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        _ => TokenKind::Ident(s.to_string()),
    }
}
