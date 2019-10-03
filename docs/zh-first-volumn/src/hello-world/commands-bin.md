# 仓库程序：开发命令

　　只有发布了自己共享软件篋以后，才能开发这个项目。

## 学习内容
- 熟悉和使用Cargo工具命令
- 熟悉和使用仓库程序项目开发命令

## 篇目

1. [创建项目命令](#创建项目命令)
1. [修改项目配置文件](#修改项目配置文件)
1. [开发主程序文件代码](#开发主程序文件代码)
1. [运行主程序文件](#运行主程序文件)
1. [安装仓库程序于本地系统](#安装仓库程序于本地系统)
1. [运行安装于本地系统的仓库程序](#运行安装于本地系统的仓库程序)
1. [删除本地系统的仓库程序](#删除本地系统的仓库程序)

## 创建项目命令
```bash
# 创建项目命令
# 先进入作业区根目录，且创建项目目录，然后进入仓库程序项目根目录
mkdir bin-hello && cd bin-hello
# 创建名称为bin-hello的可执行软件篋
cargo init --name bin-hello --bin
```

## 修改项目配置文件
```bash
# 修改项目配置文件
# 进入仓库程序项目根目录
echo 'hello_exercism = "0.4.1"' >> Cargo.toml
```

## 开发主程序文件代码
```bash
# 开发主程序文件代码
# 进入仓库程序项目根目录
rm src/main.rs
mkdir -p src/bin
touch src/bin/hello.rs
vi src/bin/hello.rs
touch src/bin/hallo.rs
vi src/bin/hallo.rs
```

## 运行主程序文件
```bash
# 运行主程序文件
# 进入仓库程序项目根目录
cargo run --bin hello
cargo run --bin hallo
```

## 安装仓库程序于本地系统
```bash
# 安装仓库程序于本地系统
# 进入仓库程序项目根目录
# 所有Cargo软件篋都安装于目录~/.cargo/bin/
ls ~/.cargo/bin/
cargo install --path .
ls ~/.cargo/bin/
```

## 运行安装于本地系统的仓库程序
```bash
# 运行安装于本地系统的仓库程序
# 可以在本地系统任何目录下运行下面命令
hello
hallo
```

## 删除本地系统的仓库程序
```bash
# 删除本地系统的仓库程序
# 进入仓库程序项目根目录
ls ~/.cargo/bin/
# cargo uninstall <来自于Cargo.toml里的仓库程序名称>
cargo uninstall bin-hello
ls ~/.cargo/bin/
```