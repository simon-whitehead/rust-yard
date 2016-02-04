/*
 * Powers tests
 */

extern crate rustyard;

#[test]
fn can_raise_to_a_power() {
    let yard = rustyard::ShuntingYard::new("2 ^ 3");

    assert_eq!(8f64, yard.calculate().unwrap());
    assert_eq!("2 ^ 3 ", yard.to_string_ast());
    assert_eq!("2 3 ^ ", yard.to_string());
}
