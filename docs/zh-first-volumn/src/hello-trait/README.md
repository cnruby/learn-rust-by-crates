# 第二章 软件篋trait_exerci

### 学习内容
- 什么是Rust语言衔接关键词trait
- 为什么需要衔接关键词trait
- 怎么样实现Rust语言衔接关键词trait
- 实现关键词impl与衔接关键词trait是什么关系

### 篇目

1. [关键词trait概念表述之一](#关键词trait概念表述之一)
1. [关键词trait概念表述之二](#关键词trait概念表述之二)
1. [关键词trait概念表述之三](#关键词trait概念表述之三)
1. [解释关键词trait](#解释关键词trait)
1. [题外话](#题外话)
   1. [怎么样使用Rust语言nightly版本](#怎么样使用rust语言nightly版本)
   1. [参考资料](#参考资料)
1. [本章参考资料](#本章参考资料)

## 关键词trait概念表述之一

　　[关键词trait][id_01]是Rust语言的一项功能，可以告诉Rust编译器一种类型必须提供的功能。

## 关键词trait概念表述之二

　　[关键词trait][id_02]是为任何未知类型定义方法的集合。

## 关键词trait概念表述之三

　　[关键词trait][id_03]告诉Rust编译器一种特定的类型具有且可与其他类型共享的功效性质。

## 解释关键词trait

　　关键词trait提供了一种类型或者几种类型之间的衔接方式。它应该包含下面内容：

- 存在一种类型或者几种类型
- 使用关键词trait定义衔接名称
- 使用关键词trait代码块定义方法和函数
- 使用关键词"impl"和"for"组合，实现针对这一种类型或者这几种类型的方法和函数

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