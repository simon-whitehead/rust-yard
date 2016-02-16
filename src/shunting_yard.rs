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
    pub errors: Vec<String>
}

impl<'a> ShuntingYard<'a> {
    pub fn new(raw_input: &str) -> ShuntingYard {
        let mut yard = ShuntingYard {
            lexer: lexer::Lexer::new(raw_input),
            output_queue: vec![],
            stack: vec![],
            errors: vec![]
        };

        // If there were Lexer errors, add them now.
        let lexer_errors = yard.lexer.errors.clone();
        yard.errors.extend(lexer_errors);

        // Transform the Lexer input via the Shunting Yard algorithm
        yard.transform();
        yard
    }

    /// calculate returns a 64-bit floating value after
    /// parsing the Reverse Polish Notation represented
    /// by the output_queue.
    pub fn calculate(&self) -> Result<f64, Vec<String>> {
        // If there are lexer errors, return early with them
        if self.errors.len() > 0 {
            return Err(self.errors.clone())
        }

        match calc::calculate(&self.output_queue) {
            Some(n) => {
                Ok(n)
            },
            _ => Err(vec!["Unable to calculate a result".to_string()])
        }
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
                _ => ()
            }
        }

        // Are there any operators left on the stack?
        while self.stack.len() > 0 {
            // Pop them off and push them to the output_queue
            let op = self.stack.pop();
            match op {
                Some(token::Token::Operator(o, oa, op)) => {
                    self.output_queue.push(token::Token::Operator(o, oa, op)); 
                },
                Some(token::Token::LeftParenthesis) => {
                    self.errors.push("Unbalanced parenthesis".to_string());
                    break;
                },
                Some(token::Token::RightParenthesis) => {
                    self.errors.push("Unbalanced parenthesis".to_string());
                    break;
                },
                _ => ()
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
                token::Token::DecimalNumber(n) => result.push_str(&n.to_string()[..]),
                token::Token::LeftParenthesis => result.push_str("("),
                token::Token::RightParenthesis => result.push_str(")"),
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
                token::Token::DecimalNumber(n) => result.push_str(&n.to_string()[..]),
                token::Token::LeftParenthesis => result.push_str("("),
                token::Token::RightParenthesis => result.push_str(")"),
                _ => ()
            };

            if *tok != token::Token::Whitespace {
                result.push_str(" "); // Space separated
            }
        }

        result
    }

}
