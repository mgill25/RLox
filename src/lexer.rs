use crate::associativity::Associativity;
use crate::{token::Token, tokentype::TokenType};
use crate::literals::Literal;

pub struct Lexer {
    source: String,
    start_idx: usize,
    curr_idx: usize,
    line_num: u32,
    src_size: usize,
    pub token_stream: Vec<Token>,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            source: "".to_string(),
            start_idx: 0,
            curr_idx: 0,
            line_num: 0,
            src_size: 0,
            token_stream: Vec::<Token>::new() 
        } 
    }

    pub fn reset(&mut self, source: &str) {
        let src_size = &source.len();
        self.source = source.to_owned();
        self.start_idx = 0;
        self.curr_idx = 0;
        self.line_num = 0;
        self.src_size = *src_size;
        self.token_stream = Vec::<Token>::new();
    }

    // top-level workhorse methods
    pub fn scan(&mut self, source: &str) {
        self.reset(source);
        while self.curr_idx < self.src_size {
            self.add_next_token();
            self.start_idx = self.curr_idx;
        }
        self.token_stream.push(
            Token::new(
                "".to_string(),
                TokenType::EOF,
                0,
                Associativity::LEFT,
                Literal::None,
            )
        );
    }

    fn add_next_token(&mut self) {
        let c: String = self.advance();
        match c.as_str() {
            "+" => self.add_token(TokenType::Plus),
            "-" => self.add_token(TokenType::Minus),
            "/" => self.add_token_or_comment(TokenType::Slash),
            "*" => self.add_token(TokenType::Star),
            " " | "\t" | "\r" => return,
            "\n" => self.line_num += 1,
            _ => {
                if self.is_digit(&c) {
                    self.add_number();
                } else if self.is_alphanumeric(&c) {
                    self.add_identifier();
                } else {
                    println!("ERROR: Unexpected character: {:?}", c);
                }
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(
            token_type,
            Literal::None
        )
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Literal) {
        let substr = &self.source[self.start_idx..self.curr_idx];
        if substr.is_empty() {
            return
        }
        let t = Token::new(
            substr.to_string(),
            token_type, 
            1,
            Associativity::LEFT,
            literal,
        );
        self.token_stream.push(t);
    }
    
    // Book-keeping methods and helper utilities that keep the state moving forward
    fn advance(&mut self) -> String {
        let byte = self.source.as_bytes()[self.curr_idx];
        let byte_to_char = std::str::from_utf8(&[byte]).unwrap().to_string();
        self.curr_idx += 1;
        byte_to_char
    }

    fn look_ahead(&self) -> String {
        if self.curr_idx >= self.src_size {
            return "\0".to_string()
        }
        let byte = self.source.as_bytes()[self.curr_idx + 1];
        let byte_to_char = std::str::from_utf8(&[byte]).unwrap().to_string();
        byte_to_char
    }

    fn peek(&self) -> String {
        if self.is_end() {
            return "\0".to_string();
        }
        let byte = self.source.as_bytes()[self.curr_idx];
        let byte_to_char = std::str::from_utf8(&[byte]).unwrap().to_string();
        byte_to_char
    }

    // Assumption: because I don't want to deal with char/String/str differences right now,
    // lets just use single-length Strings to represent chars
    // Also, lucky for me, rust developers provide all these functionalities
    // for individual characters as methods! :)
    fn is_digit(&self, char: &str) -> bool {
        for c in char.chars() {
            if !c.is_numeric() {
                return false;
            }
        }
        true
    }

    fn is_alphanumeric(&self, char: &str) -> bool {
        for c in char.chars() {
            if !c.is_alphanumeric() {
                return false;
            }
        }
        true
    }

    fn add_token_or_comment(&mut self, slash: TokenType) {
        let next_char = self.look_ahead();
        if next_char == "/" {
            while self.peek() != "\n" && !self.is_end() {
                self.advance();
            }
        } else if next_char == "*" {
            while !(self.peek() == "*" && self.look_ahead() == "/") {
                self.advance();
            }
        } else {
            self.add_token(slash)
        }
    }

    fn is_end(&self) -> bool {
        self.curr_idx >= self.src_size
    }

    fn add_number(&mut self) {
        while self.is_digit(&self.peek()) {
            self.advance();
        }
        if self.peek() == "." && self.is_digit(&self.look_ahead()) {
            self.advance();
            while self.is_digit(&self.peek()) {
                self.advance();
            }
        }
        let substr = &self.source[self.start_idx..self.curr_idx].to_string();
        let number = substr.parse::<f64>().unwrap();
        self.add_token_with_literal(
            TokenType::Number,
            Literal::Double(number)
        )
    }

    fn add_identifier(&self) {
        todo!()
    }
}
