use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone, Copy)]
enum TokenType {
    Symbol, Integer, Print, Null
}

#[derive(Debug, PartialEq)]
struct Token {
    text: String,
    typ: TokenType
}

impl Token {
    fn new(text: String, typ: TokenType) -> Self {
        Self {
            text: text,
            typ: typ
        }
    }
}

pub fn is_string_numeric(value: &String) -> bool {
    for character in value.chars() {
        if !character.is_numeric() {
            return false
        }
    }
    return true
}

fn lex(code: String) -> Vec<Token> {
    let keyws: HashMap<String, TokenType> = HashMap::from([(String::from("print"), TokenType::Print)]);
    let mut tokens: Vec<Token> = vec![];
    let mut buf = String::new();
    for c in code.chars() {
        if !c.is_whitespace() {
            buf.push(c);
        }
        if c.is_whitespace() && !buf.is_empty() {
            if is_string_numeric(&buf) {
                tokens.push(Token::new(buf.clone(), TokenType::Integer));
            } else {
                if let Some(tt) = keyws.get(&buf) {
                    tokens.push(Token::new(buf.clone(), *tt))
                } else {
                    tokens.push(Token::new(buf.clone(), TokenType::Symbol));
                }
            }
            buf.clear();
        }
    }
    if !buf.is_empty() {
        if is_string_numeric(&buf) {
            tokens.push(Token::new(buf.clone(), TokenType::Integer));
        } else {
             if let Some(tt) = keyws.get(&buf) {
                 tokens.push(Token::new(buf.clone(), *tt))
             } else {
                 tokens.push(Token::new(buf.clone(), TokenType::Symbol));
             }
        }
    }
    return tokens;
}

fn main() {
    println!("{:#?}", lex(String::from("print67")));
}
