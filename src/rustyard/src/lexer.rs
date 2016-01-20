use std::fmt;
use std::f64;
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
            ast: Vec::new(),
            errors: vec![]
        }
    }

    pub fn lex(&mut self) {
        let tokens = vec![];

        let ast: Vec<token::Token> = self.consume_input(self.raw_input, tokens);
        self.ast = ast;
    }

    fn consume_input(&mut self, raw_input: &str, mut tokens: Vec<token::Token>) -> Vec<token::Token> {
        let mut chars = raw_input.chars().peekable();

        // Iterate over each character in the input
        match chars.clone().peek() {
            Some(c) if c.is_whitespace() => { chars.next(); tokens = self.consume_input(&String::from_iter(chars)[..], tokens); },
            Some(c) if c.is_numeric() => {
                // Grab the number (allowing for possibly decimals)
                let mut number = self.consume_number(&mut chars);
                // Add a numeric token to the list of tokens
                let number = match number.parse() {
                    Ok(val) => {
                        tokens.push(token::Token::DecimalNumber(val));
                    },
                    Err(E) => self.errors.push(format!("FATAL: {}", E))
                };

                tokens = self.consume_input(&String::from_iter(chars)[..], tokens);
            },
            Some(c) if *c == '+' || *c == '-' => {
                // Add the operator and advance the iterator
                self.add_op_and_continue(*c, 2, &mut chars, &mut tokens);
                tokens = self.consume_input(&String::from_iter(chars)[..], tokens);
            },
            Some(c) if *c == '*' || *c == '/' => {
                // Add the operator and advance the iterator
                self.add_op_and_continue(*c, 4, &mut chars, &mut tokens);
                tokens = self.consume_input(&String::from_iter(chars)[..], tokens);
            },
            Some(c) => self.errors.push(format!("Unknown identifier: {}", c)),
            None => ()
        }

        tokens
    }

    // Consumes the iterator until it reaches the end of a number
    fn consume_number(&mut self, it: &mut Peekable<Chars>) -> String {
        let mut chars = vec![];

        // Loop over every character until we reach a non-numeric one
        loop {
            match it.peek() {
                Some(c) if c.is_numeric() || *c == '.' => chars.push(*c),
                Some(c) if !c.is_numeric() => break,
                //Some(c) => println!("Peeking at: {}", c),
                None => break,
                _ => ()
            }
            it.next();
        }

        // Return out number as a String
        chars.into_iter().collect()
    }

    fn add_op_and_continue(&mut self, c: char, precedence: u32, chars: &mut Peekable<Chars>, tokens: &mut Vec<token::Token>) {
        chars.next();
        tokens.push(token::Token::Operator(c, precedence));
    }
}

// Allows the Lexer AST to be printed as a string
impl<'a> fmt::Display for Lexer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut result = String::new(); // String to output the result to
        let ast = self.ast.to_vec();    // Copy the AST into its own vector so we can consume it

        // Loop over each item in the AST and print a String representation of it
        for t in ast {
            match t {
                token::Token::Operator(c, p) => result.push(c),
                token::Token::DecimalNumber(n) => result.push_str(&n.to_string()[..]),
                _ => ()
            };
            result.push_str(" "); // Space separated
        }

        // Return the result
        write!(f, "{}", result)
    }
}
