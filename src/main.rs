use crate::lexer::Lexer;

mod lexer;
mod tokentype;
mod token;
mod associativity;
mod literals;

fn run_lexer(lexer: &mut Lexer, source: &str) {
    lexer.scan(source);
    for token in &lexer.token_stream {
        println!("{:?}", token);
    }
    println!("--------------------");
}

fn main() {
    let src1 = "2 + 3 * 4";
    let mut lexer = Lexer::new();
    run_lexer(&mut lexer, src1);
    let src2 = "1 + 2 / 3 * 4 - -5";
    run_lexer(&mut lexer, src2);
}
