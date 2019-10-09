# 衔接关键词trait与修饰关键词mut

## 学习内容
- 阐述衔接类型关键词`trait`基本概念
- 理解衔接类型关键词`trait`的实现


## 篇目

1. [衔接类型关键词`trait`概念](#衔接类型关键词trait概念)
1. [关键词`impl`和`for`概念](#关键词impl和for概念)
1. [实现类型关键词`trait`](#实现类型关键词trait)
1. [实现关键词`trait`代码](#实现关键词trait代码)
1. [程序结构图与衔接类型关键词`trait`](#程序结构图与衔接类型关键词trait)
1. [实现基于默认方法的关键词`trait`代码](#实现基于默认方法的关键词trait代码)
1. [题外话](#题外话)
1. [什么是可衍生特质](#什么是可衍生特质)
1. [参考资料](#参考资料)

## 衔接类型关键词trait概念

　　[引用][id_01]
> Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

　　直接翻译：特质(Trait)定义是一种途径，将方法一部分进行分组在一起，为实现某些目的所需以定义行为。

　　衔接类型关键词`trait`包含这些信息：

- 衔接类型关键词`trait`提供了类型的一种通道。
- 衔接类型关键词`trait`定义了类型行为功能的一部分，且也可实现行为功能。
- 这一组或者部分类型的行为功能是为了完成一项特有明确的任务。
- 单个类型可以使用关键词trait定义多组类型行为功能，来实现不同任务。

## 实现类型关键词trait

　　Rust语言规定：
- 关键词`trait`默认是私有的，但可增加修饰关键词`pub`；
- 使用关键词`trait`可以定义一个称之为特质的一组类型行为功能；
- 一旦定义了衔接类型特质，其函数和方法都是公共的，且且不可增加修饰关键词`pub`；

　　默认实例代码如下所示。尽管下面代码已定义了函数`new()`或者实现了默认函数`init()`，但是该函数外部还是不可访问的，因为该特质是私有的。注意，定义了函数`new()`是语句，是带分号`;`，而实现了默认函数`init()`是表达式，是无分号`;`。

```rust
#struct Person {
#    name: String,
#    age: u32,
#}
#
trait TraitPerson {
    fn new(name: String, age: u32) -> Person;
    fn init() -> Person { Person { name: String::new(), age: 0, } }
}
```


## 关键词`impl`和`for`概念

　　Rust语言规定：
- 关键词`impl`和`for`默认是公开的；
- 使用关键词`impl`和`for`实现的函数或者方法是不可增加修饰关键词`pub`；

　　默认实例代码如下所示。尽管使用关键词`impl`和`for`实现了特质`TraitPerson`，且其默认是公开的，但是为了使用其功能，其相关的类型`Person`和特质`TraitPerson`必须要公开的。

```rust
##![allow(dead_code)]
#mod trait_exerci {
#    pub struct Person {
#        name: String,
#        age: u32,
#    }
#
#    pub trait TraitPerson {
#        fn new(name: String, age: u32) -> Person;
#        fn init() -> Person {
#            Person {
#                name: String::new(),
#                age: 0,
#            }
#        }
#    }
#
    impl TraitPerson for Person {
        fn new(name: String, age: u32) -> Person {
            Person {
                name: name,
                age: age,
            }
        }
    }
#}
#
#//use self::trait_exerci::TraitPerson;
#use crate::trait_exerci::TraitPerson;
#
#fn main() {
#    trait_exerci::Person::init();
#    trait_exerci::Person::new(String::from("Leo"), 24);
#}
```

## 实现关键词trait代码

　　通过下面的代码，可以学习到这些知识：

- 使用关键词`trait`，定义了特质`TraitCanal`的函数`new()`；
- 使用关键词`impl`和`for`，基于结构类型`StructType`，为特质`TraitCanal`实现了实例化函数`new()`；
- 借助于特质`TraitCanal`，实现该结构类型的实例化方式；
- 借助于结构类型属性的公开性，实现了实例调用结构类型的属性手法；
- 使用宏方法`assert_eq!`，验证实例调用结果的正确性；

{{#playpen ../../../../hello-trait/lib-hello/examples/trait.rs editable}}

## 程序结构图与衔接类型关键词trait

![image](../../images/hello-trait-03-trait-impl.png)

## 实现基于默认方法的关键词trait代码

　　通过下面的代码，可以学习到这些知识：

- 使用关键词`trait`，定义了特质`TraitCanal`的默认实例化函数`init()`；
- 使用关键词`impl`和`for`，基于结构类型`StructType`，为特质`TraitCanal`实现了方法`new()`、get_data()`和`set_data()`；
- 借助于特质`TraitCanal`的默认实例化函数`init()`，实现该结构类型的实例化方式；

{{#playpen ../../../../hello-trait/lib-hello/examples/trait_with_default_method.rs editable}}

## 题外话

### 什么是可衍生特质

　　Rust语言标准库或者第三方提供了一些非常有用的特质，称之为可衍生特质（Derivable Trait）。通过注释`#[derive(特质名称)]`，编译器能够为这些特质提供实现。比如，要求类型实现是可打印的，可以使用特质std::fmt::Debug。具体说，使用可衍生特质#[derive(Debug)]，所有类型都可以自动创建地实现std::fmt::Debug。

　　下面的代码里第一行就是注释可衍生特质`Debug`，为类型`Person`实现了特质`Debug`，这些后面的宏`println!()`就可以使用了这个特质。

　　注意，使用注释`#[derive(特质名称)]`，必须紧挨着类型定义之上。

```rust
#[derive(Debug)]
struct Person {
  name: String,
  age: u32,
}

impl Person {
    fn init() -> Person {
        Person {
            name: String::new(),
            age: 0,
        }
    }
}

fn main() {
    let person = Person::init();
    println!("{:?}", person);
}
```

## 参考资料
- [trait std::fmt::Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
- [rust-by-example derive](https://doc.rust-lang.org/rust-by-example/trait/derive.html)
- [rust-by-example print_debug](https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html)
- [appendix-03-derivable-traits](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html#appendix-c-derivable-traits)


[id_01]:https://doc.rust-lang.org/book/ch10-02-traits.html?highlight=trait#defining-a-trait