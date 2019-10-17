# 深度解析动态与静态调度实现

## 说明是Miri

An experimental interpreter for Rust's mid-level intermediate representation (MIR).

## 

```bash
RUSTFLAGS="--emit mir" cargo build --release --example trait_dispatch_concrete
```

```
fn  static_dispatch(_1: &TraitObject) -> () {
    ...
    let mut _3: &TraitObject;            // in scope 0 at lib-hello/examples/trait_dispatch_concrete.rs:23:5: 23:11

    bb0: {
        ...
        _2 = const <TraitObject as Trait>::fn(move _3) -> bb1; // bb0[3]: scope 0 at lib-hello/examples/trait_dispatch_concrete.rs:23:5: 23:18 //...

    }

    bb1: {
        ...
    }
}
```

```bash
open  -t ./target/release/examples/trait_dispatch_concrete-*.mir
```

```
fn  dynamic_dispatch(_1: &dyn Trait) -> () {
    ...
    let mut _3: &dyn Trait;              // in scope 0 at lib-hello/examples/trait_dispatch_concrete.rs:27:5: 27:12

    bb0: {
        ...
        _2 = const <dyn Trait as Trait>::fn(move _3) -> bb1; // bb0[3]: scope 0 at lib-hello/examples/trait_dispatch_concrete.rs:27:5: 27:19 //...
    }

    bb1: {
        ...
    }
}
```



- [An interpreter for Rust's mid-level intermediate representation](https://github.com/rust-lang/miri)