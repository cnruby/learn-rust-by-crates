# 子项目：仓库程序项目bin-hello

　　在共享篋hello_exercism发布以后，Cargo项目仓库程序可以以独立的Cargo项目使用共享篋，且开发应用和检查代码。一般情况下。用户使用共享篋是以这种项目形式出现的。

　　这个仓库程序项目介绍了一种方法，在一个可执行的软件篋里，存在多个独立的可执行程序。

## 学习内容
- 阐述项目仓库程序开发方法
- 理解项目仓库程序代码

## 篇目

1. [项目配置文件Cargo.toml](#项目配置文件Cargotoml)
1. [主程序文件main.rs](#主程序文件mainrs)
1. [集成测试文件i_hello.rs](#集成测试文件i_hellors)

## 项目配置文件Cargo.toml

　　下面文件Cargo.toml里，与项目关系最大的一行代码是最后一行代码。这行代码说明了所使用的共享篋，包括共享篋名称和版本号。

```toml
{{#include ../../../../hello-world/bin-hello/Cargo.toml}}
```

## 主程序文件src/bin/hello.rs

　　下面主程序文件main.rs与前面项目bin-local-hello完全是一样的。

{{#playpen ../../../../hello-world/bin-hello/src/bin/hello.rs}}

　　执行上面程序的命令及其结果，如下所示：

```bash
$ cargo run -q
Hallo, Welt!
Hello, World!
```

## 主程序文件src/bin/hallo.rs

{{#playpen ../../../../hello-world/bin-hello/src/bin/hallo.rs}}

## 集成测试文件i_hello.rs

　　下面的集成测试文件i_hello.rs与前面项目bin-local-hello完全是一样的。

{{#playpen ../../../../hello-world/bin-hello/tests/i_hello.rs}}
