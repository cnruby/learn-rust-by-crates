# 应用篋：作为函数参数对象的生命周期

　　本节将类型`String`对象、类型`String`引用和类型`str`作为实例，分析它们作为函数参数的生命周期。

## 学习内容
- 了解和学习Rust语言含可变引用参数的函数借用实例

## 篇目

- [可变类型`String`引用的生命周期](#可变类型string引用的生命周期)　　
- [可变类型`String`对象的生命周期](#可变类型string对象的生命周期)
- [引用类型`str`对象的生命周期](#引用类型str对象的生命周期)
- [引用类型`str`对象的生命周期符](#引用类型str对象的生命周期符)
- [题外话](#题外话)
- [类型`String`与类型`str`的区别](#类型string与类型str的区别)
- [参考资料](#参考资料)


## 可变类型`String`引用的生命周期

　　先把下面程序的两个对象进行一下说明：

- 对象`instance`是类型`String`的可变对象，因为在使用`let`绑定该对象`instance`时，使用了关键词`mut`。下面将对象`instance`称之为**可变对象**；
- 对象`mut_ref`是可变对象`instance`的可变引用，因为在使用`&`绑定对象值时，使用了关键词`mut`。下面将对象`mut_ref`称之为**可变引用**；
- 对象`instance`是所有者，而对象`mut_ref`仅仅是从所有者那里租用者，它们所拥有的内容是一样的；
- 在对象`mut_ref`生命周期里，它可以操作对象`instance`所拥有的所有内容。从这个意义上说，它们是完全一样的；

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_fn/base_string.rs:feature-ok }}
```

　　上面程序的运行结果如下。从该结果可以分析该程序，无论是可变对象`instance`，还是可变引用`mut_ref`或者方法参数`input`，都是指向相同的内存地址，即可变对象`instance`的内容。这里反映了Rust语言调用函数的借用机制。

```
───────┬──────────────────────────────────────────────────────
       │ STDIN
───────┼──────────────────────────────────────────────────────
   1   │ instance = 0x7fff5a219e70
   2   │ mut_ref = 0x7fff5a219e70
   3   │ one() input = 0x7fff5a219e70
   4   │ two() input = 0x7fff5a219e70
   5   │ instance = 0x7fff5a219e70
   6   │ Hello, world!
───────┴──────────────────────────────────────────────────────
```

　　尽管主程序里，在创建可变引用`mut_ref`以后，并没有看到对可变引用`mut_ref`或者可变对象`instance`进行赋值，但是可变对象`instance`的内容一直在变化。

　　当方法借用可变引用`mut_ref`以后，还是归还了所有权到主程序，而不是把该可变引用`mut_ref`消费掉了。因此，第二个方法还可以继续使用该可变引用`mut_ref`。

　　通过给两个方法传递可变引用`mut_ref`，它们不仅可以借用其内容（这是因为绑定可变引用`mut_ref`时，使用了关键词`&`），而且还可以修改其内容（这是因为绑定可变引用`mut_ref`时，在`&`之后使用了关键词`mut`）。同时方法还将其内容返回到可变引用`mut_ref`，即可变对象`instance`。

　　必须指出的是，方法参数也必须是可变引用，与调用方法时的对象类型是完全一致的，不然就会出现程序编译错误。下面程序就是说明这个问题。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_fn/base_string.rs:feature-error_02 }}
```

## 可变类型`String`对象的生命周期

　　下面程序实例是将可变类型`String`对象直接作为方法参数使用，就是方法直接借用可变类型`String`对象，而不是其引用。下面实例可以正常运行，但是，它有什么问题呢？

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_fn/base_string.rs:feature-okay }}
```

　　上面程序的运行结果如下。当可变对象`instance`作为参数传递到方法以后，该对象就被该方法消费掉了。从它们的内存地址信息可以看到，它们是不一样的。

```
───────┬──────────────────────────────────────────────────────
       │ STDIN
───────┼──────────────────────────────────────────────────────
   1   │ instance = 0x7fff53e5cf50
   2   │ input = 0x7fff53e5cfc0
   3   │ input = Hello, world
───────┴──────────────────────────────────────────────────────
```

　　主程序可变对象`instance`是不是真的被方法消费掉了呢？通过编译下面程序，编译器会告诉我们答案。下面出现编译错误说明了可变对象`instance`不转移了，就是被方法消费掉了。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_fn/base_string.rs:feature-error_01 }}
```

## 引用类型`str`对象的生命周期

　　上面说明了类型`String`对象的可变引用，不仅可以被借用，而且还可以被修改。但是引用类型`str`对象只能被借用，而不能被修改。因为它不能被修改，所以它返回时还是它本身。

　　引用类型`str`对象作为方法参数，主要目的是传递其内容。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_fn/base_str.rs:feature-okey }}
```

　　上面程序运行结果可以看到，尽管在方法里参数`input`调用了方法`to_ascii_uppercase()`，但是，不仅参数`input`的内存地址始终与主程序输入对象的内存地址是完全一样的，而且其内容到最后还是原来的内容。这种调用了方法`to_ascii_uppercase()`并不是修改其内容，而是创建一个新对象，在新对象里，可以看到其内容发生了变化。

```
───────┬──────────────────────────────────────────────────────
       │ STDIN
───────┼──────────────────────────────────────────────────────
   1   │ after instance change = 0x7fff5c99ce48
   2   │ 
   3   │ before call one instance = 0x7fff5c99ce48
   4   │ one() input = 0x7f94a9c00330
   5   │ one() ret_input = 0x7fff5c99cc70
   6   │ one() ret_input = HELLO
   7   │ one() input = 0x7f94a9c00330
   8   │ one() input = Hello
   9   │ after call one instance = 0x7fff5c99ce48
  10   │ instance = Hello
───────┴──────────────────────────────────────────────────────
```

## 引用类型`str`对象的生命周期符

　　通过下面实例，将引出生命周期符的概念。下面程序编译会出现错误："缺少生命周期符"。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_fn/base_str.rs:feature-error_07 }}
```

　　使用函数生命周期符，一般情况下，是因为函数返回对象存在引用对象。上面程序怎么样解决呢？下面程序将上面程序问题得到解决。之所以上面程序不能编译成功，是因为该方法存在两个引用，它们都有自己的生命周期，而结果只有一个生命周期，编译器不知道应该使用哪一个生命周期。生命周期符是统一这两个引用的生命周期。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_fn/base_str.rs:feature-ok }}
```

　　在上面程序里，生命周期符`'de`是由两部分：前面单引号`'`是必须的；而后面与对象命名法则（snake_case）是一样的。一般使用`'a`。

