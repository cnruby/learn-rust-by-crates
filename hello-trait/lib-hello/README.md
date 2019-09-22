

## I. develop the crate
### Step 1: create the default crate
```bash
mkdir lib-hello && cd lib-hello
cargo init --name trait_exercism --lib
```

### Step 2: develop the crate source and test codes
- Go to Crate Root Path
```bash
vi Cargo.toml
vi src/lib.rs
cargo fmt
cargo clippy # warning: struct is never constructed: `MyType`
mkdir tests
touch tests/hello.rs
vi tests/hello.rs
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