# How to Create an Own Crate

## I. develop the crate
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
touch tests/u_hello.rs
vi tests/u_hello.rs
touch tests/i_hello.rs
vi tests/i_hello.rs
cargo test
```
### Step 3: develop the example codes
- Go to Crate Root Path
```bash
mkdir examples
touch examples/u_hello.rs
vi examples/u_hello.rs
cargo run --example u_hello
touch examples/i_hello.rs
vi examples/i_hello.rs
cargo run --example i_hello
```

## II. use the crate 'hello_exercism'
### Step 1: create the default Bin
```bash
mkdir bin-hello && cd bin-hello
# this is Bin Root Path
cargo init --name bin-hello --bin
```

### Step 2: configure the file Cargo.toml
- Go to Bin Root Path
```bash
echo 'hello_exercism = "0.3.6"' >> Cargo.toml
```
### Step 3: edit the rust file main.rs
- Go to Bin Root Path
```rust
// vi src/main.rs
use hello_exercism;

fn main () {
    println!("{}",hello_exercism::hello());
    assert_eq!("Hello, World!", hello_exercism::hello());
}
```
### Step 4: run the Bin program
- Go to Bin Root Path
```bash
cargo run main
```

## III. create the crate 'hello_exercism' doc in local version
- Go to Crate Root Path
```bash
cargo doc --open --package hello_exercism
```

## IV. create the crate 'hello_exercism' doc in server version
- github.com >> <REPOSITORY> >> Setting >> Options >> GitHub Pages >> ([INFO...](https://github.blog/2016-08-22-publish-your-project-documentation-with-github-pages/))
- Go to Crate Root Path
```bash
mkdir <REPOSITORY>/docs/<PROJECT_NAME>
cargo doc
cp -rf target/doc/.  <REPOSITORY>/docs/<PROJECT_NAME>/.
```
- Example:
- Go to Crate Root Path
```bash
mkdir -p ../../docs/hello-world
cargo doc
cp -rf target/doc/. ../../docs/hello-world/
```