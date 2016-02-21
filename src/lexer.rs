use token;
use peekable_string_iterator as peek;

/// The Lexer struct represents a lexer that tokenizes
/// the string input.
pub struct Lexer<'a> {
    iter: peek::PeekableStringIterator<'a>,
    sign: Option<char>,
    pub ast: Vec<token::Token>,
    pub errors: Vec<String> 
}

impl<'a> Lexer<'a> {
    pub fn new() -> Lexer<'a> {
        Lexer { 
            ast: Vec::new(),
            errors: vec![],
            iter: peek::PeekableStringIterator::new(),
            sign: None
        }
    }

    pub fn lex(&mut self, raw_input: &'a str) {
        // Clear out everything
        self.ast.clear();
        self.errors.clear();

        self.iter.set_input(raw_input);
        self.consume_input();
    }

    // Recursively consume the input
    fn consume_input(&mut self) {
        // Should we skip advancing if a sub method has done it for us?
        let mut skip_advance = false;

        // Peek the next character
        let peeked: Option<char> = match self.iter.peek() {
            Some(&c) => Some(c),
            None => None
        };

        // Decide what to do
        match peeked {
            Some(c) if c.is_whitespace() => {
                // Reset the sign
                self.sign = None;

                self.ast.push(token::Token::Whitespace);
            }, 
            Some(c) if c.is_numeric() => {
                // Grab the number (allowing for possible decimals)
                let number = self.consume_number();
                // Add a numeric token to the list of tokens
                match number.parse() {
                    Ok(val) => {
                        self.ast.push(token::Token::DecimalNumber(val));
                    },
                    Err(e) => self.errors.push(format!("FATAL: {}", e))
                }
                skip_advance = true;
            },
            Some(c) if c == '+' || c == '-' => {
                // Add the operator and advance the iterator
                self.ast.push(token::Token::Operator(c, token::LEFT_ASSOCIATIVE, 2));
            },
            Some(c) if c == '*' || c == '/' => {
                // Add the operator and advance the iterator
                self.ast.push(token::Token::Operator(c, token::LEFT_ASSOCIATIVE, 3));
            },
            Some(c) if c == '(' => self.ast.push(token::Token::LeftParenthesis),
            Some(c) if c == ')' => self.ast.push(token::Token::RightParenthesis),
            Some(c) if c == '^' => self.ast.push(token::Token::Operator(c, token::RIGHT_ASSOCIATIVE, 4)),
            Some(c) => self.errors.push(format!("Unknown identifier: {}", c)),
            None => return
        }
        // Advance the iterator and continue consuming the input
        if !skip_advance {
            self.iter.advance();
        }
        self.consume_input();
    }

    // Consumes the iterator until it reaches the end of a number
    fn consume_number(&mut self) -> String {
        // Decipher the sign of the number we want to consume
        self.decipher_sign();

        // Initialize our number with the given sign
        let mut chars = vec![self.sign.unwrap_or('+')];

        // Reset the sign
        self.sign = None;

        // Loop over every character until we reach a non-numeric one
        loop {
            match self.iter.peek() {
                Some(c) if c.is_numeric() || *c == '.' => chars.push(*c),
                Some(c) if !c.is_numeric() => break,
                //Some(c) => println!("Peeking at: {}", c),
                None => break,
                _ => break
            }
            self.iter.advance();
        }

        // Return out number as a String
        chars.into_iter().collect()
    }

    fn decipher_sign(&mut self) {
        // If the last operator was a sign ... set the sign
        let last_op = self.ast.last().cloned();
        match last_op {
            Some(token::Token::Operator(o, _, _)) => {
                if o == '+' || o == '-' {
                    // Pop the operator (because its not an operator .. its indicating the numbers'
                    // sign) and store our sign
                    self.ast.pop();
                    self.sign = Some(o);
                }
            },
            _ => ()
        }
    }
}

