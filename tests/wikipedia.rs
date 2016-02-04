/*
 * Tests the results of the ShuntingYard examples on Wikipedia
 */

extern crate rustyard;

#[test]
fn wikipedia_one() {
    let yard = rustyard::ShuntingYard::new("3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3");

    assert_eq!("3 4 2 * 1 5 - 2 3 ^ ^ / + ", yard.to_string());
}
