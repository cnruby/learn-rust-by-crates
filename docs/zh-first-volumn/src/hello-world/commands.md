# 开发软件篋命令

## 开发共享篋项目命令

### 创建项目命令
```bash
# 先进入整体项目根目录，且创建项目目录，然后进入共享篋根目录
mkdir lib-hello && cd lib-hello
# 创建名称为hello_exercism的共享篋
cargo init --name hello_exercism --lib
```

### 开发共享篋和测试代码
```bash
vi Cargo.toml
vi src/lib.rs
mkdir tests
touch tests/hello.rs
vi tests/hello.rs
```

### 执行共享篋和测试代码
```bash
cargo fmt
cargo clippy
cargo test
```

### 开发共享篋的实例代码
```bash
mkdir examples
touch examples/hello.rs
vi examples/hello.rs
```

### 执行共享篋的实例代码
```bash
cargo fmt
cargo clippy
cargo run --example hello
```

## 开发本地程序项目命令


## 开发仓库程序项目命令