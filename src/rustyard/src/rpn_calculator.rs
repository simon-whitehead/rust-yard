use token;

// Calculate accepts input tokens that are
// ordered according to Reverse Polish Notation
// and returns a result
pub fn calculate(input: &Vec<token::Token>) -> f64 {
    let mut input = input.clone();
    let mut stack = Vec::new();
    let mut len = input.len();

    // Iterate over the tokens and calculate a result
    while len > 0 {
        let tok = input.remove(0);
        match tok {
            token::Token::DecimalNumber(n) => stack.push(token::Token::DecimalNumber(n)),
            token::Token::Operator(o, _, _) => {
                let left = stack.pop();
                let right = stack.pop();

                match (left, right) {
                    (Some(token::Token::DecimalNumber(n1)), Some(token::Token::DecimalNumber(n2))) => stack.push(token::Token::DecimalNumber(operate(o, n1, n2))),
                    _ => break
                }
            },
            token::Token::WholeNumber(n) => stack.push(token::Token::DecimalNumber(n as f64)),
            token::Token::FunctionCall(_, _) => break,
            token::Token::LeftParenthesis => (),
            token::Token::RightParenthesis => ()
        }
        len = input.len();
    }

    let result = stack.pop();

    match result {
        Some(token::Token::DecimalNumber(n)) => n,
        _ => -1f64 
    }
}

fn operate(operator: char, left: f64, right: f64) -> f64 {
    match operator {
        '+' => left + right,
        '-' => left - right,
        '*' => left * right,
        '/' => left / right,
        _ => 0f64
    }
}
