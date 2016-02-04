/*
 * Tests for negative numbers.
 */

extern crate rustyard;

#[test]
fn can_add_two_negative_numbers() {
    let yard = rustyard::ShuntingYard::new("-2 + -2");

    assert_eq!(-4f64, yard.calculate().unwrap());
    assert_eq!("-2 + -2 ", yard.to_string_ast());
    assert_eq!("-2 -2 + ", yard.to_string());
}

#[test]
fn can_subtract_two_negative_numbers() {
    let yard = rustyard::ShuntingYard::new("-2 - -2");

    assert_eq!(0f64, yard.calculate().unwrap());
    assert_eq!("-2 - -2 ", yard.to_string_ast());
    assert_eq!("-2 -2 - ", yard.to_string());
}

#[test]
fn can_multiply_two_negative_numbers() {
    let yard = rustyard::ShuntingYard::new("-2 * -2");

    assert_eq!(4f64, yard.calculate().unwrap());
    assert_eq!("-2 * -2 ", yard.to_string_ast());
    assert_eq!("-2 -2 * ", yard.to_string());
}

#[test]
fn can_divide_two_negative_numbers() {
    let yard = rustyard::ShuntingYard::new("-20 / -2");

    assert_eq!(10f64, yard.calculate().unwrap());
    assert_eq!("-20 / -2 ", yard.to_string_ast());
    assert_eq!("-20 -2 / ", yard.to_string());
}
