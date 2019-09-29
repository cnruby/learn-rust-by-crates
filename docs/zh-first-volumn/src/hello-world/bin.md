# 子项目：仓库程序项目bin-hello

　　在共享篋hello_exercism发布以后，Cargo项目仓库程序可以以独立的Cargo项目使用共享篋，且开发应用和检查代码。一般情况下。用户使用共享篋是以这种项目形式出现的。

## 学习内容
- 阐述项目仓库程序开发方法
- 理解项目仓库程序代码

## 文件代码解释

### Cargo项目配置文件Cargo.toml

　　下面文件Cargo.toml里，与项目关系最大的一行代码是最后一行代码。这行代码说明了所使用的共享篋，包括共享篋名称和版本号。

```toml
{{#include ../../../../hello-world/bin-hello/Cargo.toml}}
```

### 主程序文件main.rs

　　下面主程序文件main.rs与前面项目bin-local-hello完全是一样的。

{{#playpen ../../../../hello-world/bin-hello/src/main.rs}}

　　执行上面程序的命令及其结果，如下所示：

```bash
$ cargo run -q
Hallo, Welt!
Hello, World!
```

### 集成测试文件i_hello.rs

　　下面的集成测试文件i_hello.rs与前面项目bin-local-hello完全是一样的。

{{#playpen ../../../../hello-world/bin-hello/tests/i_hello.rs}}
