# 关于作业区和软件篋（Crate）实例

　　软件篋（Crate）是其他语言的库（library）或包（package）的同义词。软件篋可以生成这里称之为应用程序的可执行文件或共享库，

## 实例：创建作业区空间

```bash
# 创建一个工作空间
mkdir workpsaces && cd workpsaces
```

## 实例：创建作业区

```bash
# 创建一个作业区hello-world
mkdir hello-world && cd hello-world
# 创建一个作业区配置文件
touch Cargo.toml
# 作业区存在四个软件篋
# 注意：下面两行代码是一行命令
echo '[workspace]
members = ["lib-hello", "bin-hello", "bin-local-hello", "lib-extern"]' >> Cargo.toml
```

## 实例：创建默认共享软件篋程序命令

```bash
# 进入作业区根目录
# 命令说明：
# mkdir <crate-project-name>
# 命令实例，如创建名称为lib-hello的共享篋程序项目目录
mkdir lib-hello
# 进入软件篋程序根目录
cd lib-hello
# 命令说明：
# cargo init --name <crate_name> --lib
# 命令实例，如创建名称为hello_exercism的软件篋程序
cargo init --name hello_exercism --lib
```

## 实例：创建默认可执行的应用程序命令

```bash
# 进入作业区根目录
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

## 实例：说明共享软件篋结构

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

　　在上面的结构里，除了两个文件hello.rs之外，其他都是Cargo项目的默认目录和文件。这些目录和文件都是与Cargo工具默认命令相关的。Cargo项目还有其他默认目录和文件。目录src下的默认文件lib.rs是共享篋的入口文件。

## 参考资料
- [a4.cargo,crates_and_basic_project_structure](https://learning-rust.github.io/docs/a4.cargo,crates_and_basic_project_structure.html)