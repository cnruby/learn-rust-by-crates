# 共享软件篋 hello_exercism ：目录src的测试代码解释

## 学习内容
- 了解共享篋目录src下的测试代码结构
- 了解私有代码的单元测试方法
- 了解目录src下的集成测试方法

## 目录src测试代码结构

```bash
$ tree ./src -L 3
./src
├── integration_tests
│   ├── i_hello.rs
│   └── mod.rs
├── lib.rs
└── private_tests
    ├── mod.rs
    └── owned_hello.rs
```

## 默认模块文件mod.rs

　　Ⓓ 从目录src开始，Cargo项目共享篋程序目录名称就是模块名称。目录src就是共享篋模块名称，其模块文件就是lib.rs。

　　如：这里共享篋名称hello_exercism就是模块名称

　　Ⓓ 所有目录src的子目录也是模块名称，其模块文件就是mod.rs。

　　如，目录private_tests就是模块hello_exercism的子模块。

　　Ⓓ 除了lib.rs和mod.rs文件以外，所有其它文件名称也就是模块名称，且其文件名称就是模块名称。

　　如，文件i_hello.rs就是一个模块，其模块名称为i_hello。

## 私有代码的单元测试

　　Cargo项目私有代码的单元测试思路：单元测试与共享篋程序代码是融为一体的。所以测试代码都是在私有代码相关的可访问模块里，是不可分开的。

### 程序文件lib.rs与单元测试文件owned_hello.rs

　　为了测试共享篋程序文件src/lib.rs的私有函数hallo()，需要将测试代码存放在该文件的可访问模块里，测试代码可以分离到另外文件里。

　　在篋程序文件src/lib.rs里，对于私有代码的单元测试，有四段单元测试代码，它们是为三个不同模块：hello_exercism::private_tests::owned_hello、hello_exercism::private_tests_with_use和hello_exercism::private_tests_without_use，而每一个模块都是一个单元测试函数，其测试目的和代码含义都是完全一样的，只是代码形式不一样。

{{#playpen ../../../../hello-world/lib-hello/src/lib.rs editable}}

　　在程序文件lib.rs里，第一段代码和第二段代码方法都是把测试代码分离到另外文件里，这里它们指向相同的单元测试文件或者说模块。分离文件'src/private_tests/owned_hello.rs'如下所示里。它们的第二行说明其下一行模块的位置。

　　Ⓓ 因为第二段代码的访问模块方式是默认方式，所以第二行代码可以省略。

{{#playpen ../../../../hello-world/lib-hello/src/private_tests/owned_hello.rs editable}}

　　第三段代码和第四段代码方法是把测试代码存放在可访问私有代码的模块里。它们仅仅是否使用了关键词use不同而已。

　　第三段代码的第三行说明该模块hello_exercism::private_tests_with_use需要访问其父模块hello_exercism的所有函数。

　　第四段代码里super也是说明了需要使用期父模块的函数hallo()。

　　在程序文件mod.rs和owned_hello.rs里，第一行代码都是需要访问其父模块的所有函数。因为从模块owned_hello出发，需要访问其上两层模块，使用两个模块里都需要使用super语句。

{{#playpen ../../../../hello-world/lib-hello/src/private_tests/mod.rs editable}}

## 基于共享篋目录src内的集成测试

　　基于共享篋目录src内的集成测试，与私有代码的单元测试思路有类似性，其测试代码也都是在模块程序代码里，但是有本质上区别，它仅仅使用了共享篋模块结构属性，而非共享篋的原代码，因此它是只能访问共享篋的公共接口。

　　在程序文件lib.rs里，存在两段集成测试集成代码。第一段代码和第二段代码方法都是把测试代码分离到另外文件里，这里它们指向相同的集成测试文件或者说模块。代码原理与前面私有代码的单元测试是完全一样的。

　　不同代码的是，程序文件mod.rs和集成测试文件i_hello.rs。程序文件mod.rs没有super相关语句，只是说明了使用i_hello模块。集成测试文件i_hello.rs也是不一样的，也没有super相关语句，而是引用了一行使用自己模块的语句，且把自己也称之为模块hello_exercism，这个模块名称可以随意自己命名。

{{#playpen ../../../../hello-world/lib-hello/src/integration_tests/mod.rs editable}}

{{#playpen ../../../../hello-world/lib-hello/src/integration_tests/i_hello.rs editable}}

- ![image](../../images/crates_io_api_access_create.png)

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