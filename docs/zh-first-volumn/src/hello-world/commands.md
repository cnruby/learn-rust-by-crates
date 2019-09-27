# 开发软件篋命令

## 学习内容
- 熟悉和使用共享篋项目命令
- 熟悉和使用本地程序项目命令
- 熟悉和使用仓库程序项目命令
- 了解项目软件篋开发过程

## 开发共享篋项目命令

　　只有发布了自己共享软件篋以后，才能开发这个项目的集成测试代码和集成实例。

### 创建项目命令
```bash
# 先进入作业区根目录，且创建项目目录，然后进入共享篋根目录
mkdir lib-hello && cd lib-hello
# 创建名称为hello_exercism的共享篋
cargo init --name hello_exercism --lib
```

### 开发共享篋和单元测试代码
```bash
vi Cargo.toml
vi src/lib.rs
mkdir tests
touch tests/u_hello.rs
vi tests/u_hello.rs
touch tests/i_hello.rs
vi tests/i_hello.rs
```

### 开发共享篋的单元实例代码
```bash
mkdir examples
touch examples/u_hello.rs
vi examples/u_hello.rs
touch examples/i_hello.rs
vi examples/i_hello.rs
```

### 开发共享篋的单元实例代码
```bash
mkdir examples
touch examples/u_hello.rs
vi examples/u_hello.rs
```

### 开发共享篋和集成测试代码
```bash
echo 'i_crate = { version = "0.3.6", package = "hello_exercism"}' >> Cargo.toml
touch tests/i_hello.rs
vi tests/i_hello.rs
touch examples/i_hello.rs
vi examples/i_hello.rs
```

### 执行共享篋和测试代码
```bash
# 这些命令需要重复运行
cargo fmt
cargo clippy
cargo test
```

### 执行共享篋的实例代码
```bash
# 这些命令需要重复运行
cargo fmt
cargo clippy
cargo run --example u_hello
cargo run --example i_hello
```

## 开发本地程序项目命令

### 创建项目命令
```bash
# 先进入作业区根目录，且创建项目目录，然后进入本地程序项目根目录
mkdir bin-local-hello && cd bin-local-hello
# 创建名称为bin-hello的可执行软件篋
cargo init --name bin-local-hello --bin
```

### 配置项目文件Cargo.toml
```bash
# 进入本地程序项目根目录
echo 'hello_exercism = { path = "../lib-hello"}' >> Cargo.toml
```

### 开发Rust程序文件main.rs代码
```bash
# 进入本地程序项目根目录
vi src/main.rs
```

### 运行Rust程序文件main.rs
```bash
# 进入本地程序项目根目录
cargo run
# 或者
cargo run main
```

## 开发仓库程序项目命令

　　只有发布了自己共享软件篋以后，才能开发这个项目。

### 创建项目命令
```bash
# 先进入作业区根目录，且创建项目目录，然后进入本地程序项目根目录
mkdir bin-hello && cd bin-hello
# 创建名称为bin-hello的可执行软件篋
cargo init --name bin-hello --bin
```

### 配置项目文件Cargo.toml
```bash
# 进入本地程序项目根目录
echo 'hello_exercism = "0.3.7"' >> Cargo.toml
```

### 开发Rust程序文件main.rs代码
```bash
# 进入本地程序项目根目录
vi src/main.rs
```

### 运行Rust程序文件main.rs
```bash
# 进入本地程序项目根目录
cargo run
# 或者
cargo run main
```