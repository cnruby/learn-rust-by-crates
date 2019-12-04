[![Minimum rustc version](https://img.shields.io/badge/rustc-1.38+-brightgreen)](https://github.com/rust-lang/rust)
[![GitHub release](https://img.shields.io/github/v/release/cnruby/learn-rust-by-crates)](https://github.com/cnruby/learn-rust-by-crates/releases)
[![The Crate `mod_trait_exerci` Code](https://img.shields.io/badge/crate-code-yellowgreen)](https://github.com/cnruby/learn-rust-by-crates/tree/master/hello-borrowing)
[![Build Status on appveyor.com](https://img.shields.io/appveyor/ci/cnruby/learn-rust-by-crates?label=build%20on%20appveyor.com)](https://github.com/cnruby/learn-rust-by-crates/tree/master/hello-borrowing)
[![GitHub issues](https://img.shields.io/github/issues/cnruby/learn-rust-by-crates)](https://github.com/cnruby/learn-rust-by-crates/issues)
[![Twitter URL](https://img.shields.io/twitter/url?style=social&url=https%3A%2F%2Fmobile.twitter.com%2Fcnruby)](https://mobile.twitter.com/cnruby)

## The Rust Feature Borrowing and Ownenship
<img src="https://github.com/cnruby/learn-rust-by-crates/blob/master/docs/zh-first-volumn/src/hello-borrowing/images/hello_borrowing-06-pointers.png?raw=true"/>

## Getting Started
- Learn the crate [hello_exercism](https://crates.io/crates/hello_exercism)
- install [cargo script](https://crates.io/crates/cargo-script)
- install [bat](https://crates.io/crates/bat)
```bash
cargo install cargo-script
cargo install bat
```

## Project
- name: hello-borrowing
- crate name: borrowing_exerci
- description: how to understand the rust borrowing

## Subproject: bin-hello
- folder name: bin-hello
- description: the crate 'borrowing_exerci'

## install the crate `borrowing_exerci`
```bash
cargo install borrowing_exerci
```

## Use the crate `borrowing_exerci` help
```bash
bw -h
```

## List all commands with features for code
```bash
bw -c <code>

# example:
bw -c closure_immut_string
```

## Run the code with a feature
- run a rust file with a feature
```bash
bw -c <code> -f <feature> | bat -l rs

# example "closure_immut_string" with a feature "ok":
bw -c closure_immut_string -f ok | bat -l rs
# tip: `f`, Forward  one window
# tip: `b`, Backward  one window
# tip: `q`, Exit.

# example "closure_immut_string" with a feature "err_01":
bw -c closure_immut_string -f err_01 | bat -l rs
# tip: `f`, Forward  one window
# tip: `b`, Backward  one window
# tip: `q`, Exit.
```

### Resources
