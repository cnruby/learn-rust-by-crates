# hello_exercism

## develop the crate
### Step 1: create the default crate
```bash
mkdir lib-hello && cd lib-hello
# this is Crate Root Path
cargo init --name hello_exercism --lib
```
### Step 2: develop the crate source and test codes
- Go to Crate Root Path
```bash
vi Cargo.toml
vi src/lib.rs
mkdir tests
touch tests/hello.rs
vi tests/hello.rs
cargo test
```
### Step 3: develop the example codes
- Go to Crate Root Path
```bash
mkdir examples
touch examples/hello.rs
vi examples/hello.rs
cargo run --example hello
```

## use the crate 'hello_exercism'
### Step 1: configure the file Cargo.toml
- Go to Bin Root Path
```bash
echo 'hello_exercism = "0.1.6"' >> Cargo.toml
```
### Step 2: edit the rust file main.rs
- Go to Bin Root Path
```rust
// vi src/main.rs
use hello_exercism;

fn main () {
    println!("{}",hello_exercism::hello());
    assert_eq!("Hello, World!", hello_exercism::hello());
}
```
### Step 3: run the Bin program
- Go to Bin Root Path
```bash
cargo run main
```