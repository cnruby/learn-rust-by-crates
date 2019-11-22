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

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_immut/mut_string.rs:feature-okey }}
```



```
// https://internals.rust-lang.org/t/allow-mut-value-not-just-mut-reference/7424/2
let mut a = &mut x;
```

You can also do let mut a = &mut x; which would mean you could later change a to be a mutable reference to another variable.
A mut after let means the variable can be changed. This is not equivalent to a mut at any place right of the =.


https://hermanradtke.com/2015/06/09/strategies-for-solving-cannot-move-out-of-borrowing-errors-in-rust.html


borrow-clone.rs


Can you spot the difference? Now, on line 4, we call the other function with a clone of the value held by the hello variable. Obviously this is not the most efficient way to write your program, and it can be risky if you don't know the size of the values which are going to be getting cloned. Also, if your functions mutate the value in some way then this is not a good solution:

您看得出来差别吗？ 现在，在第4行，我们用hello变量保存的值的克隆调用另一个函数。 显然，这不是编写程序的最有效方法，如果您不知道将要克隆的值的大小，则可能会有风险。 另外，如果您的函数以某种方式更改了值，那么这不是一个好的解决方案：

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


- [what-is-right-ways-to-concat-strings](https://users.rust-lang.org/t/what-is-right-ways-to-concat-strings/3780/1)
- [best-way-to-do-string-concatenation-in-2019-status-quo](https://users.rust-lang.org/t/best-way-to-do-string-concatenation-in-2019-status-quo/24004)
- [how-do-i-concatenate-strings](https://stackoverflow.com/questions/30154541/how-do-i-concatenate-strings)
- 

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
- 