# 共享篋：程序结构和代码解释

　　在本节里，了解两个模块`mod_bare`和`mod_trait`结构和代码实现。

## 学习内容
- 了解和学习不同结构类型关键词`struct`实现方法

## 篇目
- [基于结构类型的实现](#基于结构类型的实现)
- [基于衔接特质的实现](#基于衔接特质的实现)
- [题外话](#题外话)
- [学习理解编译错误](#学习理解编译错误)
- [参考资料](#参考资料)

## 基于结构类型的实现

### 模块`mod_bare`结构
![image](../../images/hello_mod_trait_02_mod_bar.png)

### 模块`mod_bare`代码

https://doc.rust-lang.org/stable/error-index.html#E0038

{{#playpen ../../../../hello-mod-trait/lib-hello/src/mod_bare/mod.rs editable}}

## 基于衔接特质的实现

### 模块`mod_trait`结构

![image](../../images/hello_mod_trait_01_mod_trait.png)

### 模块`mod_trait`代码

　　两个类型的函数new()是通过属性值，实现创建其类型的实例，而特质方法get_object()是通过其类型本身实例，实现创建一个新的类型实例。

{{#playpen ../../../../hello-mod-trait/lib-hello/src/lib.rs editable}}

## 题外话

### 学习理解编译错误

　　在程序文件`src/lib.rs`里，使用关键词`trait`定义衔接特质`TraitCanal`代码块的第一行代码注释掉，而第二行代码去掉注释，一旦执行编译，就会出现下面错误信息：`error[E0038]`。

    error[E0038]: the trait `mod_trait::TraitCanal` cannot be made into
    an object
    --> lib-hello/src/mod_dynamic_fn.rs:4:1
    |
    4 | pub fn get_dynamic_trait_ref(canal: &dyn TraitCanal) -> (u32) {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ t
    he trait `mod_trait::TraitCanal` cannot be made into an object
    |
    = note: method `init` references the `Self` type in its arguments
    or return type

　　下面链接里的`E0038`就是该错误编号，点击下面链接就可以了解到错误的原因信息。

[https://doc.rust-lang.org/stable/error-index.html#E0038](https://doc.rust-lang.org/stable/error-index.html#E0038)

## 参考资料
- [Rust Compiler Error Index](https://doc.rust-lang.org/stable/error-index.html)
- [std Sized](https://doc.rust-lang.org/std/marker/trait.Sized.html)
- [why-is-the-sized-bound-necessary-in-this-trait](https://stackoverflow.com/questions/30938499/why-is-the-sized-bound-necessary-in-this-trait)










