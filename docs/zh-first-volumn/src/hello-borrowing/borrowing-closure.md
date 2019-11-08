# 应用篋：闭包借用实例

## 学习内容
- 了解和学习Rust语言闭包借用实例

## 篇目

- [理解Rust语言闭包](#理解Rust语言闭包)
- [闭包实例](#闭包实例)
- [未实现复制特质`Copy`类型的借用实例](#未实现复制特质copy类型的借用实例)
- [借用机制代码实例](#借用机制代码实例)
- [题外话](#题外话)
- [题外话](#题外话)
- [参考资料](#参考资料)

## 理解Rust语言闭包

　　闭包（closure）是现代计算机语言发展起来的重要技术，它延伸了传统计算机语言函数的概念。Rust语言中的闭包（也称为lambda表达式或lambda，）是可以捕获封闭环境的函数。传统函数就是一个独立的和重复使用的功能性代码单位，它通过被调用而与其它代码进行交互。而闭包不仅可以被其它代码调用；而且其代码块本身也可以直接与其封闭环境内的代码进行交互，捕获外部环境变量的能力，而这一点传统函数没有这样的功能。

　　闭包与函数比较说明：

- 闭包可以捕获外部环境变量的能力，而函数不能；
- 传递到闭包的输入参数。使用`||`，而不是像函数`()`一样；
- 闭包代码块也使用函数一样的`{}`，但是函数是强制性使用，而闭包在一些情况下是可选性使用；
- 输入参数的类型也是可选的，而非函数是强制的；
- 闭包不存在闭包名，而函数不能，甚至闭包连绑定变量也可以不要；
- 调用闭包不仅与函数有完全一样的方式，而且还存在闭包自身特殊方式；
- 闭包输出的表达方式与函数完全一样；
- 闭包返回方式与函数完全一样；

　　将上面比较说明，对照下面实例代码。方法`main()`的第二行代码，第一，定义了闭包，且绑定到变量`i_am_closure`，可以理解为“闭包名”；第二，把第一行代码这种外部变量`outside_closure`应用到闭包代码块内部；第三，输入参数`input_var`省略了类型名称，之所以可以省略，是因为第一行代码定义了输入参数的类型。第三行代码里存在调用第二行定义的闭包代码。

```rust
fn main() {
    let outside_closure = 21u8;
    let i_am_closure = |input_var| -> u8 { input_var + outside_closure };
    println!("i_am_closure: {}", i_am_closure(outside_closure));
}
```

## 闭包实例

　　通过下面实例来进一步理解闭包使用方法。这个实例看起来比较抽象，这一行代码连一个字母都没有。但这是理解闭包很好的代码。

```rust
fn main(){
    (|__:()|->(){__})(())
}
```

　　通过下面程序代码逐步展开来解析上面程序的含义。程序里每一段代码都是一样的功能，从上到下还原到可以比较理解的代码。

　　需要说明的是，在方法`main()`里，前面四段代码，既定义了闭包，同时也调用了闭包。只有第五段代码定义与调用分成了两个语句。

{{ #playpen ../../../../hello-borrowing/bin-hello/examples/use_closure.rs editable}}

## 未实现复制特质`Copy`类型的借用实例

　　下面代码说明字符串类型`String`对象作为闭包参数使用以后，其生命周期结束的实例。变量作为闭包参数使用以后，在闭包里，进行了一次变量的复制，导致了Rust语言所示的移动`move`，实际上，在闭包完成调用以后，该变量从内存里被剔除掉了。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/closure_string.rs:feature-err }}
```

## 借用机制代码实例

　　解决上面实例的方法，与前面一节函数方法一样，使用引用类型对象。传递到闭包里，而不是类型字符串对象。引用变量对象仅仅是利用了类型字符串对象的内容而已，闭包本身实际上没有与类型字符串对象发生联系。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/closure_string.rs:feature-ok }}
```

## 题外话

## 参考资料
- [www.cs.brandeis.edu tuples](https://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/tuples.html)
- [Understanding the different types of closures in Rust](https://medium.com/swlh/understanding-closures-in-rust-21f286ed1759)
- [Why Rust Closures are (Somewhat) Hard](https://stevedonovan.github.io/rustifications/2018/08/18/rust-closures-are-hard.html)
- [rust-by-example closures](https://doc.rust-lang.org/rust-by-example/fn/closures.html)
- 