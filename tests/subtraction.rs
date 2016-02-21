/*
 * Subtraction tests
 */

extern crate rustyard;

#[test]
fn can_subtract_two_numbers() {
    let mut yard = rustyard::ShuntingYard::new();

    assert_eq!(3f64, yard.calculate("5 - 2").unwrap());
    assert_eq!("5 - 2 ", yard.to_string_ast());
    assert_eq!("5 2 - ", yard.to_string());
}

#[test]
fn can_subtract_floating_point_numbers() {
    let mut yard = rustyard::ShuntingYard::new();

    assert_eq!(2.5f64, yard.calculate("3.75 - 1.25").unwrap());
    assert_eq!("3.75 - 1.25 ", yard.to_string_ast());
    assert_eq!("3.75 1.25 - ", yard.to_string());
}
