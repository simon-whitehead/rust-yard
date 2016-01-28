//! Rustyard - Simon Whitehead, 2016.
//!
//! My first proper attempt at Rust code, Rustyard is
//! an implementation of the Shunting Yard algorithm and
//! can calculate the value of mathematical expressions
//! passed to it as strings.

pub mod token;
pub mod shunting_yard;

mod lexer;
mod rpn_calculator;
mod peekable_string_iterator;

// Exported types
pub use self::shunting_yard::ShuntingYard;
