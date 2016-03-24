use token;

// Calculate accepts input tokens that are
// ordered according to Reverse Polish Notation
// and returns a result
pub fn calculate(input: &Vec<token::Token>) -> Result<f64, Vec<String>> {
    let mut input = input.clone();
    let mut stack = Vec::new();
    let mut len = input.len();
    let mut errors = Vec::new();

    // Iterate over the tokens and calculate a result
    while len > 0 {
        let tok = input.remove(0);
        match tok {
            token::Token::DecimalNumber(n) => stack.push(token::Token::DecimalNumber(n)),
            token::Token::Operator(o, _, _) => {
                let right = stack.pop();
                let left = stack.pop();

                match (left, right) {
                    (Some(token::Token::DecimalNumber(n1)), Some(token::Token::DecimalNumber(n2))) => stack.push(token::Token::DecimalNumber(operate(o, n1, n2))),
                    _ => break
                }
            },
            token::Token::WholeNumber(n) => stack.push(token::Token::DecimalNumber(n as f64)),
            token::Token::FunctionCall(function_name) => {
                match &function_name as &str {
                    "cos" => {
                        let arg = stack.pop();

                        if let Some(token::Token::DecimalNumber(n1)) = arg {
                            stack.push(token::Token::DecimalNumber(n1.cos()));
                        }
                    },
                    "sin" => {
                        let arg = stack.pop();

                        if let Some(token::Token::DecimalNumber(n1)) = arg {
                            stack.push(token::Token::DecimalNumber(n1.sin()));
                        }
                    },
                    "tan" => {
                        let arg = stack.pop();

                        if let Some(token::Token::DecimalNumber(n1)) = arg {
                            stack.push(token::Token::DecimalNumber(n1.tan()));
                        }
                    },
                    "floor" => {
                        let arg = stack.pop();

                        if let Some(token::Token::DecimalNumber(n1)) = arg {
                            stack.push(token::Token::DecimalNumber(n1.floor()));
                        }
                    },
                    "ceil" => {
                        let arg = stack.pop();

                        if let Some(token::Token::DecimalNumber(n1)) = arg {
                            stack.push(token::Token::DecimalNumber(n1.ceil()));
                        }
                    },
                    "round" => {
                        let arg = stack.pop();

                        if let Some(token::Token::DecimalNumber(n1)) = arg {
                            stack.push(token::Token::DecimalNumber(n1.round()));
                        }
                    },
                    "trunc" => {
                        let arg = stack.pop();

                        if let Some(token::Token::DecimalNumber(n1)) = arg {
                            stack.push(token::Token::DecimalNumber(n1.trunc()));
                        }
                    },
                    "fract" => {
                        let arg = stack.pop();

                        if let Some(token::Token::DecimalNumber(n1)) = arg {
                            stack.push(token::Token::DecimalNumber(n1.fract()));
                        }
                    },
                    "pow" => {
                        let right = stack.pop();
                        let left = stack.pop();

                        if let (Some(token::Token::DecimalNumber(n1)), Some(token::Token::DecimalNumber(n2))) = (left, right) {
                            stack.push(token::Token::DecimalNumber(n1.powf(n2)));
                        }
                    },
                    "sqrt" => {
                        let arg = stack.pop();
                        match arg {
                            Some(token::Token::DecimalNumber(n)) => stack.push(token::Token::DecimalNumber((n as f64).sqrt())),
                            Some(token::Token::WholeNumber(n)) => stack.push(token::Token::DecimalNumber((n as f64).sqrt())),
                            _ => ()
                        }
                    },
                    "max" => {
                        let right = stack.pop();
                        let left = stack.pop();

                        if let (Some(token::Token::DecimalNumber(n1)), Some(token::Token::DecimalNumber(n2))) = (left, right) {
                            stack.push(token::Token::DecimalNumber(n1.max(n2)));
                        }
                    },
                    "min" => {
                        let right = stack.pop();
                        let left = stack.pop();

                        if let (Some(token::Token::DecimalNumber(n1)), Some(token::Token::DecimalNumber(n2))) = (left, right) {
                            stack.push(token::Token::DecimalNumber(n1.min(n2)));
                        }
                    },
                    "sum" => {
                        let mut values = Vec::new();

                        loop {
                            match stack.last() {
                                Some(&token::Token::DecimalNumber(n)) => {
                                    values.push(n);
                                    stack.pop();
                                },
                                _ => break
                            }
                        }

                        stack.push(token::Token::DecimalNumber(values.iter().fold(0.0, |acc, val| acc + val)));
                    },
                    _ => errors.push(format!("Unknown identifier: {}", function_name))
                }
            },
            _ => ()
        }
        len = input.len();
    }

    if errors.len() > 0 {
        Err(errors)
    } else {
        let result = stack.pop();

        match result {
            Some(token::Token::DecimalNumber(n)) => Ok(n),
            _ => Err(errors)
        }
    }
}

fn operate(operator: char, left: f64, right: f64) -> f64 {
    match operator {
        '+' => left + right,
        '-' => left - right,
        '*' => left * right,
        '/' => left / right,
        '^' => left.powf(right),
        _ => 0f64
    }
}
