# 关于Cargo工具基础命令

　　安装Rust语言软件篋存在两个行为，其目的和作用是不同的。使用软件工具rustup安装的软件篋是支持版本管理工具，而使用软件工具Cargo安装的软件篋是支持项目级开发环境。

## 编写规范格式代码工具 Rustfmt

### 安装Rustfmt命令

    rustup self update
    rustup component add rustfmt

### 使用Rustfmt命令

```bash
# 进来Cargo项目根目录
cargo fmt
```

## 编写有效代码工具 Clippy

### 安装Clippy命令

    rustup self update
    rustup component add clippy

### 使用Clippy命令

```bash
# 进来Cargo项目根目录
cargo clippy
```

## 说明Cargo软件篋开发命令

　　除了上面两个开发工具命令之外，Cargo项目还有自身命令：

### 测试代码运行命令

```bash
# -- 适用于所有Cargo项目和作业区 --
# 进来Cargo项目根目录
# 默认测试命令
# 说明：测试在目录tests下的所有测试文件
cargo test
```

### 运行应用程序命令

```bash
# -- 适用于所有Cargo项目 --
# 进来Cargo项目根目录
# 命令说明：
# cargo run --example <程序文件名称>
# 命令实例，如运行在目录examples下文件名称为hello.rs的应用程序
cargo run --example hello
```

### 运行应用程序命令

```bash
# -- 仅适用于Cargo应用程序项目 --
# 进来Cargo项目根目录
# 默认运行命令
# 说明：运行目录src下默认入口文件main.rs
cargo run
```

## 说明Cargo作业区开发命令

### 测试代码运行命令

```bash
# -- 适用于Cargo作业区和所有Cargo项目 --
# 进来作业区根目录
# 命令说明：
# cargo test --package <篋名称>
# 或者
# cargo test -p <篋名称>
# 命令实例，如运行在Cargo项目lib-hello下共享篋名称为hello_exercism
cargo test -p hello_exercism
```

### 运行应用程序命令

```bash
# -- 适用于Cargo作业区内Cargo应用程序项目 --
# 进来作业区根目录
# 命令说明：
# cargo run --package <篋名称>
# 或者
# cargo run -p <软件篋名称>
# 命令实例，如运行在Cargo项目bin-hello下篋名称为bin-hello
cargo run -p bin-hello
```

　　后续还将介绍上面Cargo工具的其他实用命令。