pub mod lexer;

use lexer::{Token, Scanner, Tokenizer};

fn main() {
    _test_token_class();
    println!();
    _test_scanner();
}

fn _test_scanner() {
    let mut stream = "a+1-bc+def-23+456".as_bytes();
    let scanner = Scanner::new(
        &mut stream,
    );
    let mut tokenizer = Tokenizer::new(scanner);
    let mut tokens: Vec<Token> = Vec::new();
    loop {
        let token = tokenizer.next();
        match token {
            Some(token) => tokens.push(token),
            None => break,
        }
    };
    let result = tokens
        .iter()
        .map(|token| token.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", result);
}

fn _test_token_class() {
    println!("{}", Token::IDENTIFIER("a".to_owned()));
    println!("{}", Token::NUMBER(12));
    println!("{}", Token::PLUS);
    println!("{}", Token::MINUS);
}
