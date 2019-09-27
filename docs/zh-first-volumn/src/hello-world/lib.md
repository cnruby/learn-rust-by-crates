# 子项目：共享软件篋hello_exercism代码

　　共享软件篋本身只能提供给其他共享篋和应用程序使用。Cargo工具实现了项目内所有目录和文件有机联系在一起。当运行测试代码或者实例代码时，这些代码都知道应该怎么样连接到正在开发的共享篋。

## 学习内容
- 阐述Rust语言文件功能
- 理解项目共享篋程序代码

## 项目配置文件

### 项目配置文件: ./Cargo.toml

　　文件Cargo.toml是由用户编写的描述项目共享篋依赖关系。而文件Cargo.lock包含有关共享软件篋的依赖项的确切信息。它是由Cargo工具自动生成和维护的，不应手动对其进行编辑。

```toml
{{#include ../../../../hello-world/lib-hello/Cargo.toml}}
```

　　在文件Cargo.toml里，最重要的一项是共享软件篋名称：name。这是使用该共享篋的入口名称。这里默认模块名称为hello_exercism。

　　最常用的一节是共享篋依赖关系：[dependencies]。这里有一行依赖关系代码，说明共享篋有赖于外部共享篋。

## 程序文件lib.rs

　　程序文件lib.rs是共享篋的入口文件。这里它有两个函数：hello()和hallo()，其功能都是返回一个字符串文字。hello()返回英文问候，而hallo()返回德文问候。它们返回的类型也都是&'static str的字符串，这一类型是静态生命周期的字符串文字，或者简单说静态的字符串文字。

　　ⓡ 所有字符串文字类型都是引用，且具有静态生命周期的功能。

　　↳ 这里的函数返回值是包含一个引用字符串文字类型值，所以函数返回类型也要此类型`&str`。因为在整个程序过程中需要该类型是有效的，所以此类型还要是静态生命周期`'static`。

　　Ⓓ 在默认情况下，所有使用关键词mod定义的模块和使用关键词fn定义的函数都是私有的。要使得它们可公开访问的话，就需要使用修饰词关键词'pub'。

　　↳ hello()是公共可访问的函数，而hallo()是只有模块hello_exercism内可访问的私有函数。

　　↳ 所有私有函数的单元测试代码必须在程序文件内。将在后面说明该文件的相关测试代码。

　　Ⓓ 共享篋模块默认情况下是公开的。

　　↳ 这里共享篋模块名称是hello_exercism，尽管它没有使用关键词mod定义，但是Rust语言默认定义了它。

{{#playpen ../../../../hello-world/lib-hello/src/lib.rs editable}}

## 说明目录examples的实例文件

### 单元实例文件u_hello.rs

　　这是可执行的Rust程序。

{{#playpen ../../../../hello-world/lib-hello/examples/u_hello.rs editable}}

### 集成实例文件i_hello.rs

{{#playpen ../../../../hello-world/lib-hello/examples/i_hello.rs editable}}

## 题外话

### Rust语言类型与关键词

　　对于修饰词关键词'static'，Rust语言以与常量类似的方式提供了类似“全局变量”的功能。对每一个值只有一个实例，并且位于内存中的固定位置。

　　在Rust语言里，存在一类关键词是修饰词关键词，如关键词"pub"和“static”。

| 类型 | 关键词 | 类型说明 |
|---|---|---|
| &str |  	| 字符串文字 |
|  	| 'static | 静态修饰词关键词 |
|  	| ' | 生命周期注释符 |
|  	| 'static | 静态生命周期注释符 |

### 浅谈软件篋的模块

　　每一个共享篋都有自己的入口模块名称，这里是hello_exercism，使用共享篋都要从这个名称开始，这里模块hello_exercism有自己的函数hello()。程序文件lib.rs可以使用关键词mod再定义模块名称，但是它们都是hello_exercism的子模块。


### 函数hello()功能

　　从作用意义上，共享篋的函数hello()类似于使用关键词let语句，即可把它看作为一个类型为&str的字符串与一个变量绑定，如下所示：

{{#playpen ../../../../hello-world/lib-hello/examples/main.rs}}

### 表达式、语句和模块

　　在Rust语言里，表达式和语句都可以作为一行代码。要是一行代码，最后没有分号就是表达式，而有分号就是语句。表达式只有作为函数的返回值。

　　表达式和语句可以汇聚成一个由{}内的代码块和由关键词fn开始的函数与方法。函数和方法可以组成一个由关键词mod开始的模块。若干个模块可以形成由关键词mod开始的父模块。

### 项目配置文件

　　除了项目配置文件Cargo.toml之外，还可以有其它功能的配置文件，如工具rustfmt的配置文件。

## 参考资料
- [Rust中的const和static](https://blog.csdn.net/s_lisheng/article/details/79287713)
- [specifying-dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)