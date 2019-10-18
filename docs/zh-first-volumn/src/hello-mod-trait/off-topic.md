# 题外话

## 篇目
- [泛型类型实例](#泛型类型实例)
- [泛型方法实例](#泛型方法实例)
- [泛型实现实例](#泛型实现实例)
- [泛型特质实例](#泛型特质实例)
- [参考资料](#参考资料)

## 泛型类型实例

{{#playpen ../../../../hello-mod-trait/lib-hello/examples/generics_type_hello.rs editable}}

## 泛型方法实例

{{#playpen ../../../../hello-mod-trait/lib-hello/examples/generics_fn_hello.rs editable}}

## 泛型实现实例

{{#playpen ../../../../hello-mod-trait/lib-hello/examples/generics_impl_hello.rs editable}}

## 泛型特质实例

{{#playpen ../../../../hello-mod-trait/lib-hello/examples/generics_trait_hello.rs editable}}

```rust
// Dynamically Sized Types (DSTs)
// https://doc.rust-lang.org/nomicon/exotic-sizes.html
struct MySuperSliceable<T: ?Sized> {
    info: u32,
    data: T
}

fn main() {
    let sized: MySuperSliceable<[u8; 8]> = MySuperSliceable {
        info: 17,
        data: [0; 8],
    };

    let dynamic: &MySuperSliceable<[u8]> = &sized;

    // prints: "17 [0, 0, 0, 0, 0, 0, 0, 0]"
    println!("{} {:?}", dynamic.info, &dynamic.data);
}
```

## 参考资料
- [learning-generics-in-rust](https://tutorialedge.net/rust/learning-generics-in-rust/)
- [book ch10-01-syntax](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- 