# 共享篋hello_exercism：目录tests的测试代码解释

　　之所以在本共享篋里测试代码存在重复，是因为需要解释不同的问题。

## 学习内容
- 理解开发共享篋的测试代码
- 了解公共接口的单元测试方法
- 了解目录tests的集成测试方法

## 公共接口的单元测试

### 单元测试文件u_hello.rs

　　在下面的单元测试程序里，只有一个测试函数，其功能是判断函数hello()返回值与字符串文字“Hello, World!”是否完全一致。

{{#playpen ../../../../hello-world/lib-hello/tests/u_hello.rs editable}}

　　Ⓘ 单元测试（Unit tests）是单个模块或者说单个软件篋的单独测试。

　　一般情况下，单元测试很小且可以测试私有代码。它们的目的是软件篋每一个功能能否正常工作。Rust语言将单元测试分成两类：这里探讨公共接口的单元测试和下面将要说明的私有代码的单元测试。

　　ⓡ 从模块范围之外只能访问模块的公共接口，而不能访问模块的私有内容。

　　Ⓒ 大多数单元测试都带有注解'#[cfg(test)]'的测试模块。

　　ⓡ 每一个单元测试函数带有'#[test]'注解标记。测试文件名称命名是由用户自己确定的。

　　每个测试函数都是单独地调用正在开发的共享软件篋进行运行的。因此Cargo工具将共享软件篋纳入到每个测试函数的范围里。

　　Ⓓ 所有公共接口的单元测试文件存储于默认测试目录tests下。

## 基于目录tests的集成测试

### 集成测试文件i_hello.rs

　　在下面的集成测试程序里，有两个测试函数，第一个函数功能是判断外部共享篋的函数hello()返回值与字符串文字“Hello, World!”是否完全一致。第二个函数功能是判断正在开发共享篋与外部共享篋的函数hello()返回值是否完全一致。

{{#playpen ../../../../hello-world/lib-hello/tests/i_hello.rs editable}}

　　Ⓘ 集成测试（Integration tests）是与外部的多个共享篋的测试。它们比较大，但仅测试正在开发共享篋的公共接口。它们的目的是与其它篋能否正常协同工作。

　　集成测试可以存储于这里探讨的基于测试目录tests，也可以存储于下面将要解释的基于共享篋目录src。但是它们处理代码的方式是完全不同的。

　　Ⓒ 集成测试不需要使用注释'#[cfg(test)]'来注释任何测试代码。

　　ⓡ 每一个单元测试函数注解带有'#[test]'标记。

　　ⓡ 集成测试文件可以存储于默认测试目录tests下。

## 参考资料
- [unit_testing](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
- [unit-test-vs-integration-test](https://www.guru99.com/unit-test-vs-integration-test.html)
- [rust-unit-test-placement](http://xion.io/post/code/rust-unit-test-placement.html)
- [writing-integration-tests-in-rust](https://klausi.github.io/rustnish/2017/05/25/writing-integration-tests-in-rust.html)
- [integration-testing-a-service-written-in-rust-and-iron](https://www.nibor.org/blog/integration-testing-a-service-written-in-rust-and-iron/)
- [practical-rust-web-development-testing](https://dev.to/werner/practical-rust-web-development-testing-4eo5)
- [book/contrib-test](https://rust-random.github.io/book/contrib-test.html)
- [testing-in-rust-temporary-files](http://andrewradev.com/2019/03/01/testing-in-rust-temporary-files/)
- [unit-tests-with-rust-tutorial-101](https://jonathanmh.com/unit-tests-with-rust-tutorial-101/)
- [use-declarations](https://doc.rust-lang.org/reference/items/use-declarations.html)