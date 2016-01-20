use std::iter::Peekable;
use std::iter::FromIterator;
use std::str::Chars;

use token;

pub struct Lexer<'a> {
    raw_input: &'a str,
    ast: Vec<token::Token>,
    pub errors: Vec<String> 
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer { 
            raw_input: input,
            ast: vec![],
            errors: vec![]
        }
    }

    pub fn lex(&mut self) -> Vec<token::Token> {
        let tokens = vec![];

        self.consume_input(self.raw_input, tokens)
    }

    fn consume_input(&mut self, raw_input: &str, mut tokens: Vec<token::Token>) -> Vec<token::Token> {
        println!("Current input: {}", raw_input);
        let mut chars = raw_input.chars().peekable();

        // Iterate over each character in the input
        match chars.clone().peek() {
            Some(c) if c.is_whitespace() => { tokens = self.consume_input(&String::from_iter(chars)[..], tokens); },
            Some(c) if c.is_numeric() => {
                // Grab the number (allowing for possibly decimals)
                let mut number = self.consume_number(&mut chars);
                println!("CONSUMED A NUMBER: {}", number);

                // Add a numeric token to the list of tokens
                let number = match number.parse() {
                    Ok(val) => {
                        tokens.push(token::Token::DecimalNumber(val));
                    },
                    Err(E) => self.errors.push(format!("FATAL: {}", E))
                };

                println!("DEBUG: {}", c);

                tokens = self.consume_input(&String::from_iter(chars)[..], tokens);
            }
            Some(c) => self.errors.push(format!("Unknown identifier: {}", c)),
            None => ()
        }

        tokens
    }

    // Consumes the iterator until it reaches the end of a number
    fn consume_number(&mut self, it: &mut Peekable<Chars>) -> String {
        let mut chars = vec![];

        loop {
            match it.peek() {
                Some(c) if c.is_numeric() || *c == '.' => { println!("ISNUMERIC"); chars.push(*c); },
                Some(c) if !c.is_numeric() => break,
                Some(c) => println!("Peeking at: {}", c),
                None => { println!("None hit"); break; },
            }
            it.next();
        }

        println!("{:?}", chars);

        chars.into_iter().collect()
    }
}
