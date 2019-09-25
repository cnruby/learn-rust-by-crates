# 本书项目结构

## 项目、共享篋和应用程序

　　Cargo项目是Cargo工具所生成的目录文件内容。软件篋项目是由若干个Cargo项目组成，在本书里，软件篋项目也简称为整体项目或者项目。所有Cargo项目都是在项目目录之下，并且在相同目录层上。所有项目名称也是目录名称。

　　每一章最顶层目录是项目目录，其目录名称的命名法则，使用短横线命名（kebab-case）。而每一个软件篋源代码存在于一个Cargo项目里。简称为共享软件篋程序或者共享软件篋或者共享篋，这是发布共享软件篋的Cargo项目（library package）。

　　除了这个共享软件篋的Cargo项目外，还有一个使用本地共享篋的可执行应用程序Cargo项目（binary package），称为可执行的本地程序，或者简称为本地程序。其目录名称的命名法则，也使用短横线命名（kebab-case）。其目的是在发布共享篋之前，作为完全独立的Cargo项目，来测试和应用该共享软件篋。按照Rust语言命名法则，共享软件篋名称使用小蛇式命名（lower snake case）。

　　另外，一个使用仓库crates.io里共享篋的可执行应用程序Cargo项目（binary package），称为可执行的本地程序，或者简称为仓库程序。其目录名称的命名法则，也使用短横线命名（kebab-case）。其目的是在发布共享软件篋之后，作为完全独立的Cargo项目，来测试和应用该共享软件篋。

## 说明项目结构实例

　　下面说明这些目录名称含义。整个项目名称随着不同项目其名称也会不同，但每一个整体项目目录下，所有Cargo项目名称都是一样的。

<hr/>

| 项目类型 | 项目名称 | 相对路径 |
|---|---|---|
| 整个项目 | **hello-world** | ./hello-world |
| 共享篋 | lib-hello | ./hello-world/lib-hello |
| 本地程序 | bin-local-hello | ./hello-world/bin-local-hello |
| 仓库程序 | bin-hello | ./hello-world/bin-hello |

<hr/>

| 篋类型 | 篋名称 | 相对路径 |
|---|---|---|
| 共享软件篋 | **hello_exercism** | ./hello-world/lib-hello |
| 可执行程序 | bin-local-hello | ./hello-world/bin-local-hello |
| 可执行程序 | bin-hello | ./hello-world/bin-hello |

<hr/>

```
── hello-world
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
    └── lib-hello
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

