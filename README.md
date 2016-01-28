# Rustyard

The [Shunting Yard algorithm](https://en.wikipedia.org/wiki/Shunting-yard_algorithm), implemented in Rust.

### The code

This is me experimenting with Rust (I love it so far). As such, the code quality may be questionable and not 100% idiomatic Rust.

### Examples

The implementation accepts the 4 basic mathematical operators and parenthesis. Some examples are below:

Basic addition:
```
Simon$ cargo run "2 + 4"
Input is: 2 + 4
Lexer result: 2 + 4 
Shunting Yard result: 2 4 + 
Equation equals: 6
```

Operator precedence:
```
Simon$ cargo run "2 + 4 * 3"
Input is: 2 + 4 * 3
Lexer result: 2 + 4 * 3 
Shunting Yard result: 2 4 3 * + 
Equation equals: 14
```

Parenthesis:
```
Simon$ cargo run "(2 + 4) * 3"
Input is: (2 + 4) * 3
Lexer result: ( 2 + 4 ) * 3 
Shunting Yard result: 2 4 + 3 * 
Equation equals: 18
```

### Licence

You're free to use this as you see fit. Some acknowledgement would be nice if you feel like doing so.
