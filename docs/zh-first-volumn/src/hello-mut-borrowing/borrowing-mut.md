# 应用篋：可变大小类型借用实例

　　本节将解释可变大小类型的不同借用方式。尤其是当可变类型对象与固定和可变引用对象交互在一起时，程序代码将会比较复杂。但是通过这一节学习可以加深对借用对象的生命周期概念的理解。

## 学习内容
- 了解和学习Rust语言可变大小类型的借用实例

## 篇目
　　
- [可变类型对象的固定引用对象](#可变类型对象的固定引用对象)
- [可变类型对象的可变引用对象](#可变类型对象的可变引用对象)
- [可变类型对象的不同引用对象](#可变类型对象的不同引用对象)
- [题外话](#题外话)
- [参考资料](#参考资料)

## 可变类型对象的固定引用对象

![image](../../hello-borrowing/images/hello_borrowing-16-mut_ref.png)

　　上面示意图说明了下面程序对象`mut_instance`和`immut_ref`的生命周期。左边图一直可以使用到它们的生命周期结束，这是下面程序的情况。图上带箭头的连接线，是程序代码顺序，而图上无箭头的连接线说明了对象的生命周期范围。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_ref.rs:feature-ok }}
```

　　另外一种情况，就是右边示意图，当对象`mut_instance`需要拿回所有权之时，也是对象`immut_ref`生命周期结束之时。要是还想使用已经结束生命周期的对象`immut_ref`，编译器就会告诉我们这是不可以的。在之后的其他实例说明过程中，不再说明左边图的情况，主要说明右边示意图的情况。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_ref.rs:feature-error_02 }}
```

　　要是上面程序，在对象`mut_instance`需要拿回所有权以后，不再使用对象`immut_ref`，程序一切正常。

　　下面程序说明了这种直接转移可变对象`mut_instance`，或者说使用对象`copy_mut_instance`，使得可变对象`mut_instance`失去其生命周期，取而代之。在对象`copy_mut_instance`取而代之以后，这样，就不再可以使用对象`mut_instance`。要是再使用它就会如下下面程序一样，出现编译错误。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_ref.rs:feature-error_01 }}
```

　　对于可变类型对象的固定引用对象，上面程序仅仅是一种可能性。下面示意图说明还有一些其他更多的可能性。

![image](../../hello-borrowing/images/hello_borrowing-15-mut_refs.png)

## 可变类型对象的可变引用对象

![image](../../hello-borrowing/images/hello_borrowing-16-mut_ref.png)

　　上面示意图也是说明下面程序对象`mut_instance`和`mut_ref`的生命周期。

　　对于这种可变类型对象的可变引用对象情况，从借用机制上看，与上面可变类型对象的固定引用对象思路是一样的，不同的是可变引用对象可以修改值，并且修改结果直接改变其可变对象的内容。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_ref.rs:feature-cp }}
```

　　下面程序说明了，当对象`mut_instance`拿回所有权以后，不再可以使用可变引用对象`mut_ref`了。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_ref.rs:feature-error_03 }}
```

## 可变类型对象的不同引用对象

![image](../../hello-borrowing/images/hello_borrowing-14-mut_refs.png)

　　上面示意图的左边分支说明下面程序对象`mut_instance`、`mut_ref`和`immut_ref`的生命周期。这里看到，一个可变对象涉及到其可变引用对象和其固定引用对象的使用方法。要是查看三个对象的内存地址，它们是完全相同的。

　　下面程序是左边示意图的正确借用方法：

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_refs.rs:feature-ok }}
```

　　下面程序说明了对象`immut_ref`的生命周期编译错误问题：

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_refs.rs:feature-error_01 }}
```

　　下面程序说明了对象`mut_ref`的生命周期编译错误问题：

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_refs.rs:feature-error_02 }}
```

　　上面示意图的右边分支是另外一种可能性。上面左边和右边示意图的分支仅仅是下面代码不同。左边分支示意图的对象`immut_ref`的值取自于引用对象`mut_ref`，而右边分支示意图的对象`immut_ref`的值直接取自于`instance`的引用：

```rust
    // left side image
    //let immut_ref :&String = mut_ref;
    // right side image
    let immut_ref :&String = &instance;
```

　　对于这种情况，下面程序给出了对象`mut_ref`的生命周期编译错误问题：

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_refs.rs:feature-error_03 }}
```

## 题外话

## 参考资料
- [mutable-borrow-automatically-changes-to-immutable](https://stackoverflow.com/questions/40654940/mutable-borrow-automatically-changes-to-immutable/40655179#40655179)
