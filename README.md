# Rustyard ![alt text](https://travis-ci.org/simon-whitehead/rust-yard.svg?branch=master "master build status")

The [Shunting Yard algorithm](https://en.wikipedia.org/wiki/Shunting-yard_algorithm), implemented in Rust.

### The code

This is me experimenting with Rust (I love it so far). As such, the code quality may be questionable and not 100% idiomatic Rust.

### Using the code

You can instantiate a `ShuntingYard` instance with an expression and call its `calculate` method, which returns an `Result<f64, Vec<String>>`. An example would be:

```
extern crate rustyard;

fn main() {
    let yard = rustyard::ShuntingYard::new();

    // This prints "The result is: 14"
    println!("The result is: {}", yard.calculate("2 + 4 * 3").unwrap());
}
```

### Examples

The implementation accepts the 4 basic mathematical operators, the power operator and parenthesis. It also considers all input to be floating point. Some examples are below:

#### Basic addition:
```
Simon$ cargo run --example main "2 + 4"
Input is: 2 + 4
Lexer result: 2 + 4 
Shunting Yard result: 2 4 + 
Equation equals: 6
```

#### Floating point:
```
Simon$ cargo run --example main "1.75 * 2"
Input is: 1.75 * 2
Lexer result: 1.75 * 2 
Shunting Yard result: 1.75 2 * 
Equation equals: 3.5
```

#### Operator precedence:
```
Simon$ cargo run --example main "2 + 4 * 3"
Input is: 2 + 4 * 3
Lexer result: 2 + 4 * 3 
Shunting Yard result: 2 4 3 * + 
Equation equals: 14
```

#### Negative numbers:
```
Simon$ cargo run --example main "-2 * -2"
Input is: -2 * -2
Lexer result: -2 * -2
Shunting Yard result: -2 -2 *
Equation equals: 4
```

#### Powers:
```
Simon$ cargo run --example main "(3 + 5) ^ 2"
Input is: (3 + 5) ^ 2
Lexer result: ( 3 + 5 ) ^ 2 
Shunting Yard result: 3 5 + 2 ^ 
Equation equals: 64
```


#### Parenthesis:
```
Simon$ cargo run --example main "(2 + 4) * 3"
Input is: (2 + 4) * 3
Lexer result: ( 2 + 4 ) * 3 
Shunting Yard result: 2 4 + 3 * 
Equation equals: 18
```

#### Errors:
```
Simon$ cargo run --example main "4 / (2 + 3"
Input is: 4 / (2 + 3
Errors:
ERR: Unbalanced parenthesis
```

```
Simon$ cargo run --example main "a + b * c"
Input is: a + b * c
Errors:
ERR: Unknown identifier: a
ERR: Unknown identifier: b
ERR: Unknown identifier: c
```

### Licence

I've settled on an MIT licence for this repository.
