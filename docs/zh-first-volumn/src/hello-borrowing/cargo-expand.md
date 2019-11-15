# 可变类型派生分析

 　　首先，通过自定义结构类型`Struct(u8)`，展示派生属性`Clone`和`Copy`的代码，且说明两种派生属性区别。其次，对于派生属性`Clone`和`Copy`，应用于结构类型`Struct(u8)`与`Struct(String)`的区别。最后说明自定义类型`Struct<'cn>(&'cn &str)`的派生属性。

## 学习内容
- 了解和学习Rust语言可变类型的派生

## 篇目

- [可变类型的派生属性`Clone`和`Copy`](#可变类型的派生属性clone和copy)
- [可变类型的派生属性`Clone`](#可变类型的派生属性clone)
- [可变类型的派生属性`Copy`](#可变类型的派生属性copy)
- [分析不同类型的派生属性](#分析不同类型的派生属性)
- [自定义类型Struct<'cn>(&'cn str)的派生属性](#自定义类型structcncn-str的派生属性)
- [题外话](#题外话)
- [参考资料](#参考资料)

## 可变类型的派生属性`Clone`和`Copy`

　　在介绍可变类型的派生属性`Clone`和`Copy`之前，先考察下面不可变类型的实例。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/expand/use_u8.rs }}
```

　　上面程序是可运行的。使用派生工具`cargo-expand`，展开其代码的结果如下：

```rust,no_run,noplaypen
mod use_u8 {
    #![allow(unused_variables)]
    pub fn adjoin() {
        let instance = 42u8;
        let clone_instance = instance.clone();
        let copy_instance = instance;
        let use_instance = instance;
    }
}
```

　　对于自定义的可变类型，要能够保证其对象克隆和复制，必须实现`Clone`和`Copy`两个特质，该类型才能使用克隆和复制其对象。途径有两条：要么自己实现该类型的这两个特质；要么使用语言提供的默认实现。这里使用后面方法。

　　下面程序是包含了自定义的可变类型。之所以该程序能够正常运行，是因为，下面与上面程序比较可以知道，除了类型不同之外，还多增加了包括两个派生属性`Clone`和`Copy`的一行代码，其作用是使得该类型可以克隆和复制其对象。对于不可变类型，Rust语言已经实现了克隆和复制功能，在这种情况下，克隆和复制功能是等价的。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/expand/struct_u8.rs:feature-ok }}
```

　　下面看看上面程序其两个派生属性`Clone`和`Copy`代码是怎么样实现的。使用派生工具运行结果如下。可以看到对于自定义类型`Struct(u8)`，存在两个特质`Clone`和`Copy`实现：

```rust,no_run,noplaypen
mod struct_u8 {
    #![allow(unused_variables)]
    #[cfg(feature = "ok")]
    pub fn adjoin() {
        struct Struct(u8);
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Struct {
            #[inline]
            fn clone(&self) -> Struct {
                {
                    let _: ::core::clone::AssertParamIsClone<u8>;
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for Struct {}
        let instance: Struct = Struct(42u8);
        let clone_instance = instance.clone();
        let copy_instance = instance;
        let use_instance = instance;
    }
}
```

　　对于上面这种可变类型，要是其每一元素都是不可变类型，Rust语言可以实现了克隆和复制功能，同样，在这种情况下，克隆和复制功能也是等价的。

　　在上面程序里，之所以该类型对象`instance`能够被克隆和复制，是因为该类型实现了克隆特质`Clone`；之所以该类型对象`instance`能够被转移（move），是因为该类型实现了复制特质`Copy`。下面看看这是为什么。

## 可变类型的派生属性`Clone`

　　注意比较上下两个程序的代码，下面代码缺少了复制特质`Copy`，且最后一行代码不是使用对象`instance`，而是使用`copy_instance`作为变量的绑定值。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/expand/struct_u8.rs:feature-cp }}
```

　　还是先让我们使用派生工具，展开其代码，其运行结果：

```rust,no_run,noplaypen
mod struct_u8 {
    #![allow(unused_variables)]
    #[cfg(feature = "cp")]
    pub fn adjoin() {
        struct Struct(u8);
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Struct {
            #[inline]
            fn clone(&self) -> Struct {
                match *self {
                    Struct(ref __self_0_0) => Struct(::core::clone::Clone::clone(&(*__self_0_0))),
                }
            }
        }
        let instance = Struct(42u8);
        let clone_instance = instance.clone();
        let copy_instance = instance;
        let use_copy_instance = copy_instance;
    }
}
```

　　从上面展开的代码来看，代码里不仅没有了复制特质`Copy`的实现，而且复制特质`Copy`存在与否直接影响到克隆特质`Clone`代码的实现。

　　在上面程序里，之所以该类型对象`instance`能够被克隆和复制，是因为该类型实现了克隆特质`Clone`；之所以该类型对象`instance`不能被转移（move），是因为该类型没有实现复制特质`Copy`。下面看看这是为什么。

　　上下程序唯一不同是最后一行代码，但是其结果完全不同了。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/expand/struct_u8.rs:feature-error_01 }}
```

　　上面程序以错误运行结果结束：

```
error[E0382]: use of moved value: `instance`
  --> bin-hello/examples/expand/struct_u8.rs:76:24
   |
69 |     let instance = Struct(42u8);
   |         -------- move occurs because `instance` has type `struct_u8::adjoin::Struct`, which does not implement the `Copy` trait
70 |     let clone_instance = instance.clone();
71 |     let copy_instance = instance;
   |                         -------- value moved here
...
76 |     let use_instance = instance;
   |                        ^^^^^^^^ value used here after move
```

## 可变类型的派生属性`Copy`

　　对于自定义类型，要是只有复制特质`Copy`，程序编译会是什么结果？

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/expand/struct_u8.rs:feature-error_03 }}
```

　　该程序运行结果是以错误结束。而这种错误说明需要先实现克隆特质`Clone`。

```
error[E0277]: the trait bound `main::Struct: std::clone::Clone` is not satisfied
  --> bin-hello/examples/struct_u8.rs:56:14
   |
