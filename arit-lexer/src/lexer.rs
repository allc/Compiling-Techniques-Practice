use std::{fmt, io::BufRead};

pub enum Token {
    IDENTIFIER(String),
    NUMBER(u32),
    PLUS,
    MINUS,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::IDENTIFIER(name) => write!(f, "IDENTIFIER({})", name),
            Token::NUMBER(n) => write!(f, "NUMBER({})", n),
            Token::PLUS => write!(f, "PLUS"),
            Token::MINUS => write!(f, "MINUS"),
        }
    }
}

pub struct Scanner<'a> {
    stream: &'a mut dyn BufRead,
    buffer: Option<char>,
}

impl Scanner<'_> {
    pub fn new(stream: &mut dyn BufRead) -> Scanner {
        Scanner {
            stream: stream,
            buffer: None,
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        match self.buffer {
            Some(c) => Some(c),
            None => {
                self.buffer = self.next();
                self.buffer
            },
        }
    }

    pub fn next(&mut self) -> Option<char> {
        match self.buffer {
            Some(c) => {
                self.buffer = None;
                Some(c)
            },
            None => {
                let mut buf = vec![0u8];
                match self.stream.read_exact(&mut buf) {
                    Ok(_) => Some(buf[0] as char),
                    Err(_) => None,
                }
            },
        }
    }
}

pub struct Tokenizer<'a> {
    scanner: Scanner<'a>,
}

impl Tokenizer<'_> {
    pub fn new(scanner: Scanner) -> Tokenizer {
        Tokenizer {
            scanner: scanner,
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        let c = self.scanner.next();
        match c {
            Some(c) => {
                let token = if c.is_whitespace() {
                    self.next()
                } else if c == '+' {
                    Some(Token::PLUS)
                } else if c == '-' {
                    Some(Token::PLUS)
                } else if c.is_alphabetic() {
                    let mut name = c.to_string();
                    let token = loop {
                        let c = self.scanner.peek();
                        if c.is_some() && c.unwrap().is_alphanumeric() {
                            name.push(c.unwrap());
                            self.scanner.next();
                        } else {
                            break Some(Token::IDENTIFIER(name))
                        }
                    };
                    token
                } else if c.is_numeric() {
                    let mut digits = c.to_string();
                    let token = loop {
                        let c = self.scanner.peek();
                        if c.is_some() && c.unwrap().is_numeric() {
                            digits.push(c.unwrap());
                            self.scanner.next();
                        } else {
                            let value: u32 = digits.parse().unwrap();
                            break Some(Token::NUMBER(value));
                        }
                    };
                    token
                } else {
                    None
                };
                token
            },
            None => None,
        }
    }
}
