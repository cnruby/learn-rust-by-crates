# 应用篋：绑定引用的可变对象借用实例

　　通过这一节学习，通过类型`String`的不同借用实现，理解绑定引用的可变对象的借用方法，这种引用也是两种类型：固定引用和可变引用，但是它们绑定的对象是可变的。

## 学习内容
- 了解和学习绑定固定引用和可变引用的可变对象借用实例

## 篇目

- [可变对象及其可变引用关系](#可变对象及其可变引用关系)
- [绑定固定引用的可变对象](#绑定固定引用的可变对象)
- [绑定可变引用的可变对象](#绑定可变引用的可变对象)
- [题外话](#题外话)
- [连接字符串的正确方法](#连接字符串的正确方法)
- [参考资料](#参考资料)

## 可变对象及其可变引用关系

　　下面将列出可变对象及其固定和可变引用可能关系。关于下面程序的前面两段关系，在前面已经探讨过了。本节主要探讨其他的关系。

```rust
let mut instance = String::from("Hello");
let ref_immut = &instance;
let ref_immut :&String = &instance;

let mut instance = String::from("Hello");
let ref_mut = &mut instance;
let ref_mut :&mut String = &mut instance;

let mut instance = String::from("Hello");
let mut mut_ref_immut = &instance;
let mut mut_ref_immut :&String = &instance;

let mut instance = String::from("Hello");
let mut mut_ref_mut = &mut instance;
let mut mut_ref_mut :&mut String = &mut instance;
```

　　对于可变对象，下面两段代码创建了它们相关的`&str`引用对象。其中第一段代码前面已经说明过了。

```rust
let mut instance = String::from("Hello");
let ref_immut :&str = &instance;

let mut instance = String::from("Hello");
let mut mut_ref_immut :&str = &instance;
```


## 绑定固定引用的可变对象

　　下面程序的方法的特点是，使用引用类型`＆str`对象作为方法参数，而使用类型`String`对象作为函数返回。Rust语言标准库一部分函数使用了这种函数表达形式。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_immut/ref_str.rs:feature-ok }}
```

　　上面程序里存在下面一行代码。这行代码对象`mut_ref_immut`具有如下属性：

1. 它是可变对象`instance`的固定引用作为对象值，而不是可变引用作为对象值；
1. 它是类型`&str`对象，而不是类型`String`的对象；
1. 它是一个可变对象，即该对象可以多次赋值，这种对象`mut_ref_immut`也称之为"固定引用的可变对象"、”引用可变对象“或者”引用“；

> let mut mut_ref_immut :&str = &instance;

　　在上面程序里，方法需要类型就是对象`mut_ref_immut`的类型，就是类型`&str`对象，并且是固定引用。

　　当第一次调用方法以后，方法返回值就赋予了对象`tmp_instance`，此时，对象`mut_ref_immut`的所有权还是一直有效的，只有当对象`tmp_instance`赋值给对象`instance`时，其所有权被对象`instance`重新拿收回了。当然引用对象`mut_ref_immut`内容也被失去了，但是引用对象`mut_ref_immut`本身还是有效的，这是引用的可变对象的特点。

　　主程序里需要使用对象`mut_ref_immut`的可变性。这是因为每一次调用方法，且同时引用对象`mut_ref_immut`的所有权被收回以后，需要重新给可变对象`mut_ref_immut`进行赋值。一旦给对象`mut_ref_immut`赋值以后，对象`mut_ref_immut`就是全新的内容和内存地址，这种对象是传统意义上的变量，变化其量。

　　无论是对象`instance`，还是引用`mut_ref_immut`，在这个程序里，它们的生命周期一直到方法`main()`。

　　引用的可变对象与引用的固定对象不同的是，当前者被一个对象收回了所有权时，这仅仅是收回了该对象的内容，而不是其实体。它的实体还是存在的，且可以赋值使其重新有效。尽管该对象没有赋值以前，不能使用它。而后者，一旦收回其所有权以后，其实体本身也无效了。

　　从下面程序运行结果也可以知道，对象`instance`和引用`mut_ref_immut`的生命周期。

```
───────┬──────────────────────────────────────────────────────
       │ STDIN
───────┼──────────────────────────────────────────────────────
   1   │ instance = 0x7fff5ee45c58
   2   │ 
   3   │ before call one() mut_immut_ref = 0x7fa320c03710
   4   │ one() input = 0x7fa320c03710
   5   │ after call one() mut_immut_ref = 0x7fa320c03710
   6   │ instance = "Hello, world"
   7   │ 
   8   │ mut_immut_ref = 0x7fa320c03720
   9   │ two() input = 0x7fa320c03720
  10   │ instance = "Hello, world!"
  11   │ 
  12   │ instance = 0x7fff5ee45c58
───────┴──────────────────────────────────────────────────────
```

## 绑定可变引用的可变对象

> let mut mut_and_mut_ref = &mut mut_instance;

　　与前面了解到的绑定可变引用的固定对象与绑定固定引用的可变对象实例类似，这种如上Rust语句的对象`mut_and_mut_ref`，无论是所引用对象`mut_instance`值，还是对象`mut_and_mut_ref`本身值都是可变的。

　　下面是如何使用绑定可变引用的可变对象具体实例：

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_immut/mut_string.rs:feature-okey }}
```

　　在这里之所以使用可变对象属性，是因为在第一次调用方法`one()`以后，方法将对象`mut_ref`消费掉了，尽管对象`mut_ref`本身还是存在的，但其内容已经不在了，要是直接再次调用方法`two()`，就会出现错误。这也说明了对象`mut_ref`的值是可变的。

　　要是不存在对象`mut_ref`第二次的赋值，就会出现下面借用错误。



　　在这里之所以使用需要绑定可变引用属性，是因为调用方法的参数是可变引用。方法参数是可变引用，说明该引用将会被修改。从方法功能可以看到，参数是被修改了。

　　通过上面了解到绑定可变引用的可变对象，不仅对象`mut_ref`本身内容是可修改的，即，之后可以将其更改为另一个对象或者其本身的可变引用，而且其值的对象`mut_instance`内容也是可修改的，即，之后可以将其更改为任何字符串文字内容。

　　最后非常简单地阐述类型`Box<T>`的作用，类型`Box<T>`可以将任何类型`T`打包起来，方便其打包类型`T`进行传输或者处理等，比如，要是一组不同类型都打包成类型`Box<T>`以后，就可以作为一个数组一起处理，另外这是一种引用数据类型。

　　可以把类型`Box<T>`与`T`理解为类型`String`与`str`的类似关系，前者类型`String`存在可以大量操作其内容的功能，而后者主要的内容储存功能，两种各自具有不同的功能。当然类型`Box<T>`主要的解压功能，而后者是数据结构功能。

## 题外话

### 连接字符串的正确方法

```rust
let input = "Hello";

let result :String = [input, "!"].join("");
println!("result = {}", result);

let result :String = [input, "!"].concat();
println!("result = {}", result);

let result :String = format!("{}{}", input, "?");
println!("result = {}", result);
```


## 最主要参考资料
- [rust-ownership](https://hellocode.dev/rust-ownership)
- [passing-a-string-by-reference-and-manipulate-the-string](https://stackoverflow.com/questions/26151324/passing-a-string-by-reference-and-manipulate-the-string)

## 参考资料
- [string-concatenation-in-rust-is-not-tivial](http://dnsh.io/music/2016/10/06/string-concatenation-in-rust-is-not-tivial/)
- [how-do-i-str-string](https://blog.mgattozzi.dev/how-do-i-str-string/)
- [what-is-the-idiomatic-way-to-convert-str-to-string](https://users.rust-lang.org/t/what-is-the-idiomatic-way-to-convert-str-to-string/12160/1)
- [to-string-vs-to-owned-for-string-literals](https://users.rust-lang.org/t/to-string-vs-to-owned-for-string-literals/1441)
- [return-str-instead-of-stdborrowcow-str](https://stackoverflow.com/questions/42248444/return-str-instead-of-stdborrowcow-str)
- [strategies-for-solving-cannot-move-out-of-borrowing-errors-in-rust](https://hermanradtke.com/2015/06/09/strategies-for-solving-cannot-move-out-of-borrowing-errors-in-rust.html)
- [best-way-to-do-string-concatenation-in-2019-status-quo](https://users.rust-lang.org/t/best-way-to-do-string-concatenation-in-2019-status-quo/24004/4)
- [what-is-right-ways-to-concat-strings](https://users.rust-lang.org/t/what-is-right-ways-to-concat-strings/3780)
- [allow-mut-value-not-just-mut-reference](https://internals.rust-lang.org/t/allow-mut-value-not-just-mut-reference/7424/2)
- [strategies-for-solving-cannot-move-out-of-borrowing-errors-in-rust](https://hermanradtke.com/2015/06/09/strategies-for-solving-cannot-move-out-of-borrowing-errors-in-rust.html)

## 参考资料：题外话

- [what-is-right-ways-to-concat-strings](https://users.rust-lang.org/t/what-is-right-ways-to-concat-strings/3780/1)
- [best-way-to-do-string-concatenation-in-2019-status-quo](https://users.rust-lang.org/t/best-way-to-do-string-concatenation-in-2019-status-quo/24004)
- [how-do-i-concatenate-strings](https://stackoverflow.com/questions/30154541/how-do-i-concatenate-strings)
