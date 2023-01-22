use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum TokenKind {
    Symbol, Integer, Let, Eq
}

#[derive(Debug, PartialEq)]
pub struct Token {
    text: String,
    kind: TokenKind
}

impl Token {
    fn new(text: String, kind: TokenKind) -> Self {
        Self { text, kind }
    }
}

pub fn is_string_numeric(value: &str) -> bool {
    for character in value.chars() {
        if !character.is_numeric() {
            return false
        }
    }
    return true
}

pub fn lex(code: String) -> Vec<Token> {
    let keyws: HashMap<&str, TokenKind> = HashMap::from([("let", TokenKind::Let)]);
    let operators: HashMap<&str, TokenKind> = HashMap::from([("=", TokenKind::Eq)]);

    let mut tokens: Vec<Token> = Vec::new();
    let mut buf = String::new();
    for c in code.chars() {
        if let Some(op) = operators.get(&*c.to_string()) {
            tokens.push(Token::new(buf.clone(), TokenKind::Symbol));
            tokens.push(Token::new(c.to_string(), *op));
            buf.clear();
        } else if !c.is_whitespace() {
            buf.push(c);
        } else if c.is_whitespace() && !buf.is_empty() {
            if is_string_numeric(&buf) {
                tokens.push(Token::new(buf.clone(), TokenKind::Integer));
            } else if let Some(tt) = keyws.get(&*buf) {
                tokens.push(Token::new(buf.clone(), *tt))
            } else {
                tokens.push(Token::new(buf.clone(), TokenKind::Symbol));
            }
            buf.clear();
        }
    }
    if !buf.is_empty() {
        if is_string_numeric(&buf) {
            tokens.push(Token::new(buf.clone(), TokenKind::Integer));
        } else if let Some(tt) = keyws.get(&*buf) {
            tokens.push(Token::new(buf.clone(), *tt))
        } else {
            tokens.push(Token::new(buf.clone(), TokenKind::Symbol));
        }
    }
    return tokens
}
