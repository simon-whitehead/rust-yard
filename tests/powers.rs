/*
 * Powers tests
 */

extern crate rustyard;

#[test]
fn can_raise_to_a_power() {
    let mut yard = rustyard::ShuntingYard::new();

    assert_eq!(8f64, yard.calculate("2 ^ 3").unwrap());
    assert_eq!("2 ^ 3 ", yard.to_string_ast());
    assert_eq!("2 3 ^ ", yard.to_string());
}
