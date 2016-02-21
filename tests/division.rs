/*
 * Division tests
 */

extern crate rustyard;

#[test]
fn can_divide_two_numbers() {
    let mut yard = rustyard::ShuntingYard::new();

    assert_eq!(5f64, yard.calculate("100 / 20").unwrap());
    assert_eq!("100 / 20 ", yard.to_string_ast());
    assert_eq!("100 20 / ", yard.to_string());
}

#[test]
fn can_divide_floating_point_numbers() {
    let mut yard = rustyard::ShuntingYard::new();

    assert_eq!(7.6f64, yard.calculate("9.5 / 1.25").unwrap());
    assert_eq!("9.5 / 1.25 ", yard.to_string_ast());
    assert_eq!("9.5 1.25 / ", yard.to_string());
}
