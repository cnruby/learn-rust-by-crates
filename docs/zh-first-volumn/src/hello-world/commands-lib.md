共享篋：开发命令
======================

## 学习内容
- 熟悉和使用Cargo工具命令
- 熟悉和使用共享篋项目开发命令

## 篇目

1. [创建共享篋项目命令](#创建共享篋项目命令)
1. [开发共享篋和单元测试代码](#开发共享篋和单元测试代码)
1. [开发共享篋的单元实例代码](#开发共享篋的单元实例代码)
1. [开发共享篋的单元实例代码](#开发共享篋的单元实例代码)
1. [开发共享篋和集成测试代码](#开发共享篋和集成测试代码)
1. [执行共享篋和测试代码](#执行共享篋和测试代码)
1. [执行共享篋的实例代码](#执行共享篋的实例代码)
1. [开发共享篋文档](#开发共享篋文档)
1. [发布共享篋准备工作](#发布共享篋准备工作)
1. [发布共享篋](#发布共享篋)

## 创建共享篋项目命令
```bash
# 创建共享篋项目命令
# 先进入作业区根目录，且创建项目目录，然后进入共享篋根目录
mkdir lib-hello && cd lib-hello
# 创建名称为hello_exercism的共享篋
cargo init --name hello_exercism --lib
```

## 开发共享篋和单元测试代码
```bash
# 开发共享篋和单元测试代码
vi Cargo.toml
vi src/lib.rs
mkdir tests
touch tests/u_hello.rs
vi tests/u_hello.rs
touch tests/i_hello.rs
vi tests/i_hello.rs
```

## 开发共享篋的单元实例代码
```bash
# 开发共享篋的单元实例代码
mkdir examples
touch examples/u_hello.rs
vi examples/u_hello.rs
touch examples/i_hello.rs
vi examples/i_hello.rs
```

## 开发共享篋的单元实例代码
```bash
# 开发共享篋的单元实例代码
mkdir examples
touch examples/u_hello.rs
vi examples/u_hello.rs
```

## 开发共享篋和集成测试代码
```bash
# 开发共享篋和集成测试代码
echo 'i_crate = { version = "0.1.1", package = "hello_extern"}' >> Cargo.toml
touch tests/i_hello.rs
vi tests/i_hello.rs
touch examples/i_hello.rs
vi examples/i_hello.rs
```

## 执行共享篋和测试代码
```bash
# 执行共享篋和测试代码
# 这些命令需要重复运行
cargo fmt
cargo clippy
cargo test
```

## 执行共享篋的实例代码
```bash
# 执行共享篋的实例代码
# 这些命令需要重复运行
cargo fmt
cargo clippy
cargo run --example u_hello
cargo run --example i_hello
```

## 开发共享篋文档
```bash
# 开发共享篋文档
mkdir -p ../../docs/hello-world
cargo doc
cp -rf ../target/doc/. ../../docs/hello-world/.
```

## 发布共享篋准备工作

```bash
# 发布共享篋准备工作
# 注册网站crates.io帐号
# 登录网站crates.io
# 从网站crates.io获取token，如下所示
# 在本地电脑运行下面命令
cargo login <token>
```
![image](../../images/crates_io_api_access_new_token.png)
![image](../../images/crates_io_api_access_create.png)

## 发布共享篋
```bash
## 提交代码
cargo package
cargo publish
```