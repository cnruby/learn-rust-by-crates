# 特质实现及其对象

## 学习内容
- 理解衔接类型关键词`trait`的实现

## 篇目

- [程序结构图与衔接类型关键词`trait`](#程序结构图与衔接类型关键词trait)
- [实现基于默认方法的关键词`trait`代码](#实现基于默认方法的关键词trait代码)
- [特质对象解释](#特质对象解释)
- [题外话](#题外话)
- [什么是可衍生特质](#什么是可衍生特质)
- [参考资料](#参考资料)

## 程序结构图与衔接类型关键词trait

![image](../../images/hello-trait-03-trait-impl.png)

## 实现基于默认方法的关键词trait代码

　　通过下面的代码，可以学习到这些知识：

- 使用关键词`trait`，定义了特质`TraitCanal`的默认实例化函数`init()`；
- 使用关键词`impl`和`for`，基于结构类型`StructType`，为特质`TraitCanal`实现了方法`new()`、get_data()`和`set_data()`；
- 借助于特质`TraitCanal`的默认实例化函数`init()`，实现该结构类型的实例化方式；

{{#playpen ../../../../hello-trait/lib-hello/examples/trait_with_default_method.rs editable}}

## 特质对象解释

　　特质本身不能定义特质对象，而是通过类型的对象，进行强制转换得到的特质对象。特质对象可以访问类型的公共数据和公共特质的行为。

　　Rust语言把面向对象编程的思想更加深化了。把类的数据与行为分散化定义，而把类的实例集成化使用。不仅如此，而且Rust语言在代码里完全把这种过程都隐藏起来了。

![image](../../images/hello-trait-31-trait-object.png)

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