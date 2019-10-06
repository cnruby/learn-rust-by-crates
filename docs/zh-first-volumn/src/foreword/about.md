# 本书项目结构

## 篇目

1. [作业区和篋](#作业区和篋)
1. [共享篋和应用程序](#共享篋和应用程序)
1. [作业区命名法则](#作业区命名法则)
1. [篋命名法则](#篋命名法则)
1. [实例：项目类型清单](#实例项目类型清单)
1. [实例：篋类型清单](#实例篋类型清单)
1. [实例：作业区所有目录文件清单](#实例作业区所有目录文件清单)
1. [本书使用符号说明](#本书使用符号说明)
1. [参考资料](#参考资料)

## 作业区和篋

　　Cargo项目是Cargo工具所生成的目录和文件内容，称之为软件篋，或者简称为篋。

　　软件篋项目是由若干个Cargo项目或者说软件篋组成，在本书里，软件篋项目也简称为作业区（workspace）、整体项目或者项目。所有Cargo项目都是在作业区目录之下，且这些项目在相同目录层上。所有项目名称也是目录名称。每一章最顶层目录是作业区目录。

## 共享篋和应用程序

　　每一个软件篋源代码存在于一个Cargo项目里。

　　把Cargo项目软件篋（library package）发布到网站crates.io的共享软件篋，简称共享篋。

　　除了这个共享软件篋的Cargo项目外，还有一个使用本地共享篋的可执行应用程序Cargo项目（binary package），称为可执行的本地程序，或者简称为本地程序。其目的是在发布共享篋之前，作为完全独立的Cargo项目，来测试和应用该共享软件篋。

　　另外，一个使用仓库crates.io里共享篋的可执行应用程序Cargo项目（binary package），称为可执行的本地程序，或者简称为仓库程序。其目的是在发布共享软件篋之后，作为完全独立的Cargo项目，来测试和应用该共享软件篋。

## 作业区命名法则

　　作业区目录名称的命名法则，可以是使用短横线命名（kebab-case），也可以使用小蛇式命名（lower snake case）。

## 篋命名法则

　　按照Rust语言命名法则，共享软件篋名称使用小蛇式命名（lower snake case）。

　　可执行程序。其目录名称的命名法则，可以是使用短横线命名（kebab-case），也可以使用小蛇式命名（lower snake case）。

## 实例：项目类型清单

| 项目类型 | 项目名称 | 相对路径 |
|---|---|---|
| 作业区 | **hello-world** | ./hello-world |
| 共享篋 | lib-hello | ./hello-world/lib-hello |
| 本地程序 | bin-local-hello | ./hello-world/bin-local-hello |
| 仓库程序 | bin-hello | ./hello-world/bin-hello |

<hr/>

## 实例：篋类型清单

| 篋类型 | 篋名称 | 相对路径 |
|---|---|---|
| 共享软件篋 | **hello_exercism** | ./hello-world/lib-hello |
| 可执行程序 | bin-local-hello | ./hello-world/bin-local-hello |
| 可执行程序 | bin-hello | ./hello-world/bin-hello |

<hr/>

## 实例：作业区所有目录文件清单

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
    └── lib-hello
        ├── Cargo.lock
        ├── Cargo.toml
        ├── Cargo.txt
        ├── README.md
        ├── examples
        │   ├── i_hello.rs
        │   ├── main.rs
        │   └── u_hello.rs
        ├── src
        │   └── lib.rs
        └── tests
            ├── i_hello.rs
            └── u_hello.r
```

## 本书使用符号说明


| 符号 | 英文单词 | 说明 | 要求 | 实例 |
|---|---|---|---|---|
| Ⓓ | default | 叙述Rust语言默认情况 | 必须记住 | Ⓓ 所有模块和函数默认情况下都是私有的。 |
| ⓡ | regulation | 阐述Rust语言规则 | 必须记住 | ⓡ 所有字符串文字类型都是引用，且具有静态生命周期。 |
| Ⓒ | consensus	| 解释约定而非强制方法 | 最好记住 | Ⓒ 大多数单元测试都带有注解'#[cfg(test)]'的测试模块。 |
| Ⓘ | idea | 必须记住 | 计算机技术和Rust语言基本概念 | 

## 参考资料
- [Liste der Unicode-Zeichen der Kategorie „Sonstiges Symbol“](https://www.compart.com/de/unicode/category/So)
- [Sketchpad - Draw, Create, Share!](https://sketch.io/sketchpad/)