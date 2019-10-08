# 方法代码实现

## 学习内容
- 阐述Rust语言函数与方法概念区别
- 理解实现函数与方法手段

## 篇目

1. [修饰实例关键词`mut`概念](#修饰实例关键词mut概念)
1. [函数与方法概念](#函数与方法概念)
1. [实现方法代码](#实现方法代码)
1. [结构类型属性的私有性](#结构类型属性的私有性)
1. [题外话](#题外话)
1. [在终端里怎么样使用表格形式](#在终端里怎么样使用表格形式)
1. [参考资料](#参考资料)

## 修饰实例关键词`mut`概念

　　关键词`let`用来定义实例变量，其值是不可改变的，而在一组关键词`let mut`也用来定义实例变量，但其值是可改变的。

　　下面具体实际代码，这样执行的话，一切正常，但是去掉注释行，就会出现编译错误。

```rust, editable
fn main() {
    let instance = 1;
    // instance = 2;
    let mut instance = 1;
    instance = 2;
}
```

## 函数与方法概念

　　在Rust语言里，给予了函数（function）与方法（method）两个名称不同的概念。

　　[引用][id_01]：Methods are functions attached to objects，直接翻译：方法是附加到对象的函数（行为功能），可以这么理解，方法是附属于类型实例的行为功能。

　　而函数是附属于类型的功能。在前面一节里，可以看到，实现类型的函数代码方法及其调用手法。下面通过代码详细说明它们的区别。

## 实现方法代码

　　这里通过下面的代码，说明如下内容：

- 实现结构类型`StructType`的实例化函数`new()`。
- 实现结构类型`StructType`获取其属性的方法`get_data()`。
- 实现结构类型`StructType`变更其属性的方法`set_data()`。
- 实现结构类型`StructType`的属性`data`是私有的。
- 借助于公开性方法`get_data()`，实现结构类型的属性内容获取手法。
- 借助于公开性方法`set_data()`，实现结构类型的属性内容变更手法。

　　在Rust语言里，ⓡ 方法的第一个参数使用其本身的引用如`&self`或者`&mut self`，凡是第一个参数不是引用的就是函数。

　　调用函数如`new()`是使用类型名称如`StructType`实现的，其调用函数new()的手法形式是使用`::`，而调用函数方法如`get_data()`是使用类型的实例变量如`instance`实现的，其调用方法的手法形式是使用`.`。

{{#playpen ../../../../hello-trait/lib-hello/examples/function_methods.rs editable}}

## 结构类型属性的私有性

　　结构类型`StructType`的属性是私有的，所以模块之外是不可访问的，如使用语句`instance.data;`是不可以的。但是使用方法`get_data()`可以实现了对该属性的访问。

　　结构类型属性私有性的好处是隐蔽了结构类型的属性。

## 题外话

### 在终端里怎么样使用表格形式

　　下面打印宏`println!`语句，实现了第一列和第二列20个字符输出。

```rust
let data = 10;
println!("{0: <20} = {1: <20}", "data", data);
println!("{0: <20} = {1: <20}", "data", data);
println!("{0: <20} = {1: <20}", "data", data);
```

## 参考资料
- [rust-by-example methods](https://doc.rust-lang.org/rust-by-example/fn/methods.html)
- [how-to-print-well-formatted-tables-to-the-console](https://stackoverflow.com/questions/30379341/how-to-print-well-formatted-tables-to-the-console)

[id_01]:https://doc.rust-lang.org/rust-by-example/fn/methods.html
