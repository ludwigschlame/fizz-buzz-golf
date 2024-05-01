# FizzBuzz Golf

This is a simple implementation of the FizzBuzz problem in Rust with the goal of being as short as possible. More precisely, the implementation should fit into a single line of code when formatted with rustfmt in its default configuration (≤ 100 columns).

The implementation uses some tricks to fit the function into a single line of code:

- First, instead of branching, it indexes into an array of strings to determine the output.
- Additionally, instead of comparing the modulo to zero, it uses the min() function to determine the correct index:
- `1.min(n % 3)` $\iff$ `(n % 3 != 0) as usize`.
- `2.min(n % 5 * 2)` $\iff$ `2 * (n % 5 != 0) as usize`.

## Result

```rust
pub fn fizz_buzz(n: usize) -> String {
    ["fizzbuzz", "buzz", "fizz", &n.to_string()][1.min(n % 3) + 2.min(n % 5 * 2)].to_owned()
^        ^         ^         ^         ^         ^         ^         ^         ^         ^ ^
1       10        20        30        40        50        60        70        80        90 92
}
```

This implementation goes up to column 92 (out of 100).
Would be interesting to see if it is possible to make it even shorter.
