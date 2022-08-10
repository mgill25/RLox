use crate::tokentype::TokenType;
use crate::associativity::Associativity;
use crate::literals::Literal;

#[derive(Debug)]
pub struct Token {
    lexeme: String,
    token_type: TokenType,
    precedence: u32,
    assoc: Associativity,
    literal: Literal,
}

impl Token {
    pub fn new(lexeme: String, token_type: TokenType, precedence: u32, assoc: Associativity, literal: Literal) -> Self { 
        Self { lexeme, token_type, precedence, assoc, literal } 
    }
}