# 应用篋：借用次数实例

　　在Rust语言里，从固定和可变绑定对象引用形式上是相同的，但是它们可以引用的次数是不同的。可变绑定对象引用只能是一次，而固定对象引用可以是多次。

## 学习内容
- 了解和学习Rust语言借用次数实例

## 篇目

- [可变关键词`mut`绑定对象的借用方法](#可变关键词mut绑定对象的借用方法)
- [固定绑定对象的借用次数](#固定绑定对象的借用次数)
- [关键词`mut`绑定对象的借用次数](#关键词mut绑定对象的借用次数)
- [可变绑定对象与固定绑定对象联系](#可变绑定对象与固定绑定对象联系)　　
- [题外话](#题外话)
- [参考资料](#参考资料)

## 可变关键词`mut`绑定对象的借用方法

　　可变关键词`mut`用来绑定可变对象。下面程序代码的第一行就是实例。在`let`语句的基础上，增加可变关键词`mut`就可以实现绑定可变对象。所谓可变对象，就是其值在其范围内**肯定**会被修改的。修改可变对象值的实例是第二段代码的第一行。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_base/mut_ref.rs:mut_ref }}
```

　　上面程序第三段代码的第一行代码是，绑定可变引用对象的方法实例。在紧接着引用符`＆`之后，增加可变关键词`mut`就可实现绑定可变引用对象。修改可变引用对象值的实例是第三段代码的第二行。

　　需要注意的是，修改可变引用对象（如这里：`ref_mut_instance`）值就是修改其可变对象（如这里：`mut_instance`）值。

　　在第四段代码里，还可以使用可变对象`mut_instance`，这说明了上面可变引用对象是可行的。下面是该程序的输出结果：

```bash
1. mut_instance = 1
2. mut_instance = 33
3. mut_instance = 42
4. mut_instance = 100
```

## 固定绑定对象的借用次数

　　下面程序将要说明，对于固定和可变绑定对象而已，使用固定绑定引用对象的次数是不受限制的。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_base/mut_count.rs:feature-ok }}
```

## 关键词`mut`绑定对象的借用次数

　　对于可变绑定对象而已，当其引用对象也是可变的时，在其有效范围内，这种可变引用对象的绑定同时只能存在一个。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_base/mut_count.rs:feature-err_01 }}
```

　　下面程序里使用了两次可变引用对象的绑定，这样就会导致编译器提示错误信息：同时不能多次借用对象`instance`作为可变引用变量。

```bash
error[E0499]: cannot borrow `instance` as mutable more than once at a time
  --> bin-hello/examples/mut_base/mut_count.rs:38:26
   |
37 |     let first_mut_ref = &mut instance; // mutable reference
   |                         ------------- first mutable borrow occurs here
38 |     let second_mut_ref = &mut instance; // mutable reference
   |                          ^^^^^^^^^^^^^ second mutable borrow occurs here
39 |     println!("{:?} {:?}", first_mut_ref, second_mut_ref);
   |                           ------------- first borrow later used here
```

## 可变绑定对象与固定绑定对象联系

　　对于可变绑定对象，要是先绑定了其固定引用对象，然后再绑定其可变引用对象，注意此时，只是前面固定引用对象不能再借用了，而后面可变引用对象的借用还是有效的。这是下面的实例程序。要是把该程序最后一行代码注释掉，程序还是运行正常的。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_base/mut_count.rs:feature-err_02 }}
```

　　同样道理，对于可变绑定对象，要是先绑定了其可变引用对象，然后再绑定其固定引用对象，则前面可变引用对象就不能在借用了，而后面固定引用对象的借用还是有效的。这是下面的实例程序。要是把该程序最后一行代码注释掉，程序还是运行正常的。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_base/mut_count.rs:feature-err_03 }}
```

## 题外话

## 参考资料
- 