# 共享篋：目录测试代码解释

## 篇目

1. [结构类型`StructType`自我实现的单元测试代码](#结构类型StructType自我实现的单元测试代码)
1. [特质`TraitCanal`实现的单元测试代码](#特质TraitCanal实现的单元测试代码)
1. [特质`TraitKanal`实现的单元测试代码](#特质TraitKanal实现的单元测试代码)
1. [所有实现的单元测试代码](#所有实现的单元测试代码)
1. [基于模块的单元测试代码](#基于模块的单元测试代码)
1. [题外话](#题外话)
1. [浅说指针类型`Box`](#浅说指针类型Box)
1. [参考资料](#参考资料)

## 结构类型`StructType`自我实现的单元测试代码

　　下面单元测试仅仅使用了结构类型`StructType`的自我实现。四个方法分别测试了：

- 使用结构类型`StructType`自我实现的实例化函数new();
- 使用标准库默认特质`Default`实现的实例化函数default();
- 使用结构类型`StructType`自我实现的方法get_data_for_all();
- 使用结构类型`StructType`自我实现的方法set_data_for_all();

{{#playpen ../../../../hello-trait/lib-hello/tests/u_hello_for_struct.rs editable}}

## 特质`TraitCanal`实现的单元测试代码

　　下面单元测试使用了结构类型`StructType`的自我实现和特质`TraitCanal`实现。特别需要注意的是，从代码表面上看，第二行语句与后面代码没有任何关系。之所以代码里可以使用方法`get_data()`，就是因为第二行语句的作用。当我们写下第二行语句时，就应该知道接下来我们将要使用什么函数或／和方法。

　　方法`it_works_with_get()`测试了：

- 使用结构类型`StructType`自我实现的实例化函数`new()`;
- 使用特质`TraitCanal`实现的方法`get_data()`;

　　方法`it_works_with_default()`测试了：

- 使用标准库默认特质`Default`实现的实例化函数`default()`;
- 使用特质`TraitCanal`实现的方法`get_data()`;

{{#playpen ../../../../hello-trait/lib-hello/tests/u_hello_for_canal.rs editable}}

## 特质`TraitKanal`实现的单元测试代码

　　下面单元测试使用了结构类型`StructType`的自我实现和特质`TraitKanal`实现。所以这里没有使用到特质`TraitCanal`实现。尽管下面代码没有使用特质`TraitCanal`实现，但是我们使用了标准库平等比较特质`PartialEq`，实现了代码测试。

　　方法`it_works_with_get()`测试了：

- 使用结构类型`StructType`自我实现的实例化函数`new()`;
- 使用特质`TraitKanal`实现的方法`get_data()`;

　　方法`it_works_with_default()`测试了：

- 使用标准库默认特质`Default`实现的实例化函数`default()`;
- 使用特质`TraitKanal`实现的方法`get_data()`;

{{#playpen ../../../../hello-trait/lib-hello/tests/u_hello_for_kanal.rs editable}}

## 所有实现的单元测试代码

　　下面单元测试程序的所有三个实现。所以最前面三行语句表示使用这三个实现。

{{#playpen ../../../../hello-trait/lib-hello/tests/u_hello_for_both_traits.rs editable}}

## 基于模块的单元测试代码

　　下面单元测试主要说明在模块下如何进行单元测试。除了定义模块之外，最重要的是，使用三个实现语句必须在模块内。

{{#playpen ../../../../hello-trait/lib-hello/tests/mod_hello.rs editable}}

## 题外话

### 浅说指针类型`Box`

　　Rust标准库提供了类型`Box<T>`。可以使用该类型`Box<T>`在堆上分配内容。此类型用于安全地抽象指向堆内存的指针。同时它具有更大的灵活性，允许将实现特质如`TraitCanal`的任何事物进行`Box`类型化。

```rust, editable
#[derive(Debug, Default)]
pub struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person :Box<Person> = Box::new(Default::default());
    println!("{:?}", person);
}
```

## 参考资料
- [what-does-the-box-keyword-do](https://stackoverflow.com/questions/30352802/what-does-the-box-keyword-do)
- [To Box or not to Box — My First Real Rust Refactor](https://medium.com/@KevinHoffman/to-box-or-not-to-box-my-first-real-rust-refactor-db467119c4c7)