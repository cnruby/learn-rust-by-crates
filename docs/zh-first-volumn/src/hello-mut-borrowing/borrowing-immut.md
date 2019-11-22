# 应用篋：绑定引用的固定对象借用实例

　　从可变对象（mutable variable）出发，其引用方式，使用两种不同的借用方式：固定引用（immutable reference）和可变引用（mutable reference）。本节将解释把固定和可变引用绑定到固定对象的借用方式。尤其是当固定对象与固定和可变引用交互在一起时，程序代码将会比较更复杂一些。但是通过这一节学习可以加深对借用对象的生命周期概念的理解。

## 学习内容
- 了解和学习把固定和可变引用绑定到固定对象的借用实例

## 篇目
　　
- [绑定固定引用的固定对象](#绑定固定引用的固定对象)
- [绑定可变引用的固定对象](#绑定可变引用的固定对象)
- [绑定不同引用的固定对象](#绑定不同引用的固定对象)
- [题外话](#题外话)
- [参考资料](#参考资料)

## 绑定固定引用的固定对象

> let immut_and_immut_ref = &immut_instance;
>
> let immut_and_immut_ref = &mut_instance;

![image](../../hello-borrowing/images/hello_borrowing-17-mut_ref.png)

　　上面示意图说明了下面程序可变对象`mut_instance`及其固定引用的对象`immut_ref`（下面简称为“固定引用的固定对象”或者”引用固定对象“）的生命周期。左边图一直可以使用到它们的生命周期结束，这是下面程序的情况。图上带箭头的连接线，是程序代码顺序，而图上无箭头的连接线说明了对象的生命周期范围。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_ref.rs:feature-ok }}
```

　　另外一种情况，就是右边示意图，当可变对象`mut_instance`需要拿回所有权之时，也是引用固定对象`immut_ref`生命周期结束之时。要是还想使用已经结束生命周期的对象`immut_ref`，编译器就会告诉我们这是不可以的。在之后的其他实例说明过程中，不再说明左边图的情况，主要说明右边示意图的情况。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_ref.rs:feature-error_02 }}
```

　　要是上面程序，在对象`mut_instance`需要拿回所有权以后，不再使用对象`immut_ref`，程序一切正常。

　　下面程序说明了这种直接转移可变对象`mut_instance`，或者说使用对象`copy_mut_instance`，使得可变对象`mut_instance`失去其生命周期，取而代之。在对象`copy_mut_instance`取而代之以后，这样，就不再可以使用对象`mut_instance`。要是再使用它就会如下下面程序一样，出现编译错误。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_ref.rs:feature-error_01 }}
```

## 绑定可变引用的固定对象

> let immut_and_mut_ref = &mut mut_instance;

![image](../../hello-borrowing/images/hello_borrowing-16-mut_ref.png)

　　上面示意图也是说明下面程序可变对象`mut_instance`及其可变引用的对象`mut_ref`（下面简称为“可变引用的固定对象”或者”引用固定对象“）`mut_ref`的生命周期。

　　对于这种可变对象及其可变引用的固定对象情况，从借用机制上看，与上面可变类型对象的固定引用的固定对象思路是一样的，不同的是可变引用的固定对象可以修改其可变对象的值，并且修改结果直接改变到可变对象的内容。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_ref.rs:feature-cp }}
```

　　下面程序说明了，当对象`mut_instance`拿回所有权以后，不再可以使用引用固定对象`mut_ref`了。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_ref.rs:feature-error_03 }}
```

## 绑定不同引用的固定对象

![image](../../hello-borrowing/images/hello_borrowing-14-mut_refs.png)

　　上面示意图的左边分支说明下面程序对象`mut_instance`、`mut_ref`和`immut_ref`的生命周期。这里看到，一个可变对象涉及到其固定引用与可变引用的固定对象的使用方法。要是查看三个对象的内存地址，它们是完全相同的。

　　下面程序是左边示意图的正确借用方法：

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_refs.rs:feature-ok }}
```

　　下面程序说明了固定对象`immut_ref`的生命周期编译错误问题：

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_refs.rs:feature-error_01 }}
```

　　下面程序说明了固定对象`mut_ref`的生命周期编译错误问题：

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

　　下面示意图说明还有一些其他更多的可能性。

![image](../../hello-borrowing/images/hello_borrowing-15-mut_refs.png)

## 题外话

## 参考资料
- [mutable-borrow-automatically-changes-to-immutable](https://stackoverflow.com/questions/40654940/mutable-borrow-automatically-changes-to-immutable/40655179#40655179)
- [mutable-reference-in-rust](https://medium.com/@vikram.fugro/mutable-reference-in-rust-995320366e22)