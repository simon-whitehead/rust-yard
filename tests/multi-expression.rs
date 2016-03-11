/*
 * Multiple expressions - make sure a single ShuntingYard
 * instance can calculate more than one expression.
 */

extern crate rustyard;

#[test]
fn can_calculate_twice_with_single_instance() {
    let mut yard = rustyard::ShuntingYard::new();

    assert_eq!(4f64, yard.calculate("2 + 2").unwrap());
    assert_eq!("2 + 2 ", yard.to_string_ast());
    assert_eq!("2 2 + ", yard.to_string());

    assert_eq!(-4f64, yard.calculate("2 * -2").unwrap());
    assert_eq!("2 * -2 ", yard.to_string_ast());
    assert_eq!("2 -2 * ", yard.to_string());
}
