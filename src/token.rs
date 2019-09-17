
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    String(String),
    Value,
    Indent,
    Newline,
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub lexeme: &'a str,
    pub line: u64,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, lexeme: &'a str, line: u64) -> Self {
        Token {
            kind: kind,
            lexeme: lexeme,
            line: line,
        }
    }
}

