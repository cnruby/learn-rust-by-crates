# 应用篋：移动关键词`move`借用实例

## 学习内容
- 了解和学习Rust语言移动关键词`move`借用实例

## 篇目

- [移动关键词`move`](#移动关键词move)
- [借用机制代码实例](#借用机制代码实例)
- [题外话](#题外话)
- [参考资料](#参考资料)

## 移动关键词`move`

　　移动关键词`move`是转移对象的所有权，实际上是使得该对象失去所有权。类似于函数调用结束以后，会自动转移类型对象的所有权，但不包括引用类型对象。

　　通过比较下面两个程序，可以了解到移动关键词`move`的表达方法。

　　通过比较下面两个程序，它们唯一区别就是第二个程序多了关键词`move`。实际运行它们，可以看到，第一个程序正常，而第二个程序出现错误，借此可以理解到移动关键词`move`的作用。即使是使用对象的引用，也会使得其对象本身失去所有权，更不用说使用对象本身。

　　连使用对象的引用，也失去其对象本身的所有权，这是否与借用机制不一致了？网上有人提出这样的问题。我们下面使用了一个技巧，来解决这个问题。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/closure/kw_move.rs:feature-cp }}
```

　　在下面程序里，也有定义闭包的语句。即使仅使用定义语句，也会出现错误。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/closure/kw_move.rs:feature-err }}
```

## 借用机制代码实例

　　在使用移动关键词`move`前提下，这是因为有时候需要使得一些变量失去其所有权，而另外一些变量不能失去其所有权。

　　为了使得一些变量不能失去其所有权，而又要借用该变量。解决办法是这样的：为了使得一个对象的借用有效，这时候需要在闭包以前先把该对象引用绑定一个变量，然后把绑定引用对象变量使用到闭包里。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/closure/move_vec.rs:feature-ok }}
```

## 题外话

## 参考资料
- [What’s the purpose of the move keyword?](https://users.rust-lang.org/t/whats-the-purpose-of-the-move-keyword/16160)
- [What are move semantics in Rust?](https://stackoverflow.com/questions/30288782/what-are-move-semantics-in-rust)
- [std keyword move](https://doc.rust-lang.org/std/keyword.move.html)