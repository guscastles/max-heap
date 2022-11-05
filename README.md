# Max/Min Heap Maker

An implementation of the max/min heap algorithm (aka, priority queue), written in Rust.

## Development

### Bench Tests

To run bench tests
1. Add this to the test file and function
```rust

extern crate test;
use test::Bencher;

#[bench]
fn function_to_test(bencher: &mut Bencher) {
...
}
```
2. Add this to the top of the main file, either `main.rs` or `lib.rs`
```rust
#![feature(test)]
```

Run with
```bash
cargo bench
```
### Running code coverage

#### Installing necessary packages

Create output folder

```bash
mkdir target/coverage/html -p
```

* grcov
```bash
cargo install grcov
```

* llvm-tools preview
```bash
rustup component add llvm-tools-preview
```
#### Creating the coverage files
Run `cargo test` with below options
```bash
CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' \
LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test
```
Then run `grcov` to generate HTML files

```bash
~/.cargo/bin/grcov . --binary-path ./target/debug/deps/ -s . \
-t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/html
```

Some cleanup is necessary afterwards. Remove the `*.profraw` files after generating the HTML files.
# Author

[Gus Garcia][1]

[1]: mailto:guscastles@gmail.com
