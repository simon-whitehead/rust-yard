# Rustyard

The [Shunting Yard algorithm](https://en.wikipedia.org/wiki/Shunting-yard_algorithm), implemented in Rust.

### The code

This is me experimenting with Rust (I love it so far). As such, the code quality may be questionable and not 100% idiomatic Rust.

### Examples

The implementation accepts the 4 basic mathematical operators and parenthesis. It also considers all input to be floating point. Some examples are below:

Basic addition:
```
Simon$ cargo run "2 + 4"
Input is: 2 + 4
Lexer result: 2 + 4 
Shunting Yard result: 2 4 + 
Equation equals: 6
```

Floating point:
```
Simon$ cargo run "1.75 * 2"
Input is: 1.75 * 2
Lexer result: 1.75 * 2 
Shunting Yard result: 1.75 2 * 
Equation equals: 3.5
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

Errors:
```
Simon$ cargo run "4 / (2 + 3"
Input is: 4 / (2 + 3
Errors:
ERR: Unbalanced parenthesis
```

```
Simon$ cargo run "a + b * c"
Input is: a + b * c
Errors:
ERR: Unknown identifier: a
ERR: Unknown identifier: b
ERR: Unknown identifier: c
```

### Licence

You're free to use this as you see fit. Some acknowledgement would be nice if you feel like doing so.
