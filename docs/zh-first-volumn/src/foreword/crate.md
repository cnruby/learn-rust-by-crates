# 关于软件篋（Crate）和应用程序

## 创建默认软件篋程序命令

```bash
# 进入整体项目根目录
# 命令说明：
# mkdir <crate-project-name>
# 命令实例，如创建名称为lib-hello的软件篋程序项目目录
mkdir lib-hello
# 进入软件篋程序根目录
cd lib-hello
# 命令说明：
# cargo init --name <crate_name> --lib
# 命令实例，如创建名称为hello_exercism的软件篋程序
cargo init --name hello_exercism --lib
```

## 创建默认应用程序命令

```bash
# 进入整体项目根目录
# 命令说明：
# mkdir <app-project-name>
# 命令实例，如创建名称为bin-hello的应用程序项目目录
mkdir bin-hello
# 进入应用程序根目录
cd bin-hello
# 命令说明
# cargo init --name <app-name> --bin
# 命令实例，如创建名称为bin-hello的应用程序
cargo init --name bin-hello --bin
```

## 说明软件篋结构

　　使用软件工具Cargo，在默认Cargo项目基础上，这里除了增加了默认说明文件README.md外，还有增加了两个Cargo默认目录：tests和examples，同时在两个目录下增加了两个rust程序文件，其结果如下：

```
── lib-hello
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

　　在上面的结构里，除了两个文件hello.rs之外，其他都是Cargo项目的默认目录和文件。这些目录和文件都是与Cargo工具默认命令相关的。Cargo项目还有其他默认目录和文件。目录src下的默认文件lib.rs是软件篋程序的入口文件。

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

## 说明Cargo项目开发命令

　　除了上面两个开发工具命令之外，Cargo项目还有自身命令：

### 测试代码运行命令

```bash
# -- 适用于所有Cargo项目 --
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

