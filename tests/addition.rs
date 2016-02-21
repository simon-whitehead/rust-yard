/*
 * Addition tests
 */

extern crate rustyard;

#[test]
fn can_add_two_numbers() {
    let mut yard = rustyard::ShuntingYard::new();

    assert_eq!(4f64, yard.calculate("2 + 2").unwrap());
    assert_eq!("2 + 2 ", yard.to_string_ast());
    assert_eq!("2 2 + ", yard.to_string());
}

#[test]
fn can_add_floating_point_numbers() {
    let mut yard = rustyard::ShuntingYard::new();

    assert_eq!(5f64, yard.calculate("2.5 + 2.5").unwrap());
    assert_eq!("2.5 + 2.5 ", yard.to_string_ast());
    assert_eq!("2.5 2.5 + ", yard.to_string());
}
