# 关于软件篋项目hello-world

## 项目名称


### 项目清单

| 项目类型 | 项目名称 | 相对路径 |
|---|---|---|
| 作业区 | **hello-world** | ./hello-world |
| 共享篋 | lib-hello | ./hello-world/lib-hello |
| 本地程序 | bin-local-hello | ./hello-world/bin-local-hello |
| 仓库程序 | bin-hello | ./hello-world/bin-hello |

### 软件篋清单

| 篋类型 | 篋名称 | 相对路径 |
|---|---|---|
| 共享软件篋 | **hello_exercism** | ./hello-world/lib-hello |
| 可执行程序 | bin-local-hello | ./hello-world/bin-local-hello |
| 可执行程序 | bin-hello | ./hello-world/bin-hello |

## 项目结构

### 目录清单

| 目录名称 | 根目录说明 | 生成方式 |
|---|---|---|
| src | 篋源代码目录 | Cargo命令 |
| tests | 篋测试源代码目录 | 用户手动命令 |
| examples | 篋实例源代码目录 | 用户手动命令 |
| target | 篋构建目录 | Cargo命令 |
| debug | 篋调试构建目录 | Cargo命令 |
| release | 篋版本构建目录 | Cargo命令 |

### 文件清单

| 名称 | 说明 | 内容属性 | 名称属性 |
|---|---|---|---|
| README.md	| 项目说明文件 	| 可修改 | 不可修改 |
| Cargo.lock | 项目配置锁定文件 | 不可修改 | 不可修改 |
| Cargo.toml | 项目配置锁定文件 | 可修改 | 不可修改 |
| main.rs | 可执行软件篋的入口文件 | 可修改 | 不可修改 |
| lib.rs | 共享软件篋的入口文件 | 可修改 | 不可修改 |
| hello.rs | 软件篋源代码文件 | 可修改 | 可修改 |

### 所有项目结构树

```
── hello-world
    ├── Cargo.lock
    ├── Cargo.toml
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
    ├── lib-hello
    │   ├── Cargo.lock
    │   ├── Cargo.toml
    │   ├── Cargo.txt
    │   ├── README.md
    │   ├── examples
    │   │   ├── i_hello.rs
    │   │   ├── main.rs
    │   │   └── u_hello.rs
    │   ├── src
    │   │   └── lib.rs
    │   └── tests
    │       ├── i_hello.rs
    │       └── u_hello.rs
    └── target
        ├── debug
        │   ├── bin-hello
        │   ├── bin-hello.d
        │   ├── bin-hello.dSYM -> deps/bin_hello-e487e1d0c9778df2.dSYM
        │   ├── bin-local-hello
        │   ├── bin-local-hello.d
        │   ├── bin-local-hello.dSYM -> deps/bin_local_hello-8b6b201fe1d502bb.dSYM
        │   ├── build
        │   ├── deps
        │   ├── examples
        │   ├── incremental
        │   ├── libhello_exercism.d
        │   ├── libhello_exercism.rlib
        │   └── native
        └── release
            ├── bin-hello
            ├── bin-hello.d
            ├── bin-local-hello
            ├── bin-local-hello.d
            ├── build
            ├── deps
            ├── examples
            ├── incremental
            ├── libhello_exercism.d
            ├── libhello_exercism.rlib
            └── native
```

## 题外话

### 目录与命令

| 目录名称 | 生成命令 | 删除命令 |
|---|---|---|
| src | cargo new <project_name> <br/> cargo new <project_name> --lib <br/> cargo new <project_name> --bin <br/> cargo init --name <project_name> <br/> cargo init --name <project_name> --bin <br/> cargo init --name <project_name> --lib | 用户手动命令 |
| tests | 用户手动命令 | 用户手动命令 |
| examples | 用户手动命令 | 用户手动命令 |
| target | 随下面命令自动生成 | cargo clean |
| debug | cargo build 或者 cargo run | cargo clean --target-dir target/debug |
| release | cargo build --release <br/> cargo run --release | cargo clean --release <br/> cargo clean --target-dir target/release |
