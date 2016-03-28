/*
 * Tests the results of the ShuntingYard examples on Wikipedia
 */

extern crate rustyard;

#[test]
fn wikipedia_one() {
    let mut yard = rustyard::ShuntingYard::new();

    assert_eq!(3.0001220703125f64, yard.calculate("3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3").unwrap());
    assert_eq!("3 4 2 * 1 5 - 2 3 ^ ^ / + ", yard.to_string());
}

#[test]
fn wikipedia_two() {
    let mut yard = rustyard::ShuntingYard::new();

    yard.calculate("sin(max(2, 3) / 3 * 3.1415)").unwrap();

    assert_eq!("2 3 max 3 / 3.1415 * sin ", yard.to_string());
}
