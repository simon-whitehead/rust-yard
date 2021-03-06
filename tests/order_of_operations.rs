/*
 * Order of operations tests
 */

extern crate rustyard;

#[test]
fn multiply_before_add() {
    let mut yard = rustyard::ShuntingYard::new();

    assert_eq!(14f64, yard.calculate("2 + 4 * 3").unwrap());
    assert_eq!("2 + 4 * 3 ", yard.to_string_ast());
    assert_eq!("2 4 3 * + ", yard.to_string());
}

#[test]
fn parenthesis_overrides_multiply_before_add() {
    let mut yard = rustyard::ShuntingYard::new();

    assert_eq!(18f64, yard.calculate("(2 + 4) * 3").unwrap());
    assert_eq!("( 2 + 4 ) * 3 ", yard.to_string_ast());
    assert_eq!("2 4 + 3 * ", yard.to_string());
}

#[test]
fn multiply_before_subtract() {
    let mut yard = rustyard::ShuntingYard::new();

    assert_eq!(-10f64, yard.calculate("2 - 4 * 3").unwrap());
    assert_eq!("2 - 4 * 3 ", yard.to_string_ast());
    assert_eq!("2 4 3 * - ", yard.to_string());
}

#[test]
fn parenthesis_overrides_multiply_before_subtract() {
    let mut yard = rustyard::ShuntingYard::new();

    assert_eq!(-6f64, yard.calculate("(2 - 4) * 3").unwrap());
    assert_eq!("( 2 - 4 ) * 3 ", yard.to_string_ast());
    assert_eq!("2 4 - 3 * ", yard.to_string());
}

#[test]
fn divide_before_add() {
    let mut yard = rustyard::ShuntingYard::new();

    assert_eq!(7f64, yard.calculate("2 + 20 / 4").unwrap());
    assert_eq!("2 + 20 / 4 ", yard.to_string_ast());
    assert_eq!("2 20 4 / + ", yard.to_string());
}

#[test]
fn parenthesis_overrides_divide_before_add() {
    let mut yard = rustyard::ShuntingYard::new();

    assert_eq!(6f64, yard.calculate("(4 + 20) / 4").unwrap());
    assert_eq!("( 4 + 20 ) / 4 ", yard.to_string_ast());
    assert_eq!("4 20 + 4 / ", yard.to_string());
}

#[test]
fn divide_before_subtract() {
    let mut yard = rustyard::ShuntingYard::new();

    assert_eq!(-3f64, yard.calculate("2 - 20 / 4").unwrap());
    assert_eq!("2 - 20 / 4 ", yard.to_string_ast());
    assert_eq!("2 20 4 / - ", yard.to_string());
}

#[test]
fn parenthesis_overrides_divide_before_subtract() {
    let mut yard = rustyard::ShuntingYard::new();

    assert_eq!(4f64, yard.calculate("(20 - 4) / 4").unwrap());
    assert_eq!("( 20 - 4 ) / 4 ", yard.to_string_ast());
    assert_eq!("20 4 - 4 / ", yard.to_string());
}

#[test]
fn powers_before_everything() {
    let mut yard = rustyard::ShuntingYard::new();

    assert_eq!(55f64, yard.calculate("1 + 2 * 3 ^ 3").unwrap());
    assert_eq!("1 + 2 * 3 ^ 3 ", yard.to_string_ast());
    assert_eq!("1 2 3 3 ^ * + ", yard.to_string());
}

#[test]
fn parenthesis_overrides_powers_before_everything() {
    let mut yard = rustyard::ShuntingYard::new();

    assert_eq!(217f64, yard.calculate("1 + (2 * 3) ^ 3").unwrap());
    assert_eq!("1 + ( 2 * 3 ) ^ 3 ", yard.to_string_ast());
    assert_eq!("1 2 3 * 3 ^ + ", yard.to_string());
}
