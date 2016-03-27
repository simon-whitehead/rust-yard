/*
 * Functions
 */

extern crate rustyard;

#[test]
fn can_call_function_before() {
    let mut yard = rustyard::ShuntingYard::new();

    assert_eq!(6.0, yard.calculate("max(1, 2) + 4").unwrap());
    assert_eq!("max ( 1 , 2 ) + 4 ", yard.to_string_ast());
    assert_eq!("1 2 max 4 + ", yard.to_string());
}

#[test]
fn can_call_function_after() {
    let mut yard = rustyard::ShuntingYard::new();

    assert_eq!(8.0, yard.calculate("4 + min(5, 4)").unwrap());
    assert_eq!("4 + min ( 5 , 4 ) ", yard.to_string_ast());
    assert_eq!("4 5 4 min + ", yard.to_string());
}

#[test]
fn can_call_nested_functions() {
    let mut yard = rustyard::ShuntingYard::new();

    assert_eq!(29.0, yard.calculate("7 + max(2, min(47.94, trunc(22.54)))").unwrap());
    assert_eq!("7 + max ( 2 , min ( 47.94 , trunc ( 22.54 ) ) ) ", yard.to_string_ast());
    assert_eq!("7 2 47.94 22.54 trunc min max + ", yard.to_string());
}
