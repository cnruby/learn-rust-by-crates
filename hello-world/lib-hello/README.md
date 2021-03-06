[![Minimum rustc version](https://img.shields.io/badge/rustc-1.38+-brightgreen)](https://github.com/rust-lang/rust)
[![The Crate `hello_exercism` Code](https://img.shields.io/badge/crate-code-yellowgreen)](https://github.com/cnruby/learn-rust-by-crates/tree/master/hello-world)
[![Build Status on appveyor.com](https://img.shields.io/appveyor/ci/cnruby/learn-rust-by-crates?label=build%20on%20appveyor.com)](https://github.com/cnruby/learn-rust-by-crates/tree/master/hello-world)
[![GitHub issues](https://img.shields.io/github/issues/cnruby/learn-rust-by-crates)](https://github.com/cnruby/learn-rust-by-crates/issues)
[![Twitter URL](https://img.shields.io/twitter/url?style=social&url=https%3A%2F%2Fmobile.twitter.com%2Fcnruby)](https://mobile.twitter.com/cnruby)

## The Rust Language Tool: [Cargo](https://doc.rust-lang.org/cargo/index.html)
<img src="https://github.com/cnruby/learn-rust-by-crates/raw/master/hello-world/lib-hello/images/hello_exercism_01.png"/>

## About this Crate `hello_exercism`
- How to Create a Workspace
- How to Create and Publish an Own Crate
- How to Use an Own Crate
- How te develop the unit tests with the folder 'tests'
- How te develop the unit tests for private codes
- How te develop the integration tests with the folder 'tests'
- How te develop the integration tests with the folder 'src'

## Table of Contents

- [I. create a workspace](#i-create-a-workspace)
- [II. develop the crate](#ii-develop-the-crate)
  - [1. create the default crate](#1-create-the-default-crate)
  - [2. develop the crate source and test codes for folder 'tests'](#2-develop-the-crate-source-and-test-codes-for-folder-tests)
  - [3. develop the example codes](#3-develop-the-example-codes)
  - [4. develop the crate source and test codes for folder 'src'](#4-develop-the-crate-source-and-test-codes-for-folder-src)
  - [5. publish the own crate to crates.io](#5-publish-the-own-crate-to-cratesio)
- [III. use the crate](#iii-use-the-crate)
  - [1. create the default Bin](#1-create-the-default-bin)
  - [2. configure the file Cargo.toml](#2-configure-the-file-cargotoml)
  - [3. edit the rust file main.rs](#3-edit-the-rust-file-mainrs)
  - [4. run the Bin program](#4-run-the-bin-program)
- [IV. create the crate doc in local version](#iv-create-the-crate-doc-in-local-version)
- [V. create the crate doc in server version](#v-create-the-crate-doc-in-server-version)

# I. create a workspace

```bash
# create a workspaces
mkdir workpsaces && cd workpsaces
# create a workspace 'hello-world'
mkdir hello-world && cd hello-world
# create a configration for the workspace
touch Cargo.toml
# four crates for this workspace
# the follow lines is ONE command
echo '[workspace]
members = ["lib-hello", "bin-hello", "bin-local-hello", "lib-extern"]' >> Cargo.toml
```

# II. develop the crate
## 1. create the default crate
```bash
mkdir lib-hello && cd lib-hello
# this is Crate Root Path
cargo init --name hello_exercism --lib
```
## 2. develop the crate source and test codes for folder 'tests'
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
## 3. develop the example codes
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
## 4. develop the crate source and test codes for folder 'src'
```bash
vi src/lib.rs
mkdir -p src/integration_tests
touch src/integration_tests/mod.rs
vi src/integration_tests/mod.rs
touch src/integration_tests/i_hello.rs
vi src/integration_tests/i_hello.rs
mkdir -p src/private_tests
touch src/private_tests/mod.rs
vi src/private_tests/mod.rs
touch src/private_tests/owned_hello.rs
vi src/private_tests/owned_hello.rs
cargo test
```
## 5. publish the own crate to crates.io
- register the [crates.io](https://crates.io) with [github.com](https://github.com/) account
- login the [crates.io](https://crates.io)
- get the [token](https://crates.io/me)
- run the follow command in computer

```bash
## run the follow commant at A time
cargo login <token>
## can repeat the follow commands
cargo test
## commit and push the codes in github.com 
cargo package
cargo publish
## change the release github.com
```
# III. use the crate
## 1. create the default Bin
```bash
mkdir bin-hello && cd bin-hello
# this is Bin Root Path
cargo init --name bin-hello --bin
```
## 2. configure the file Cargo.toml
- Go to Bin Root Path
```bash
echo 'hello_exercism = "0.4.0"' >> Cargo.toml
```
## 3. edit the rust file main.rs
- Go to Bin Root Path
```rust
// vi src/main.rs
use hello_exercism;

fn main () {
    println!("{}",hello_exercism::hello());
    assert_eq!("Hello, World!", hello_exercism::hello());
}
```
## 4. run the Bin program
- Go to Bin Root Path
```bash
cargo run main
```

# IV. create the crate doc in local version
- Go to Crate Root Path
```bash
cargo doc --open --package hello_exercism
```

# V. create the crate doc in server version
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