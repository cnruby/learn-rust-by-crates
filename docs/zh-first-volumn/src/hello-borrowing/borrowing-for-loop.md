# 应用篋：循环`for`语句借用实例

　　在Rust语言里，固定大小类型对象与可变大小类型对象的处理方式常常是不一样的。下面通过固定数组和可变数组的实例说明这个问题。

## 学习内容
- 了解和学习Rust语言循环`for`语句借用实例

## 篇目
　　
- [固定数组的借用错误实例](#固定数组的借用错误实例)
- [固定数组的借用机制实例](#固定数组的借用机制实例)
- [可变数组的简单借用错误实例](#可变数组的简单借用错误实例)
- [可变数组的借用机制实例](#可变数组的借用机制实例)
- [迭代方法`iter()`和`into_iter()`区别](#迭代方法iter和into_iter区别)
- [可变数组的迭代借用机制实例](#可变数组的迭代借用机制实例)
- [题外话](#题外话)
- [参考资料](#参考资料)

## 固定数组的借用错误实例

　　这是一个说明固定数组类型`[T;N]`借用错误的实例。在循环语句里，不能将固定数组对象`instance`作为`for`语句的迭代部分，这是因为固定数组对象本身不是一个迭代器。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/for_arr.rs:feature-err }}
```

　　在编译该程序以后，编译器给出有用的信息提示，如下所示：

```bash
...
borrow the array with `&` or call `.iter()` on it to iterate over it(`instance`)
...
= note: arrays are not iterators, but slices like the following are: `&[1, 2, 3]`
...
```

　　这些信息告诉我们：

- 借用方法可以使用`&`或调用方法`.iter()`；
- 固定数组`array`不是迭代器；

　　下面利用这些信息，来解决固定数组的借用问题。

## 固定数组的借用机制实例

　　在循环`for`语句里，对于数组类型，Rust语言提供里几种借用方法。但是对于固定数组和可变数组类型，其方法上略有不同。这里先说明固定数组类型的借用机制实例。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/for_arr.rs:feature-ok }}
```
　　在方法`main()`里，一共有五段代码，说明如下：

- 中间三段代码就是固定数组的借用方法，三种方法都可以使用；
- 对于固定数组，三种方法的思路也是一致的，在循环`for`语句的迭代过程中，通过引用对象遍历其每一个项进行迭代；
- 最后一行代码可以通过编译，说明了该对象`instance`还是可以使用的，其生命周期还没有结束，同时可以验证上面借用方法都是可行的。
- 从该程序下面的输出结果，可以看到，它们的内存地址都是指向相同的原始指针；
- 当中有些宏`print!`或者`println!`只是为了输出结果可以更加容易理解；

　　该程序输出结果如下：

```bash
───────┬───────────────────────────────────────────────────────────────────────
       │ STDIN
───────┼───────────────────────────────────────────────────────────────────────
   1   │      instance ref = 0x7fff567a7c5d
   2   │           for ref = 0x7fff567a7c5d 0x7fff567a7c5e 0x7fff567a7c5f 
   3   │       for .iter() = 0x7fff567a7c5d 0x7fff567a7c5e 0x7fff567a7c5f 
   4   │  for .into_iter() = 0x7fff567a7c5d 0x7fff567a7c5e 0x7fff567a7c5f 
   5   │      instance arr = [1, 2, 3]
───────┴───────────────────────────────────────────────────────────────────────
```

## 可变数组的简单借用错误实例

　　下面是一个可变数组的循环`for`语句实例。注意，这个实例仅仅说明，与固定数组错误不同的是，可变数组对象是一个迭代器。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/for_vec.rs:feature-cp }}
```

　　可能有人发现，上面程序的下面输出结果内存地址不一样，这是为什么，是因为一旦可变数组对象作为迭代器，在循环`for`语句被使用以后，该对象就被转移了（moved），自然它们的内存地址就不一样了。

```bash
───────┬────────────────────────────────────────────────────────────────────────
       │ STDIN
───────┼────────────────────────────────────────────────────────────────────────
   1   │      instance raw = 0x7f8779500b80
   2   │            for u8 = 0x7fff54a6b05f 0x7fff54a6b05f 0x7fff54a6b05f 
───────┴────────────────────────────────────────────────────────────────────────
```

　　接下来才是一个可变数组的借用错误实例，只是在上面实例中多增加了最后一行代码。只是一旦将其作为迭代器，其生命周期就解释了。这样就出现下面出现所谓的“借用错误”。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/for_vec.rs:feature-err }}
```

## 可变数组的借用机制实例

　　下面我们使用可变数组对象作为迭代器，解决可变数组的借用问题。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/for_vec.rs:feature-ok }}
```

　　上面实例在借用机制方法下，可以看到该程序输出的结果，它们内存地址是相同的，这是因为它们仅仅借用而已。

　　该程序输出结果如下：

```bash
───────┬────────────────────────────────────────────────────────────────────────
       │ STDIN
───────┼────────────────────────────────────────────────────────────────────────
   1   │      instance raw = 0x7f8a504035e0
   2   │           for ref = 0x7f8a504035e0 0x7f8a504035e1 0x7f8a504035e2 
   3   │      instance vec = [1, 2, 3]
───────┴────────────────────────────────────────────────────────────────────────
```

## 迭代方法`iter()`和`into_iter()`区别

　　在上面固定数组对象实例中，我们使用了迭代方法`into_iter()`，运行是正常的，为什么到了可变数组对象就不可编译了呢？

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/for_vec_iter.rs:feature-err }}
```

　　下面分析其原因。Rust语言提供了三个迭代方法。这里代码仅仅说明其中两个方法`iter()`和`into_iter()`，但是这里把三个方法功能说明如下：

- 方法`iter()`通过迭代器的引用对象，遍历其每一个项；
- 方法`into_iter()`将迭代器移至新迭代器，然后通过迭代器的对象，遍历其每一个项；
- 方法`iter_mut()`通过迭代器的可变引用对象，遍历其每一个项；

　　从上面三个方法的功能，可以知道，方法`into_iter()`的迭代手段不是借用，而是直接消费了。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/for_vec_iter.rs:feature-cp }}
```

　　通过下面实例输出结果来分析方法`into_iter()`：

- 循环语句变量`item`不是引用对象`&u8`，而是类型`u8`对象；
- 要是把上面程序循环语句的注视去掉，就会出现错误，这再次说明，变量`item`是类型`u8`对象，同时也说明了方法`into_iter()`使用遍历其每一个项都是类型`u8`对象，而不是方法`iter()`一样的类型`&u8`引用对象；
- 循环语句内外内存地址是不同的；

　　第二个程序输出结果如下：

```bash
───────┬───────────────────────────────────────────────────────────────────────
       │ STDIN
