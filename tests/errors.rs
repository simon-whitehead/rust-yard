/*
 * Error testing
 */

extern crate rustyard;

#[test]
fn can_detect_unbalanced_parenthesis() {
    let mut yard = rustyard::ShuntingYard::new();

    let err = &yard.calculate("100 / 20 )").unwrap_err()[0];

    assert_eq!("Unbalanced parenthesis", &err[..]);
}

#[test]
fn can_detect_unknown_identifiers() {
    let mut yard = rustyard::ShuntingYard::new();

    let err = &yard.calculate("2a + 4").unwrap_err()[0];

    assert_eq!("Unknown identifier: a", &err[..]);
}
