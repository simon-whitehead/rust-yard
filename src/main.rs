extern crate rustyard;

use std::env;
use std::slice;

fn main() {
    let mut args = env::args().collect::<Vec<String>>();

    if args.len() > 1 {
        args.remove(0);
        let input = join_args(args);
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
    } else {
        println!("Please supply an expression");
    }
}

fn join_args(args: Vec<String>) -> String {
    let mut result = String::new();

    for arg in args {
        result.push_str(&arg[..]);
        result.push_str(" ");
    }

    result
}
