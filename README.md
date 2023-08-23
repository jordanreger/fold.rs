# `fold(1)` in Rust
This is the UNIX command `fold(1)` implemented in Rust.

## How to use
It is fairly straightforward, as it only takes two parameters:
```rs
fold(str: &'static str, len: Option<usize>) -> String {
  ...
}
```
The default fold width is 80, just like in the original command.

### Example
Here's an example of how to use `fold` in your code:

`src/main.rs`
```rs
use fold::fold;

fn main() {
    let lorem = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Morbi tristique senectus et netus. Id ornare arcu odio ut sem nulla pharetra diam. Commodo sed egestas egestas fringilla phasellus. Quis lectus nulla at volutpat diam ut venenatis tellus.";
    println!("{}", fold(lorem, None));
}
```

`Cargo.toml`
```toml
[package]
name = "fold-test"
version = "0.1.0"
edition = "2021"

[dependencies]
fold = { git = "https://git.sr.ht/~jordanreger/fold.rs" }
```
