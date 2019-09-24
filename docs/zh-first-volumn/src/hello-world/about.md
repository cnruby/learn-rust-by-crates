# 关于软件篋项目hello-world

## 项目名称

- 整个项目名称：hello-world
- 共享软件篋项目名称：lib-hello
- 本地程序项目名称：bin-local-hello
- 仓库程序项目名称：bin-hello
- 共享软件篋名称：hello_exercism


## 项目结构

```
── hello-world
    ├── README.md
    ├── bin-hello
    │   ├── Cargo.lock
    │   ├── Cargo.toml
    │   └── src
    │       └── main.rs
    ├── bin-local-hello
    │   ├── Cargo.lock
    │   ├── Cargo.toml
    │   ├── src
    │   │   └── main.rs
    │   └── tests
    │       └── hello.rs
    └── lib-hello
        ├── Cargo.lock
        ├── Cargo.toml
        ├── README.md
        ├── examples
        │   └── hello.rs
        ├── src
        │   └── lib.rs
        └── tests
            └── hello.rs
```

## 项目重要配置和代码

## 思考问题
### 使用关键词use和extern有什么区别？
- 自Rust2018版本以来几乎不再需要extern语句。
- 关键词use方法与以前相同。
- 关键词use仅仅是引用标准符号的简写。

## 参考资料
- [whats-the-difference-between-use-and-extern](https://stackoverflow.com/questions/29403920/whats-the-difference-between-use-and-extern)
- [crates-and-modules](https://doc.rust-lang.org/1.0.0-alpha.2/book/crates-and-modules.html)