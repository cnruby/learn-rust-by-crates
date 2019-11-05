# 克隆（Clone）和复制（Copy）

## 学习内容
- 了解和学习Rust语言引用`Reference`、类型与原始指针`Pointer`关系

## 篇目

- [复制特质`Copy`解释](#复制特质copy解释)
- [克隆特质`Clone`解释](#克隆特质clone解释)
- [复制特质`Copy`和克隆特质`Clone`区别](#复制特质copy和克隆特质clone区别)
- [表达方式不同](#表达方式不同)
- [内部实现不同](#内部实现不同)
- [类型区别不同](#类型区别不同)
- [复制特质`Copy`和克隆特质`Clone`的相互关系](#复制特质copy和克隆特质clone的相互关系)
- [复制特质`Copy`元素的数组实例](#复制特质copy元素的数组实例)
- [两种不同特质实现](#两种不同特质实现)
- [题外话](#题外话)
- [问题：什么时候我的类型应该使用复制特质`Copy`？](#问题什么时候我的类型应该使用复制特质copy)
- [问题：为什么存在克隆？](#问题为什么存在克隆)
- [参考资料](#参考资料)

## 复制特质`Copy`解释

　　[官方说明][id_01]:

> Types whose values can be duplicated simply by copying bits.

直接翻译：只需拷贝二进制数字位即可复制其值的类型。

解读：
- 通过简单的内存拷贝，实现该类型的复制。
- 含简单值的类型（POD，Pain Old Data），如数字类型、bool类型和引用类型，都是具有复制特质`Copy`的类型。
- 只有当结构性类型内部的每个项都是Copy类型时，允许实现此类型复制特质`Copy`。如数组类型、tuple类型、struct类型和enum类型，它们内部的每个项都是Copy类型。
- 当结构性类型内部的项存在不是Copy类型时，它们就不能实现该类型的复制特质。如Box类型、字符串String和向量Vec，它们内部的项不都是Copy类型。

解读延伸：
- 复制特质`Copy`没有任何可实现的方法或函数。
- 复制特质`Copy`的行为是不可重载的。

## 克隆特质`Clone`解释

　　[官方说明][id_02]:

> A common trait for the ability to explicitly duplicate an object.

直接翻译：能够显式地复制一个对象的通用特质。

解读：
- 克隆特质`Clone`是通用的，除了不确定大小或者动态大小类型之外，适合所有类型，可实现所有类型的克隆特质`Clone`。
- 只有手动调用克隆特质`Clone`方法，才能发挥其作用。作者称之为显式行为（an explicit action）。
- 克隆特质`Clone`的实现与具体类型密切相关。
- 克隆特质`Clone`有可以实现的方法。

解读延伸：
- 对于实现了复制特质`Copy`的类型，其克隆特质`Clone`与其复制特质`Copy`语义是一样的，等同于按位拷贝。
- 一般情况下，Rust语言使用克隆特质`Clone`方法来执行对象的复制。

## 复制特质`Copy`和克隆特质`Clone`区别

### 表达方式不同

　　复制特质`Copy`是以隐含方式表达，Rust语言也称之为为拷贝（copy），而克隆是一个显式行为方式表达，Rust语言也称之为移动（move）。注意：使用第一个字母大写`Copy`是指复制特质，而使用第一个字母小写（copy）是指复制特质`Copy`的行为，中文这里称之为“拷贝”或者“复制“。同理，使用第一个字母大写`Clone`是指克隆特质，而使用第一个字母小写（move）是指克隆特质`Clone`的行为，中文这里称之为“移动”或者”复制“。

```rust
// Copy：copy behavior
// The variables x and y are different instances, and both can be used
let x = 3;
let y = x;

// Clone: move behavior
// The variables x and y are different instances, and both can be used
let x = String::from("Hello");
let y = x.clone();
```

### 内部实现不同

　　复制特质`Copy`可以安全地复制类型对象的值。对于具有复制特质`Copy`的类型，编译器负责管理复制特质`Copy`类型的对象。

　　克隆特质`Clone`专为程序开发任意类型复制而设计。任何类型克隆特质`Clone`的实现可以执行创建类型所需的任意复杂方法。像正常特质一样使用及其方法调用。

### 类型区别不同

　　复制特质`Copy`适合于含值类型，如正整数u8类型。在类型u8的情况下，通过移动（move）无法来提高效率，这时候就不用再考虑移动（move）。

　　克隆特质`Clone`适合于重量级的类型，对于一种类型，两种特质都可以实现时，要是移动（move）比拷贝（copy）更有效，就尽可能使用移动（move）。

## 复制特质`Copy`和克隆特质`Clone`的相互关系

　　克隆特质`Clone`是复制特质`Copy`的父特质，因此特质`Copy`的所有内容也必须实现特质`Clone`。如果类型为特质`Copy`，则其特质`Clone`实现仅需要返回`*self`。每个特质`Copy`的类型也必须具有特质`Clone`的类型。

## 复制特质`Copy`元素的数组实例

　　从下面的实例可以看到，具有复制特质`Copy`元素的数组也是可以简单进行移动和拷贝。具有复制特质`Copy`元素的数组本质上就是复制自身，因此克隆特质`Clone`的实现可以不需要再引用直接返回自身的传递值。

{{#playpen ../../../../hello-borrowing/bin-hello/examples/use_clone_array.rs editable}}

## 两种不同特质实现

　　复制特质`Copy`是特定的编译器特征，它指示编译器，开发人员希望为该类型激活隐式复制。仅当浅表拷贝与深度拷贝等效时，此特质才可用，这样可确保不会发生内存分配作为这些隐式拷贝的一部分。复制特质`Copy`是一种复制的方法，仅可拷贝若干个内存地址字节的类型。

　　克隆特质`Clone`指示编译器，开发人员创建新的对象，且必须显式调用才对象。大多数类型（但不是全部）都可以使用它进行拷贝。克隆特质`Clone`是也一种复制方法，可以拷贝运行任意代码的类型。

## 题外话

### 问题：什么时候我的类型应该使用复制特质`Copy`？

　　一般来说，如果一种类型可以实现复制特质`Copy`，则应该去实现该类型的复制特质`Copy`。但是如果该类型将来可能变为非复制特质`Copy`类型，则最好从一开始就不要实现复制特质，以避免以后发生重大更改。

　　如果在编译代码时，警告显示，请添加复制特质`Copy`，除非有充分的理由不这样做。

### 问题：为什么存在克隆？

　　从内部分析，克隆特质`Clone`的移动（move），与复制特质`Copy`的拷贝（copy）是一样的，也有拷贝（copy），只是这种拷贝（copy）资源不能让开发人员访问。为了区分两种情况，需要克隆特质`Clone`。

　　我们知道，复制特质`Copy`的拷贝（copy）要求可以使用堆栈中字节的简单内存来复制该值。但是克隆特质`Clone`的移动（move），不仅需要拷贝（copy）在栈（Stack）上的值（即容量、长度和指向内容的指针内存地址等），而且还需要创建（create）一个新的内存储存位置来复制其具体的内容。所以，移动（move）不仅仅是拷贝（copy）还有创建（create）行为，它也称之为深度拷贝（deep copy），而仅仅只有拷贝（copy）行为称之为浅表拷贝（shallow copy）。

　　需要指出的是，Rust语言的克隆特质`Clone`的实现既可以是深度拷贝，也可以是浅表拷贝。

　　比如，字符串`String`类型对象是可复制的，即移动（move）行为，请使用克隆方法`clone()`；字符串`String`类型对象不可完成隐式复制的目的，即拷贝（copy）行为，因为这将导致发生非显而易见的内存分配。

## 参考资料
- [std/marker/trait.Copy](https://doc.rust-lang.org/std/marker/trait.Copy.html)
- [whats-the-difference-between-copy-and-clone](https://doc.rust-lang.org/std/marker/trait.Copy.html#whats-the-difference-between-copy-and-clone)
- [what-is-the-difference-between-copy-and-clone](https://stackoverflow.com/questions/31012923/what-is-the-difference-between-copy-and-clone)
- [whats-the-difference-between-trait-copy-and-clone](https://users.rust-lang.org/t/whats-the-difference-between-trait-copy-and-clone/2609)
- [when_should_my_type_be_copy](https://www.reddit.com/r/rust/comments/2xxjda/when_should_my_type_be_copy/)
- [cant-derive-copy-because-of-string](https://users.rust-lang.org/t/cant-derive-copy-because-of-string/18665/11)
- [how-do-i-implement-copy-and-clone-for-a-type-that-contains-a-string](https://stackoverflow.com/questions/38215753/how-do-i-implement-copy-and-clone-for-a-type-that-contains-a-string)
- [how-to-define-a-copyable-struct-containing-a-string](https://stackoverflow.com/questions/38304666/how-to-define-a-copyable-struct-containing-a-string)
- [Clone VS Copy](https://zhuanlan.zhihu.com/p/21730929)
- [move-clone-copy](https://jeenalee.com/2016/08/29/move-clone-copy.html)

[id_01]:https://doc.rust-lang.org/std/marker/trait.Copy.html
[id_02]:https://doc.rust-lang.org/std/clone/trait.Clone.html
