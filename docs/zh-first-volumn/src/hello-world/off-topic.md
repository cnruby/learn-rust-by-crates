# 题外话

## Cargo工具命令

### 构建和运行软件篋目录examples下文件

```bash
# cargo build --example <目录examples下无扩展名的文件名称>
# cargo run --example <目录examples下无扩展名的文件名称>
cargo build --example hello
cargo run --example hello
```

### Cargo项目构建命令
```bash
cargo build
cargo build --release
```

```
── lib-hello
    ├── Cargo.lock
    ├── Cargo.toml
    ├── examples
    │   └── hello.rs
    ├── README.md
    ├── src
    │   └── lib.rs
    ├── target
    │   ├── debug
    │   └── package
    └── tests
        └── hello.rs
```

### Cargo项目测试特定代码命令
```bash
cargo test tests::it_works_hello_exercism
cargo test test_hello_world
```

## 项目重要配置和代码

## 思考问题
### 使用关键词use和extern有什么区别？
- 自Rust2018版本以来几乎不再需要extern语句。
- 关键词use方法与以前相同。
- 关键词use仅仅是引用标准符号的简写，或者使用trait时必须出现。

### 注解#[cfg(test)]有什么意义?
- 它告诉编译器在测试环境下进行编译，
- 仅当使用命令'cargo test'运行测试时，Cargo工具才会编译测试代码。

## 参考资料
- [whats-the-difference-between-use-and-extern](https://stackoverflow.com/questions/29403920/whats-the-difference-between-use-and-extern)
- [crates-and-modules](https://doc.rust-lang.org/1.0.0-alpha.2/book/crates-and-modules.html)
- [cfg-test-and-cargo-test-a-missing-information](https://freyskeyd.fr/cfg-test-and-cargo-test-a-missing-information/)
- [ch11-03-test-organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
- [writing-integration-tests-in-rust](https://klausi.github.io/rustnish/2017/05/25/writing-integration-tests-in-rust.html)
- 