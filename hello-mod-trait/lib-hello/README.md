# Crate
- crate name: mod_trait_exerci
- folder name: lib-hello
- description: how to understand the rust feature trait with mod

## I. develop the crate
### Step 1: create the default crate
```bash
mkdir lib-hello && cd lib-hello
cargo init --name mod_trait_exerci --lib
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
touch tests/mod_hello.rs
vi tests/mod_hello.rs
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