───────┼───────────────────────────────────────────────────────────────────────
   1   │ instance raw = 0x7fb3604035e0
   2   │for into_iter = 0x7fff5ad6705f 0x7fff5ad6705f 0x7fff5ad6705f 
───────┴───────────────────────────────────────────────────────────────────────
```

## 可变数组的迭代借用机制实例

　　通过上面分析，下面实例代码就比较简单了。它是使用方法`iter()`，实现循环语句`for`的迭代借用机制。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/for_vec_iter.rs:feature-ok }}
```

　　该程序输出结果如下。从中可以看到这些原始指针的内存地址都是一样的，它们指向相同的可变数组的数据。

```bash
───────┬───────────────────────────────────────────────────────────────────────
       │ STDIN
───────┼───────────────────────────────────────────────────────────────────────
   1   │ instance raw = 0x7f90084035e0
   2   │  for vec ref = 0x7f90084035e0 0x7f90084035e1 0x7f90084035e2 
   3   │ for vec iter = 0x7f90084035e0 0x7f90084035e1 0x7f90084035e2 
   4   │ instance vec = [1, 2, 3]
───────┴───────────────────────────────────────────────────────────────────────
```

![image](../../hello-borrowing/images/hello_borrowing-12_vec_for_loop.png)

## 题外话

### 关键词`ref`与引用符`&`

{{#playpen ../../../../hello-borrowing/bin-hello/examples/use_ref_and.rs}}


## 参考资料
- [What is the difference between iter and into_iter?](https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter)
- [Effectively Using Iterators In Rust](https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html)
- [How to iterate over and filter an array?](https://stackoverflow.com/questions/30467085/how-to-iterate-over-and-filter-an-array)
- [How to iterate over an array of integers?](https://stackoverflow.com/questions/28378407/how-to-iterate-over-an-array-of-integers)
- [rust-patterns-ref](http://xion.io/post/code/rust-patterns-ref.html)