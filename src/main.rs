extern crate rustyard;

fn main() {
    let shunting_yard = rustyard::ShuntingYard::new("2 + 4 * 3");

    println!("{}", shunting_yard.to_string());
}
