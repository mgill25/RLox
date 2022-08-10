#[derive(Debug)]
pub enum TokenType {
    Number,
    String,
    True, False,
    Plus, Minus, Star, Slash,
    Bang,
    EOF,
    BangEqual, EqualEqual
}