/*
 * Rustyard - Simon Whitehead, 2016
 */

use std;

use lexer;
use rpn_calculator as calc;
use token;

/// The ShuntingYard struct transforms an expression
/// to a 64-bit floating point value
pub struct ShuntingYard<'a> {
    lexer: lexer::Lexer<'a>,
    output_queue: Vec<token::Token>,
    stack: Vec<token::Token>,
    errors: Vec<String>
}

impl<'a> ShuntingYard<'a> {
    pub fn new() -> ShuntingYard<'a> {
        ShuntingYard {
            lexer: lexer::Lexer::new(),
            output_queue: vec![],
            stack: vec![],
            errors: vec![]
        }
    }

    /// calculate returns a 64-bit floating value after
    /// parsing the Reverse Polish Notation represented
    /// by the output_queue.
    pub fn calculate(&mut self, raw_input: &'a str) -> Result<f64, Vec<String>> {
        // Clear out everything
        self.output_queue.clear();
        self.stack.clear();
        self.errors.clear();

        // Lex the input
        self.lexer.lex(raw_input);

        // If there were Lexer errors, add them now.
        let lexer_errors = self.lexer.errors.clone();
        self.errors.extend(lexer_errors);

        // Transform the Lexer input via the Shunting Yard algorithm
        self.transform();

        // If there are lexer errors, return early with them
        if self.errors.len() > 0 {
            println!("Errors: {:?}", self.errors);
            return Err(self.errors.clone())
        }

        calc::calculate(&self.output_queue)
    }

    // Transforms the input from the Lexer in to the output_queue
    // and stack based on the Shunting Yard algorithm
    fn transform(&mut self) {
        // Iterate over each token and move it based on the algorithm
        for tok in &self.lexer.ast {
            // If the token is a number, then add it to the output queue
            match *tok {
                token::Token::WholeNumber(_) => self.output_queue.push(tok.to_owned()),
                token::Token::DecimalNumber(_) => self.output_queue.push(tok.to_owned()),
                token::Token::FunctionCall(_) => self.stack.push(tok.to_owned()),
                token::Token::Operator(o1, o1_associativity, o1_precedence) => {
                    while self.stack.len() > 0 {
                        match self.stack.last() {
                            Some(&token::Token::Operator(_, _, o2_precedence)) => {
                                if (o1_associativity == token::LEFT_ASSOCIATIVE &&
                                   o1_precedence <= o2_precedence) ||
                                   (o1_associativity == token::RIGHT_ASSOCIATIVE &&
                                   o1_precedence < o2_precedence) { 
                                       self.output_queue.push(self.stack.pop().unwrap());
                                   } else {
                                       break
                                   }
                            },
                            Some(&token::Token::FunctionCall(_)) => {
                                self.output_queue.push(self.stack.pop().unwrap());
                            },
                            _ => break
                        }
                    }
                    self.stack.push(token::Token::Operator(o1, o1_associativity, o1_precedence));
                },
                token::Token::LeftParenthesis => self.stack.push(token::Token::LeftParenthesis),
                token::Token::RightParenthesis => {
                    loop {
                        match self.stack.last() {
                            Some(&token::Token::LeftParenthesis) => {
                                self.stack.pop().unwrap(); 
                                break;
                            },
                            None => {
                                self.errors.push("Unbalanced parenthesis".to_string());
                                break;
                            },
                            _ => self.output_queue.push(self.stack.pop().unwrap()),
                        }
                    }
                },
                token::Token::Comma => {
                    loop {
                        match self.stack.last() {
                            Some(&token::Token::LeftParenthesis) => {
                                break;
                            },
                            _ => self.output_queue.push(self.stack.pop().unwrap())
                        }
                    }
                },
                _ => ()
            }
        }

        // Are there any operators left on the stack?
        while self.stack.len() > 0 {
            // Pop them off and push them to the output_queue
            let op = self.stack.pop();
            match op {
                Some(token::Token::LeftParenthesis) => {
                    self.errors.push("Unbalanced parenthesis".to_string());
                    break;
                },
                Some(token::Token::RightParenthesis) => {
                    self.errors.push("Unbalanced parenthesis".to_string());
                    break;
                },
                _ => self.output_queue.push(op.unwrap())
            }
        }
    }

    /// to_string_ast returns the string representation of the
    /// Lexer tokens.
    pub fn to_string_ast(&self) -> String {
        let mut result = String::new(); // String to output the result to

        // Loop over each item in the AST and print a String representation of it
        for tok in &self.lexer.ast {
            match *tok {
                token::Token::Operator(c, _, _) => result.push(c),
                token::Token::FunctionCall(ref f) => result.push_str(&f.clone()[..]),
                token::Token::DecimalNumber(n) => result.push_str(&n.to_string()[..]),
                token::Token::LeftParenthesis => result.push_str("("),
                token::Token::RightParenthesis => result.push_str(")"),
                token::Token::Comma => result.push_str(","),
                _ => ()
            };

            if *tok != token::Token::Whitespace {
                result.push_str(" "); // Space separated
            }
        }

        // Return the result
        result
    }
}

impl<'a> std::string::ToString for ShuntingYard<'a> {
    /// to_string returns the string representation of the Shunting Yard
    /// algorithm in Reverse Polish Notation.
    fn to_string(&self) -> String {
        let mut result = String::new(); // String to output the result

        // Iterate over the output queue and print each one to the result
        for tok in &self.output_queue {
            match *tok {
                token::Token::Operator(c, _, _) => result.push(c),
                token::Token::FunctionCall(ref f) => result.push_str(&f.clone()[..]),
                token::Token::DecimalNumber(n) => result.push_str(&n.to_string()[..]),
                token::Token::LeftParenthesis => result.push_str("("),
                token::Token::RightParenthesis => result.push_str(")"),
                token::Token::Comma => result.push_str(","),
                _ => ()
            };

            if *tok != token::Token::Whitespace {
                result.push_str(" "); // Space separated
            }
        }

        result
    }

}