## 题外话

### 类型`String`与类型`str`的区别

　　字符串类型对象是可变的。而类型`str`，通常以`＆str`表示对象是不可更改的，它只是对字符串的内容储存。

　　下面程序通过两个方法`make_ascii_uppercase()`与`to_ascii_uppercase()`说明，两种类型对象使用时的区别。方法`make_ascii_uppercase()`是直接修改其对象内容，得到其结果；而方法`to_ascii_uppercase()`并不是直接修改本身引用对象，而是创建了一个新类型`String`对象作为返回结果。

```rust
fn main () {
    let mut instance = String::from("Hello");
    println!("instance = {:p}", &instance);
    instance.make_ascii_uppercase();
    println!("{}", instance);
    println!("instance = {:p}", &instance);
    println!();

    let instance = "Hello";
    println!("instance = {:p}", instance);
    instance.to_ascii_uppercase();
    println!("instance = {}", instance);
    println!();

    let instance = "Hello";
    println!("instance = {:p}", instance);
    let ret_instance = instance.to_ascii_uppercase();
    println!("instance = {}", instance);
    println!("ret_instance = {:p}", &ret_instance);
    println!("ret_instance = {}", ret_instance);
}
```



## 参考资料
- [rust-ownership](https://hellocode.dev/rust-ownership)
- [how-can-i-create-a-static-string-in-rust](https://stackoverflow.com/questions/55977067/how-can-i-create-a-static-string-in-rust)
- [what-are-the-differences-between-rusts-string-and-str](https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str)
- [rust-lifetimes-a-high-wall-for-rust-newbies](https://dev.to/takaakifuruse/rust-lifetimes-a-high-wall-for-rust-newbies-3ap)
- [book lifetimes](https://doc.rust-lang.org/1.9.0/book/lifetimes.html)
- [book naming](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)

