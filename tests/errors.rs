/*
 * Error testing
 */

extern crate rustyard;

#[test]
fn can_detect_unbalanced_parenthesis() {
    let yard = rustyard::ShuntingYard::new("100 / 20 )");

    let err = &yard.calculate().unwrap_err()[0];

    assert_eq!("Unbalanced parenthesis", &err[..]);
}

#[test]
fn can_detect_unknown_identifiers() {
    let yard = rustyard::ShuntingYard::new("2a + 4");

    let err = &yard.calculate().unwrap_err()[0];

    assert_eq!("Unknown identifier: a", &err[..]);
}
