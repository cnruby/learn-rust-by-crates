# 应用篋：循环for语句可变借用实例


## 学习内容
- 了解和学习循环for语句可变借用实例

## 篇目

- [方法`into_iter()`与关键词`mut`的组合结构](#方法into_iter与关键词mut的组合结构)
- [方法`iter_mut()`实例](#方法iter_mut实例)
- [题外话](#题外话)
- [方法`next()`实例](#方法next实例)
- [方法`enumerate()`实例](#方法enumerate实例)
- [参考资料](#参考资料)


iter() iterates over the items by reference
into_iter() iterates over the items, moving them into the new scope
iter_mut() iterates over the items, giving a mutable reference to each item


## 方法`into_iter()`与关键词`mut`的组合结构

　　对于具有迭代器`Iterator`的对象而言，方法`into_iter()`是完全依赖于其对象类型，且遍历其所有的元素本身。下面通过实例再回顾一下这种方法。

　　如下面程序，对象`instance`的类型是`Vec<u8>`，这样其元素是原始类型`u8`，那么方法就遍历其所有原始类型`u8`的元素。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/for_loop/for_into_iter.rs:feature-okay }}
```

　　其实，上面程序从表面上，方法`into_iter()`，既看不能让人看到其遍历的元素类型，也看不能让人知道其已经消费了对象`instance`。

　　对于第一个问题，为了简单快速了解其元素类型，可以使用上面程序注释掉的宏方法`println!()`，要是编译没有错误，就说明对象`item`是引用类型，否则就是非引用类型。

　　对于第二个问题，只要在上面程序基础上，在其最后增加一行宏方法`println!()`代码，如下所示。只要编译该程序，就会出现借用编译错误。说明方法`into_iter()`已经把对象`instance`消费掉了。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/for_loop/for_into_iter.rs:feature-error_01 }}
```

　　为了使得对象`instance`在调用方法`into_iter()`以后，且修改其元素，而对象`instance`依然存在，需要借用可变对象`instance`。

　　需要指出的是，即使对象`instance`是可变的，上面程序的方法元素本身也是不可变对象。

　　对于方法`into_iter()`，要是对象类型是`&Vec<u8>`，其元素是引用类型`&u8`，那么方法就遍历其所有引用类型`&u8`的元素。

　　下面程序可变对象`instance`的类型是`&mut Vec<u8>`，其元素是可变引用类型`&mut u8`，那么方法就遍历其所有可变引用类型`&mut u8`的元素。

　　下面程序是这里重点解释的结构。三个部分缺一不可：对象`instance`本身是可变的；其引用本身也是可变的；使用引用作为方法`into_iter()`的对象。这样可以实现对象`instance`内容的修改。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/for_loop/for_into_iter.rs:feature-cp }}
```

　　上面程序里，引用对象`ref_instance`的类型是一种默认情况，即绑定对象`ref_instance`时，其类型写与不写是一样的意思。但是下面程序的引用对象`ref_instance`的类型可以不是默认的，而是人为绑定的，这是程序第二段代码的第一行代码。当然程序第二段代码的第二行也是一种默认绑定方式。

　　对象`instance`的类型是`&mut [u8]`，其元素是引用类型`&mut u8`，那么方法就遍历其所有引用类型`&mut u8`的元素。这种情况可以把`&Vec<u8>`和`＆[u8]`看作为类似于`&String`和`＆str`。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/for_loop/for_into_iter.rs:feature-okey }}
```

## 方法`iter_mut()`实例

> 方法`iter_mut()` = 方法`into_iter()`与关键词`mut`的组合结构

　　从上面这行文字可以了解到，接下来说明方法`iter_mut()`其实就是上面程序代码的简化。

　　通过下面程序使用方法`iter_mut()`，可以了解到它涉及到下面几方面内容：

- 方法`iter_mut()`遍历其元素都是引用；
- 方法`iter_mut()`遍历其元素都是可变引用；
- 在调用方法`iter_mut()`以后，对象`instance`依然是存在的；
- 方法`iter_mut()`隐藏了一个可变借用对象；

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/for_loop/for_iter_mut.rs:feature-ok }}
```

## 题外话

### 方法`enumerate()`实例

　　常常希望通过数组的顺序号访问数组。第一个程序说明了这种想法的解决方案：

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/for_loop/for_enumerate.rs:feature-cp }}
```

　　实现这种想法的比较标准方法是使用标准库方法`enumerate()`。这个方法不仅遍历了数组的顺序号。而且还遍历其所有元素：

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/for_loop/for_enumerate.rs:feature-okey }}
```

　　第三种是最广泛使用的实现，它摒弃了循环语句：

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/for_loop/for_enumerate.rs:feature-okay }}
```

### 方法`next()`实例

　　通过下面程序解释使用方法`next()`的技巧：

> 可行结构：可变对象＋可变迭代器(iter_mut)＋方法`next`

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/for_loop/for_next.rs:feature-ok }}
```

> 不可行结构：可变对象＋迭代器(iter_mut)＋方法`next`

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/for_loop/for_next.rs:feature-error_01 }}
```

> 不可行结构：（可变）对象＋可变迭代器(into_iter)＋方法`next`

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/for_loop/for_next.rs:feature-error_02 }}
```

## 参考资料
- [rust-for-loop](http://xion.io/post/code/rust-for-loop.html)
- [understanding-rust-loops](https://blog.codeship.com/understanding-rust-loops/)
- [fighting-the-borrow-checker-in-a-loop](https://users.rust-lang.org/t/fighting-the-borrow-checker-in-a-loop/22975)
- [rust-by-example/flow_control](https://doc.rust-lang.org/rust-by-example/flow_control/for.html)
- [whats-the-difference-between-for-x-in-b-and-for-x-in-b-into-iter](https://users.rust-lang.org/t/whats-the-difference-between-for-x-in-b-and-for-x-in-b-into-iter/14739)
- [how-can-i-do-a-mutable-borrow-in-a-for-loop](https://stackoverflow.com/questions/39622783/how-can-i-do-a-mutable-borrow-in-a-for-loop)
- [pointers-in-rust-a-guide](https://words.steveklabnik.com/pointers-in-rust-a-guide)
- [rust-ref](https://jvns.ca/blog/2017/11/27/rust-ref/)
- [search?q=rust+mutable+borrow+in+loop](https://www.google.com/search?q=rust+mutable+borrow+in+loop)
- [what-is-the-difference-between-iter-and-into-iter](https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter)
- [a-journey-into-iterators](https://hoverbear.org/blog/a-journey-into-iterators/)
- [yarit-yet-another-rust-iterators-tutorial](https://dev.to/dandyvica/yarit-yet-another-rust-iterators-tutorial-46dk)
- [effectively-using-iterators-in-rust](https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html)
- 


```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/for_loop/mut_string.rs:feature-okey }}
```

![image](../../hello-borrowing/images/hello_borrowing-12_vec_for_loop.png)