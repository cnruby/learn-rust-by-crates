[![Crates.io](https://img.shields.io/crates/v/trait_exerci?label=trait_exerci)](https://crates.io/crates/trait_exerci)
[![The Crate trait_exerci Code](https://img.shields.io/badge/hello--trait-code-yellowgreen)](https://github.com/cnruby/learn-rust-by-crates/tree/master/hello-trait)

# 软件篋trait_exerci

### 本章学习内容
- 什么是Rust语言衔接关键词trait
- 为什么需要衔接关键词trait
- 怎么样实现Rust语言衔接关键词trait
- 实现关键词impl与衔接关键词trait是什么关系

### 本节篇目

- [题外话](#题外话)
   - [怎么样使用Rust语言nightly版本](#怎么样使用rust语言nightly版本)
   - [参考资料](#参考资料)
- [本章参考资料](#本章参考资料)

## 题外话

### 怎么样使用Rust语言nightly版本
```bash
rustc --version
rustup default nightly
rustc --version
rustup default stable
rustup update
rustup show
```

### 参考资料
- [Setting "rustup default nightly" and back to stable ends up](https://github.com/rust-lang/rustup.rs/issues/451)
- [Toolchain 'nightly-x86_64-apple-darwin' missing](https://github.com/rust-lang/rust/issues/55571)

## 本章参考资料
- [Impls & Traits](https://learning-rust.github.io/docs/b5.impls_and_traits.html)
- [Traits from 'rust-by-example'](https://doc.rust-lang.org/stable/rust-by-example/trait.html)
- [Traits from 'The Rust Programming Language'](https://doc.rust-lang.org/1.8.0/book/traits.html)
- [Traits: Defining Shared Behavior](https://doc.rust-lang.org/beta/book/ch10-02-traits.html)

[id_01]:https://doc.rust-lang.org/1.8.0/book/traits.html
[id_02]:https://doc.rust-lang.org/stable/rust-by-example/trait.html
[id_03]:https://doc.rust-lang.org/beta/book/ch10-02-traits.html