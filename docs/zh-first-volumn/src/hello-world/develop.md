# 子项目：共享软件篋hello_exercism

　　创建Cargo软件篋项目所有命令，参考本章后面一节“开发软件篋命令”。

　　共享软件篋本身只能提供给其他共享篋和应用程序使用。Cargo工具实现了项目内所有目录和文件有机联系在一起。当运行测试代码或者实例代码时，这些代码都知道应该怎么样连接到正在开发的共享篋。

## 说明共享篋代码

### 项目配置文件: ./Cargo.toml

　　文件Cargo.toml是由用户编写的描述项目共享篋依赖关系。而文件Cargo.lock包含有关共享软件篋的依赖项的确切信息。它是由Cargo工具自动生成和维护的，不应手动对其进行编辑。

<span class="filename">项目配置文件: ./Cargo.toml</span>
```toml
{{#include ../../../../hello-world/lib-hello/Cargo.toml}}
```

　　在文件Cargo.toml里，最重要的一项是共享软件篋名称：name。这是使用该共享篋的入口名称。这里默认模块名称为hello_exercism。最常用的一节是共享篋依赖关系：[dependencies]。

### 程序文件lib.rs

　　程序文件lib.rs是共享篋的入口文件。这里它只有一个函数hello()，其功能就是返回一个类型为&str的字符串。可以把它看作为一个类型为&str的字符串与一个变量绑定。

　　每一个共享篋都有自己的入口模块名称，这里是hello_exercism，使用共享篋都要从这个名称开始，这里模块hello_exercism有自己的函数hello()。程序文件lib.rs可以使用关键词mod再定义模块名称，但是它们都是hello_exercism的子模块。

　　在Rust语言里，表达式和语句都可以作为一行代码。要是一行代码，最后没有分号就是表达式，而有分号就是语句。表达式只有作为函数的返回值。

　　在Rust语言里，存在一类关键词是修饰词关键词，不能独立使用它们，如关键词"pub"和“static”。

　　在Rust语言里，所有模块和函数都是私有的。要使得它们可公开访问的话，就需要使用修饰词关键词'pub'。

　　对于修饰词关键词'static'，Rust语言以与常量类似的方式提供了类似“全局变量”的功能。对每一个值只有一个实例，并且位于内存中的固定位置。

<span class="filename">Rust文件: src/lib.rs</span>
{{#playpen ../../../../hello-world/lib-hello/src/lib.rs editable}}

　　函数hello()从意义上类似于使用关键词let语句，如下所示：

<span class="filename">Rust文件: examples/main.rs</span>
{{#playpen ../../../../hello-world/lib-hello/examples/main.rs}}

### 单元测试文件u_hello.rs

　　测试文件名称命名是由用户自己确定的。

　　大多数单元测试都带有注解'#[cfg(test)]'的测试模块。每一个测试函数注解带有'#[test]'标记。每个测试函数都是单独地调用正在开发的共享软件篋进行运行的。因此Cargo工具将共享软件篋纳入到每个测试函数的范围里。

<span class="filename">Rust文件: tests/u_hello.rs</span>
{{#playpen ../../../../hello-world/lib-hello/tests/u_hello.rs editable}}

### 集成测试文件i_hello.rs

　　集成测试不需要使用注释'#[cfg(test)]'来注释任何测试代码。

<span class="filename">Rust文件: tests/i_hello.rs</span>
{{#playpen ../../../../hello-world/lib-hello/tests/i_hello.rs editable}}

### 单元实例文件u_hello.rs

　　这是可执行的Rust程序。

<span class="filename">Rust文件: examples/u_hello.rs</span>
{{#playpen ../../../../hello-world/lib-hello/examples/u_hello.rs editable}}

### 集成实例文件i_hello.rs

<span class="filename">Rust文件: examples/i_hello.rs</span>
{{#playpen ../../../../hello-world/lib-hello/examples/i_hello.rs editable}}

## 题外话

### 表达式、语句和模块

　　表达式和语句可以汇聚成一个由{}内的代码块和由关键词fn开始的函数与方法。函数和方法可以组成一个由关键词mod开始的模块。若干个模块可以形成由关键词mod开始的父模块。

### 单元测试与集成测试

　　单元测试（Unit tests）是单个模块的单独测试：它们很小且可以测试私有代码。

　　集成测试（Integration tests）在您的板条箱外部，并且仅以其公共接口的方式使用其他任何代码。 它们的目的是测试库的许多部分能否正常协同工作。

## 参考资料
- https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
- [Rust中的const和static](https://blog.csdn.net/s_lisheng/article/details/79287713)
- [unit-test-vs-integration-test](https://www.guru99.com/unit-test-vs-integration-test.html)