/*
 * Multiplication tests
 */

extern crate rustyard;

#[test]
fn can_multiply_two_numbers() {
    let yard = rustyard::ShuntingYard::new("5 * 9");

    assert_eq!(45f64, yard.calculate().unwrap());
    assert_eq!("5 * 9 ", yard.to_string_ast());
    assert_eq!("5 9 * ", yard.to_string());
}

#[test]
fn can_multiply_floating_point_numbers() {
    let yard = rustyard::ShuntingYard::new("3.75 * 1.25");

    assert_eq!(4.6875f64, yard.calculate().unwrap());
    assert_eq!("3.75 * 1.25 ", yard.to_string_ast());
    assert_eq!("3.75 1.25 * ", yard.to_string());
}
