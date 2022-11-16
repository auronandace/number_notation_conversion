# number_notation_conversion
A 0 dependency simple tool for displaying strings of various number notations

# Motivation
I wanted to learn how to convert from one number notation to another as a string.
You can already very simply display a number in different notations in rust by specifying a formatter like so:
```rust
let number = my_string.parse::<u32>().unwrap(); // assume my_string is a valid number
println!("{:b}", number); // binary
println!("{:o}", number); // octal
println!("{:x}", number); // hexadecimal
```
This tool is therefore redundant if all you want to do is display a number in a different notation.
It does instead serve as a nice learning experience for how to do the conversion in a mostly string based way.

# Installation
First ensure you have Rust installed (best to use rustup).

Then:
```
git clone https://github.com/auronandace/number_notation_conversion
cd number_notation_conversion
cargo install --path .
```
