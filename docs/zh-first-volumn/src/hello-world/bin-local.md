# 子项目：本地程序项目bin-local-hello

　　在共享篋hello_exercism发布以前，Cargo项目本地程序可以以独立的Cargo项目使用该共享篋，且开发应用和检查代码。

## 学习内容
- 阐述项目本地程序开发方法
- 理解项目本地程序代码

## 篇目

1. [修改项目配置文件Cargo.toml](#修改项目配置文件Cargotoml)
1. [开发主程序文件main.rs](#开发主程序文件mainrs)
1. [运行主程序及其结果](#运行主程序及其结果)
1. [运行测试代码及其结果](#运行测试代码及其结果)
1. [参考资料](#参考资料)

## 修改项目配置文件Cargo.toml

　　下面文件Cargo.toml里，与项目关系最大的一行代码是最后一行代码。这行代码说明了共享篋源代码所处的位置。

```toml
{{#include ../../../../hello-world/bin-local-hello/Cargo.toml}}
```

## 开发主程序文件main.rs

　　主程序文件main.rs与前面代码非常类似，但是这个程序的执行命令与以前是不一样的，并且代码文件结构形式也是不一样的，程序代码和测试代码存在于一个文件里。

{{#playpen ../../../../hello-world/bin-local-hello/src/main.rs}}


## 运行主程序及其结果

```bash
# 运行主程序及其结果
$ cargo run -q
Hallo, Welt!
Hello, World!
```

## 运行测试代码及其结果

```bash
# 运行测试代码及其结果
$ cargo test -q

running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## 参考资料
- [how-to-use-a-local-unpublished-crate](https://stackoverflow.com/questions/33025887/how-to-use-a-local-unpublished-crate)
