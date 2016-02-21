extern crate rustyard;

use std::env;

fn main() {
    match env::args().nth(1) {
        Some(input) => {
            let mut shunting_yard = rustyard::ShuntingYard::new();

            println!("Input is: {}", input);

            match shunting_yard.calculate(&input[..]) {
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
