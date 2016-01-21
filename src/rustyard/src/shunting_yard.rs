
use lexer;
use token;

pub struct ShuntingYard<'a> {
    lexer: lexer::Lexer<'a>,
    output_queue: Vec<token::Token>,
    stack: Vec<token::Token>
}

impl<'a> ShuntingYard<'a> {
    pub fn new(raw_input: &str) -> ShuntingYard {
        let mut yard = ShuntingYard {
            lexer: lexer::Lexer::new(raw_input),
            output_queue: vec![],
            stack: vec![]
        };

        yard.transform();
        yard
    }

    // Transforms the input from the Lexer in to the output_queue
    // and stack based on the Shunting Yard algorithm
    fn transform(&mut self) {
        // Iterate over each token and move it based on the algorithm
        for t in self.lexer.ast.to_vec() {
            // If the token is a number, then add it to the output queue
            match t {
                token::Token::WholeNumber(_) => self.output_queue.push(t),
                token::Token::DecimalNumber(_) => self.output_queue.push(t),
                token::Token::Operator(o1, o1_associativity, o1_precedence) => {
                    match self.stack.last() {
                        Some(&token::Token::Operator(_, _, o2_precedence)) => {
                            if (o1_associativity == token::LEFT_ASSOCIATIVE &&
                               o1_precedence <= o2_precedence) ||
                               (o1_associativity == token::RIGHT_ASSOCIATIVE &&
                               o1_precedence < o2_precedence) {
                                self.output_queue.push(self.stack.pop().unwrap());
                            } else {
                                self.stack.push(token::Token::Operator(o1, o1_associativity, o1_precedence))
                            }
                        },
                        _ => self.stack.push(token::Token::Operator(o1, o1_associativity, o1_precedence))
                    }
                },
                _ => ()
            }
        }

        // Are there any operators left on the stack?
        while self.stack.len() > 0 {
            self.output_queue.push(self.stack.pop().unwrap());
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new(); // String to output the result
        let output_queue = self.output_queue.to_vec(); // Make a copy of the output queue

        // Iterate over the output queue and print each one to the result
        for t in output_queue {
            match t {
                token::Token::Operator(c, _, _) => result.push(c),
                token::Token::DecimalNumber(n) => result.push_str(&n.to_string()[..]),
                _ => ()
            };

            result.push_str(" "); // Space separated
        }

        result
    }

    pub fn to_string_ast(&self) -> String {
        let mut result = String::new(); // String to output the result to
        let ast = self.lexer.ast.to_vec();    // Copy the AST into its own vector so we can consume it

        // Loop over each item in the AST and print a String representation of it
        for t in ast {
            match t {
                token::Token::Operator(c, _, _) => result.push(c),
                token::Token::DecimalNumber(n) => result.push_str(&n.to_string()[..]),
                _ => ()
            };
            result.push_str(" "); // Space separated
        }

        // Return the result
        result
    }
}
