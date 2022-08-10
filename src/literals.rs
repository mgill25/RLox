// Representing various types that a literal value can have
// in our Programming language
#[derive(Debug)]
pub enum Literal {
    None,
    Double(f64),
    String(String),
}