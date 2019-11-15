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



```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_ref.rs:feature-ok }}
```


```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_ref.rs:feature-error_01 }}
```

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_ref.rs:feature-error_02 }}
```

## 可变类型对象的可变引用对象

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_ref.rs:feature-cp }}
```

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_ref.rs:feature-error_03 }}
```

## 可变类型对象的不同引用对象

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_refs.rs:feature-ok }}
```


err_04:上面对象ref_instance生命周期
```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_refs.rs:feature-error_01 }}
```

err_05:上面对象ref_mut_instance生命周期
```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_refs.rs:feature-error_02 }}
```

err_06：比较string_refs.rs:feature-ok
    //let ref_instance :&String = ref_mut_instance;
    let ref_instance :&String = &instance;
```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/mut_var_sized/string_refs.rs:feature-error_03 }}
```


## 题外话

## 参考资料
- [mutable-borrow-automatically-changes-to-immutable](https://stackoverflow.com/questions/40654940/mutable-borrow-automatically-changes-to-immutable/40655179#40655179)
