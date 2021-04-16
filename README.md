# grrs

## Learning Rust by Writing a Command Line App in 15 Minutes

<https://rust-cli.github.io/book/tutorial/index.html>

## Usage

### With Cargo

```sh
cargo run -- <pattern> <path>
```

### With binary

```sh
grrs <pattern> <path>
```

## Example

```sh
git clone https://github.com/koyashiro/grrs
cd grrs
cargo run -- main src/main.rs
```

```sh
$ cargo run -- main src/main.rs
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/grrs main src/main.rs`
fn main() -> Result<()> {
```