56 |     #[derive(Copy)]
   |              ^^^^ the trait `std::clone::Clone` is not implemented for `main::Struct`
```

　　尽管该程序不能编译，但是还是可以运行派生工具，其结果如下：

```
#![feature(prelude_import)]
#![allow(unused_variables)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
#[cfg(feature = "err_02")]
fn main() {
    struct Struct(u8);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Struct {}
    let instance = Struct(42u8);
    let clone_instance = instance.clone();
    let copy_instance = instance;
    let get_instance = instance;
}
```

## 分析不同类型的派生属性

　　将前面包含派生属性`Clone`和`Copy`程序进行这样的调整：把结构类型的元素类型`u8`修改为`String`，得到如下程序：

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/expand/struct_string.rs:feature-error_01 }}
```

　　上面程序运行结果如下。我们知道前面类似程序运行是正常的，为什么到这里就不能运行了呢？这是因为类型`String`是可变类型，对于所有这种形式的类型，Rust语言都没有实现其复制特质`Copy`.

```
error[E0204]: the trait `Copy` may not be implemented for this type
  --> bin-hello/examples/expand/struct_string.rs:38:21
   |
38 |     #[derive(Clone, Copy)]
   |                     ^^^^
39 |     struct Struct(String);
   |                   ------ this field does not implement `Copy`
```

　　那么什么是这种类型一般性使用方法呢？对于这种类型，仅使用Rust语言提供的克隆特质`Clone`，其代码如下：

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/expand/struct_string.rs:feature-ok }}
```

　　通过派生工具命令，展开该程序代码。注意这里克隆特质`Clone`实现，比较前面类型`Struct(u8)`程序实现，可以了解到两者实现是完全一样的。对于仅仅使用克隆特质`Clone`，两种类型`Struct(u8)`和`Struct(String)`是完全相似的。

```rust
mod struct_string {
    #![allow(unused_variables)]
    #[cfg(feature = "ok")]
    pub fn adjoin() {
        struct Struct(String);
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Struct {
            #[inline]
            fn clone(&self) -> Struct {
                match *self {
                    Struct(ref __self_0_0) => Struct(::core::clone::Clone::clone(&(*__self_0_0))),
                }
            }
        }
        let instance = Struct(String::from("Hello"));
        let clone_instance = instance.clone();
        let copy_instance = instance;
        let use_copy_instance = copy_instance;
    }
}
```

　　下面程序说明了，对于这种类型`Struct(String)`也是不能转移（move）的。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/expand/struct_string.rs:feature-error_02 }}
```

## 自定义类型`Struct<'cn>(&'cn str)`的派生属性

　　对于结构类型`Struct<'cn>(&'cn str)`的元素是字符串文字类型，只是其类型定义的表达方式上差距比较大，下面给出实现代码。在实际中经常会使用这种类型。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/expand/struct_str.rs:feature-ok }}
```

　　但是，其处理方法上与类型`Struct(u8)`是完全相同的，通过派生工具展开其代码，可以解释这个问题，把前面展开代码与下面展开代码相比较。

　　该程序派生工具运行结果如下：

```rust,no_run,noplaypen
mod struct_str {
    #![allow(unused_variables)]
    #[cfg(feature = "ok")]
    pub fn adjoin() {
        struct Struct<'cn>(&'cn str);
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<'cn> ::core::clone::Clone for Struct<'cn> {
            #[inline]
            fn clone(&self) -> Struct<'cn> {
                {
                    let _: ::core::clone::AssertParamIsClone<&'cn str>;
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<'cn> ::core::marker::Copy for Struct<'cn> {}
        let instance: Struct = Struct("Hello");
        let clone_instance = instance.clone();
        let use_copy_instance = instance;
        let use_instance = instance;
    }
}
```

## 题外话

## 参考资料
- [cargo-expand](https://github.com/dtolnay/cargo-expand)
- [borrowed-pointer-tutorial](http://smallcultfollowing.com/babysteps/blog/2012/07/17/borrowed-pointer-tutorial/)
- [book rust-for-c](https://aminb.gitbooks.io/rust-for-c/content/borrowed/index.html)
- [rust-borrowed-pointers-syntax](https://stackoverflow.com/questions/18943513/rust-borrowed-pointers-syntax)
- [fighting-the-borrow-checker-in-a-loop](https://users.rust-lang.org/t/fighting-the-borrow-checker-in-a-loop/22975)