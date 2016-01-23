extern crate rustyard;

fn main() {
    let shunting_yard = rustyard::ShuntingYard::new("2 + 4 * 3");

    println!("Input is: 2 + 4 * 3");
    println!("Shunting Yard result: {}", shunting_yard.to_string());

    println!("Equation equals: {}", shunting_yard.calculate());
}
