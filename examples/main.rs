extern crate rustyard;

use std::env;

fn main() {
    match env::args().nth(1) {
        Some(input) => {
            let shunting_yard = rustyard::ShuntingYard::new(&input[..]);

            println!("Input is: {}", input);

            match shunting_yard.calculate() {
                Ok(n) => {
                    println!("Lexer result: {}", shunting_yard.to_string_ast());
                    println!("Shunting Yard result: {}", shunting_yard.to_string());
                    println!("Equation equals: {}", n);
                },
                Err(errors) =>  {
                    for err in errors {
                        println!("ERR: {}", err);
                    }
                }
            }
        },
        None => println!("Please supply an expression")
    };
}
