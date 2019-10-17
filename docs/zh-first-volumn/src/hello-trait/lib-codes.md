# 共享篋：程序代码解释

## 学习内容
- 学习和理解关键词`trait`开发过程

## 篇目

1. [项目共享篋程序结构示意图](#项目共享篋程序结构示意图)
1. [程序代码解释](#程序代码解释)
1. [理解关键词`trait`和`impl`关系](#理解关键词trait和impl关系)
1. [题外话](#题外话)
1. [标准库平等比较特质`PartialEq`](#标准库平等比较特质PartialEq)
1. [参考资料](#参考资料)

## 项目共享篋程序结构示意图

　　下面示意图是共享篋`trait_exerci`程序的结构。该共享篋提供了对外三个衔接通道：两个特质`TraitCanal`和`TraitKanal`以及一个类型的自我实现`StructType`。

![image](../../images/hello-trait-05-complex.png)

## 程序代码解释

　　通过下面代码，实现了如下内容：

- 结构类型`StructType`进行了自我实现；
- 针对结构类型`StructType`，定义了两个特质`TraitCanal`和`TraitKanal`；
- 对于结构类型`StructType`，实现了两个特质`TraitCanal`和`TraitKanal`；

{{#playpen ../../../../hello-trait/lib-hello/src/lib.rs editable}}

　　结构类型`StructType`自我实现，包含了一个函数`new()`和两个方法`get_data_for_all()`和`set_data_for_all()`。

　　针对一个结构类型`StructType`，定义且实现了两个不同的特质`TraitCanal`和`TraitKanal`。特质`TraitCanal`包含一个方法`get_data()`，而特质`TraitKanal`包含一个方法`set_data()`。

## 理解关键词`trait`和`impl`关系

　　针对一个结构类型`StructType`实例，可以存在不同的特质，只要把这些特质在一起使用，这些不同特质实现的函数和方法是可以相通的。

　　任何结构类型`StructType`的特质实现，都可以使用结构类型`StructType`自我实现的函数和方法。

## 题外话

### 标准库平等比较特质`PartialEq`

　　标准库平等比较特质`PartialEq`，可以比较类型的实例以检查它们是否相等。

```rust, editable
#[derive(Default, Debug, PartialEq)]
pub struct Person {
    name: String,
    age: u32,
}

fn main() {
    assert_eq!(
        Person::default(),
        Person {
            name: String::new(),
            age: 0
        }
    );
}
```

## 参考资料
- [std::cmp::PartialEq](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)
- [PartialEq#partialeq-and-eq-for-equality-comparisons](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html?highlight=PartialEq#partialeq-and-eq-for-equality-comparisons)