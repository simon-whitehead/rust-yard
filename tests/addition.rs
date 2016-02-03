/*
 * Addition tests
 */

extern crate rustyard;

#[test]
fn can_add_two_numbers() {
    let yard = rustyard::ShuntingYard::new("2 + 2");

    assert_eq!(4f64, yard.calculate().unwrap());
    assert_eq!("2 + 2 ", yard.to_string_ast());
    assert_eq!("2 2 + ", yard.to_string());
}

#[test]
fn can_add_floating_point_numbers() {
    let yard = rustyard::ShuntingYard::new("2.5 + 2.5");

    assert_eq!(5f64, yard.calculate().unwrap());
    assert_eq!("2.5 + 2.5 ", yard.to_string_ast());
    assert_eq!("2.5 2.5 + ", yard.to_string());
}
