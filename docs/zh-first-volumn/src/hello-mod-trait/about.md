# 关于软件篋mod_trait_exerci

## 学习内容
- 了解项目名称和目录

## 篇目

- [项目名称清单](#项目名称清单)
- [软件篋类型清单](#软件篋类型清单)
- [项目目录文件结构](#项目目录文件结构)

## 项目名称清单

| 项目类型 | 项目名称 | 相对路径 | 项目说明 |
|---|---|---|---|
| 作业区 | **hello-mod-trait** | ./hello-mod-trait | 开发共享软件篋工作区 |
| 共享篋 | lib-hello | ./hello-mod-trait/lib-hello | 开发共享软件篋实例 |
| 本地程序 | bin-local-hello | ./hello-mod-trait/bin-local-hello | 使用在本地的共享篋 |
| 仓库程序 | bin-hello | ./hello-mod-trait/bin-hello | 使用在crates.io上共享篋 |

## 软件篋类型清单

| 篋类型 | 篋名称 | 相对路径 |
|---|---|---|
| 共享软件篋 | **mod_trait_exerci** | ./hello-mod-trait/lib-hello |
| 可执行程序 | bin-local-hello | ./hello-mod-trait/bin-local-hello |
| 可执行程序 | bin-hello | ./hello-mod-trait/bin-hello |

## 项目目录文件结构

```bash
$ tree -L 3
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── bin-hello
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── src
│   │   ├── bin
│   │   └── main.rs
│   └── target
│       └── debug
├── bin-local-hello
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── src
│   │   └── main.rs
│   ├── target
│   │   └── debug
│   └── tests
│       └── type_both_hello.rs
└── lib-hello
    ├── Cargo.lock
    ├── Cargo.toml
    ├── README.md
    ├── examples
    │   ├── bare_hello.rs
    │   ├── box_dynamic_hello.rs
    │   ├── box_static_hello.rs
    │   ├── generics_fn_hello.rs
    │   ├── generics_impl_hello.rs
    │   ├── generics_trait_hello.rs
    │   ├── generics_type_hello.rs
    │   ├── simple_dynamic_dispatch.rs
    │   ├── simple_static_dispatch.rs
    │   ├── trait_dispatch_abstract.rs
    │   ├── trait_dispatch_concrete.rs
    │   ├── trait_fn_hello.rs
    │   ├── trait_instance_hello.rs
    │   └── trait_where_hello.rs
    ├── src
    │   ├── lib.rs
    │   ├── mod_bare
    │   ├── mod_dynamic_fn.rs
    │   ├── mod_static_fn.rs
    │   └── mod_where_fn.rs
    └── tests
        ├── box_dynamic_hello.rs
        ├── box_static_hello.rs
        ├── mod_bare.rs
        └── mod_trait.rs
```