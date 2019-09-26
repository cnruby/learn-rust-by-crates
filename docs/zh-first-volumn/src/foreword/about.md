# 本书项目结构

## 作业区和篋

　　Cargo项目是Cargo工具所生成的目录和文件内容，称之为软件篋，或者简称为篋。

　　软件篋项目是由若干个Cargo项目或者说软件篋组成，在本书里，软件篋项目也简称为作业区（workspace）、整体项目或者项目。所有Cargo项目都是在作业区目录之下，且这些项目在相同目录层上。所有项目名称也是目录名称。

## 共享篋和应用程序

　　每一章最顶层目录是作业区目录，其目录名称的命名法则，使用短横线命名（kebab-case）。而每一个软件篋源代码存在于一个Cargo项目里。简称为共享软件篋程序或者共享软件篋或者共享篋，这是发布共享软件篋的Cargo项目（library package）。

　　除了这个共享软件篋的Cargo项目外，还有一个使用本地共享篋的可执行应用程序Cargo项目（binary package），称为可执行的本地程序，或者简称为本地程序。其目录名称的命名法则，也使用短横线命名（kebab-case）。其目的是在发布共享篋之前，作为完全独立的Cargo项目，来测试和应用该共享软件篋。按照Rust语言命名法则，共享软件篋名称使用小蛇式命名（lower snake case）。

　　另外，一个使用仓库crates.io里共享篋的可执行应用程序Cargo项目（binary package），称为可执行的本地程序，或者简称为仓库程序。其目录名称的命名法则，也使用短横线命名（kebab-case）。其目的是在发布共享软件篋之后，作为完全独立的Cargo项目，来测试和应用该共享软件篋。

## 说明作业区实例

　　下面说明这些目录名称含义。整个项目名称随着不同项目其名称也会不同，但每一个作业区目录下，所有功能相同的Cargo项目名称都是一样的。

<hr/>

### 项目类型清单

| 项目类型 | 项目名称 | 相对路径 |
|---|---|---|
| 作业区 | **hello-world** | ./hello-world |
| 共享篋 | lib-hello | ./hello-world/lib-hello |
| 本地程序 | bin-local-hello | ./hello-world/bin-local-hello |
| 仓库程序 | bin-hello | ./hello-world/bin-hello |

<hr/>

### 篋类型清单

| 篋类型 | 篋名称 | 相对路径 |
|---|---|---|
| 共享软件篋 | **hello_exercism** | ./hello-world/lib-hello |
| 可执行程序 | bin-local-hello | ./hello-world/bin-local-hello |
| 可执行程序 | bin-hello | ./hello-world/bin-hello |

<hr/>

### 作业区所有目录文件清单

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

| 符号 | 英文单词 | 说明 | 实例 |
|---|---|---|---|
| Ⓓ | default | 叙述Rust语言默认情况 | Ⓓ 所有模块和函数默认情况下都是私有的。 |
| ⓡ | regulation | 阐述Rust语言规则 | ⓡ 所有字符串文字类型都是引用，且具有静态生命周期。 |
| Ⓒ | consensus	| 解释常规情况下操作方法 | Ⓒ 大多数单元测试都带有注解'#[cfg(test)]'的测试模块。 |