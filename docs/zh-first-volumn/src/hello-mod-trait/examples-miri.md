# 深度解析动态与静态调度实现

　　在本节里。主要解释静态函数与动态函数的内部结构。

## 学习内容
- 进一步学习静态函数与动态函数
- 通过Miri了解分析Rust语言代码

## 篇目

- [什么是Miri](#什么是Miri)
- [与Miri相关的Cargo工具命令](#与Miri相关的Cargo工具命令)
- [打开Miri代码文件命令](#打开Miri代码文件命令)
- [Miri代码实例说明](#Miri代码实例说明)
- [参考资料](#参考资料)

## 什么是Miri

　　[Miri](https://github.com/rust-lang/miri)网站说明如下：

    An experimental interpreter for Rust's mid-level intermediate representation (MIR).

直接翻译为：Rust语言的中间中级水平表达层（mid-level intermediate representation，MIR）的实验解释器。这里有两个中间是什么意思？“intermediate”是说明Miri是介于Rust语言与汇编语言编译过程的中间位置；“mid-level”可以理解为表达层代码水平。

　　Miri作用是什么？在编译器中引入这一表达层（MIR），消除了Rust语言代码大部分表面的表示层（mid-level），留下了一种更简单的形式，目的是适合于类型检查和翻译成汇编语言（intermediate）。

## 与Miri相关的Cargo工具命令

　　在项目根目录下，执行代码命令，可以得到Miri代码文件。一般情况下，该文件是不会出现在项目目录里的。

　　这个命令是两部分，在`cargo之前部分，是告诉编译器想到得到额外的编译结果。后面部分是想编译什么内容。

　　与Miri编译相关内容是在`release`编译版本下才能得到。

```bash
RUSTFLAGS="--emit mir" cargo build --release --example trait_dispatch_concrete
```

## 打开Miri代码文件命令

　　可以通过资源管理器来寻找如下命令里的目录文件，也可以使用下面命令打开，其中`-t`是告诉命令`open`使用默认编辑器打开该文件。该文件名称非常长，所以命令里使用了星号。

　　从网络上看，还没有能够显示Miri代码的工具。

```bash
open -t ./target/release/examples/trait_dispatch_concrete-*.mir
```

## Miri代码实例说明

　　为了说明问题，下面Miri代码仅仅是其一部分代码，包括静态函数和动态函数，并且还是省略过的。凡是"..."都是两个函数相同的代码。

　　从下面的Miri代码里，可以看到，函数`static_dispatch()`的参数只有一个，它是衔接特质的对象指针，函数`dynamic_dispatch`的参数也只有一个，它是衔接特质的指针。除了这一点区别之外，其余都是一样的。

　　从上面分析可以了解到，衔接特质的对象是确定的，使用它，可以理解为已知类型的特质准备的，而动态的衔接特质是不确定的，可以理解为未知类型的特质准备的。

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

## 参考资料
- [An interpreter for Rust's mid-level intermediate representation](https://github.com/rust-lang/miri)