# 子项目：共享软件篋hello_exercism测试代码

## 学习内容
- 了解Rust语言测试代码
- 理解共享篋项目测试代码
- 了解Rust语言公共接口的单元测试
- 了解Rust语言私有代码的单元测试
- 了解Rust语言集成测试

## 公共接口的单元测试

### 单元测试文件u_hello.rs

　　Ⓘ 单元测试（Unit tests）是单个模块或者说单个软件篋的单独测试。它们很小且可以测试私有代码。它们的目的是软件篋每一个功能能否正常工作。

　　什么是模块？

　　ⓡ 从模块范围之外只能访问模块的公共项，而不能访问模块的私有项。

　　Ⓒ 大多数单元测试都带有注解'#[cfg(test)]'的测试模块。

　　ⓡ 每一个单元测试函数注解带有'#[test]'标记。

　　每个测试函数都是单独地调用正在开发的共享软件篋进行运行的。因此Cargo工具将共享软件篋纳入到每个测试函数的范围里。测试文件名称命名是由用户自己确定的。

{{#playpen ../../../../hello-world/lib-hello/tests/u_hello.rs editable}}

## 集成测试

### 集成测试文件i_hello.rs

　　Ⓘ 集成测试（Integration tests）是与外部的多个共享篋的测试。它们比较大，但仅测试正在开发共享篋的公共接口。它们的目的是与其它篋能否正常协同工作。

　　Ⓒ 集成测试不需要使用注释'#[cfg(test)]'来注释任何测试代码。

　　ⓡ 每一个单元测试函数注解带有'#[test]'标记。

{{#playpen ../../../../hello-world/lib-hello/tests/i_hello.rs editable}}

## 私有代码的单元测试

### 单元测试文件tests.rs

{{#playpen ../../../../hello-world/lib-hello/private/tests.rs editable}}

## 参考资料
- [unit_testing](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
- [unit-test-vs-integration-test](https://www.guru99.com/unit-test-vs-integration-test.html)
- [rust-unit-test-placement](http://xion.io/post/code/rust-unit-test-placement.html)