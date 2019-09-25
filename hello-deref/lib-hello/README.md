# Crate
- crate name: deref_exerci
- folder name: lib-hello
- description: how to understand the rust feature deref

## I. develop the crate
### Step 1: create the default crate
```bash
mkdir lib-hello && cd lib-hello
cargo init --name deref_exerci --lib
touch README.md
vi README.md
```

### Step 2: develop the crate source and test codes
- Go to Crate Root Path
```bash
vi Cargo.toml
vi src/lib.rs
cargo fmt
cargo clippy
mkdir tests
touch tests/hello_array.rs
touch tests/hello_box.rs
touch tests/hello_other.rs
touch tests/hello_string.rs
touch tests/hello_vec.rs
vi tests/hello_array.rs
vi tests/hello_box.rs
vi tests/hello_other.rs
vi tests/hello_string.rs
vi tests/hello_vec.rs
cargo fmt
cargo clippy
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

# Rust Play
- [rust th feature deref](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=fdbb1441992501b578dc31a703bea044)