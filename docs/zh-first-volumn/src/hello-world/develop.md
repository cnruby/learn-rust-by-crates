# 子项目：共享软件篋hello_exercism

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

## 说明共享篋代码

### 项目配置文件: ./Cargo.toml



<span class="filename">项目配置文件: ./Cargo.toml</span>
```toml
{{#include ../../../../hello-world/lib-hello/Cargo.toml}}
```

### 程序文件lib.rs
- 默认模块名称为hello_exercism
- 函数hello()意义上类似于关键词let

<span class="filename">Rust文件: src/lib.rs</span>
{{#playpen ../../../../hello-world/lib-hello/src/lib.rs editable}}

<span class="filename">Rust文件: examples/main.rs</span>
{{#playpen ../../../../hello-world/lib-hello/examples/main.rs}}

### 测试文件hello.rs

<span class="filename">Rust文件: tests/hello.rs</span>
{{#playpen ../../../../hello-world/lib-hello/tests/hello.rs editable}}

### 实例文件hello.rs

<span class="filename">Rust文件: examples/hello.rs</span>
{{#playpen ../../../../hello-world/lib-hello/examples/hello.rs editable}}

