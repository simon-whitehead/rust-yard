extern crate rustyard;

fn main() {
    let mut _lexer = rustyard::Lexer::new("2.34 + 4 * 3");

    _lexer.lex();

    println!("{}", _lexer);

    if _lexer.errors.len() > 0 {
        for err in _lexer.errors {
            println!("{}", err);
        }
    }

}
