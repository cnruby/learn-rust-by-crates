[![Minimum rustc version](https://img.shields.io/badge/rustc-1.38+-brightgreen)](https://github.com/rust-lang/rust)
[![GitHub release](https://img.shields.io/github/v/release/cnruby/learn-rust-by-crates)](https://github.com/cnruby/learn-rust-by-crates/releases)
[![The Crate `mod_trait_exerci` Code](https://img.shields.io/badge/crate-code-yellowgreen)](https://github.com/cnruby/learn-rust-by-crates/tree/master/hello-borrowing)
[![Build Status on appveyor.com](https://img.shields.io/appveyor/ci/cnruby/learn-rust-by-crates?label=build%20on%20appveyor.com)](https://github.com/cnruby/learn-rust-by-crates/tree/master/hello-borrowing)
[![GitHub issues](https://img.shields.io/github/issues/cnruby/learn-rust-by-crates)](https://github.com/cnruby/learn-rust-by-crates/issues)
[![Twitter URL](https://img.shields.io/twitter/url?style=social&url=https%3A%2F%2Fmobile.twitter.com%2Fcnruby)](https://mobile.twitter.com/cnruby)

## Getting Started
- Learn the crate [hello_exercism](https://crates.io/crates/hello_exercism)

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

## Use the crate `borrowing_exerci`
- get the crate help
```bash
bw -h
```
- run a rust file `kw_let.rs` with borrowing error in the folder `examples`
```bash
bw --file kw_let --mode err | bat -l rs
# or
bw -f kw_let | bat -l rs
```
- run a rust file `kw_let.rs` without error in the folder `examples`
```bash
bw --file kw_let --mode ok | bat -l rs
# or
bw -f kw_let --mode ok | bat -l rs
```

### Resources
