extern crate rustyard;

use std::env;

fn main() {
    match env::args().nth(1) {
        Some(input) => {
            let shunting_yard = rustyard::ShuntingYard::new(&input[..]);

            println!("Input is: {}", input);

            if shunting_yard.errors.len() > 0 {
                println!("Errors:");
                for err in shunting_yard.errors {
                    println!("ERR: {}", err);
                }
            } else {
                println!("Lexer result: {}", shunting_yard.to_string_ast());
                println!("Shunting Yard result: {}", shunting_yard.to_string());
                println!("Equation equals: {}", shunting_yard.calculate());
            }
        },
        None => println!("Please supply an expression")
    };
}
