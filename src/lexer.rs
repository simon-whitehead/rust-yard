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
    pub fn new(input: &str) -> Lexer {
        let mut l = Lexer { 
            ast: Vec::new(),
            errors: vec![],
            iter: peek::PeekableStringIterator::new(input),
            sign: None
        };
        l.lex();
        l
    }

    fn lex(&mut self) {
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
        // If the last operator was a sign ... prefix the number with it
        let last_op = self.ast.pop();
        match last_op {
            Some(token::Token::Operator(o, _, _)) => {
                if o == '+' || o == '-' {
                    // Pop the operator and set our sign
                    self.sign = Some(o);
                }
            },
            None => (),
            Some(_) => self.ast.push(last_op.unwrap())
        }

        let mut chars = vec![self.sign.unwrap_or('+')];

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
}

