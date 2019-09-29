# 子项目：本地程序项目bin-local-hello

　　在共享篋hello_exercism发布以前，Cargo项目本地程序可以以独立的Cargo项目使用该共享篋，且开发应用和检查代码。

## 学习内容
- 阐述项目本地程序开发方法
- 理解项目本地程序代码

## 文件代码解释

### Cargo项目配置文件Cargo.toml

　　下面文件Cargo.toml里，与项目关系最大的一行代码是最后一行代码。这行代码说明了共享篋源代码所处的位置。

```toml
{{#include ../../../../hello-world/bin-local-hello/Cargo.toml}}
```

### 主程序文件main.rs

　　主程序文件main.rs与前面代码非常类似，但是这个程序的执行命令与以前是不一样的。

{{#playpen ../../../../hello-world/bin-local-hello/src/main.rs}}

　　执行上面程序的命令及其结果，如下所示：

```bash
$ cargo run -q
Hallo, Welt!
Hello, World!
```

### 集成测试文件i_hello.rs

　　对于共享篋hello_exercism而言，这里只能进行集成测试代码。在需要的时候，对于项目内的应用程序而言，也可以写单元测试代码。

　　下面的集成测试文件i_hello.rs方法与前面探讨的内容是一致的。

{{#playpen ../../../../hello-world/bin-local-hello/tests/i_hello.rs}}

## 参考资料
- [how-to-use-a-local-unpublished-crate](https://stackoverflow.com/questions/33025887/how-to-use-a-local-unpublished-crate)
