# 第三章 软件篋mod_trait_exerci

## 学习内容
- 学习软件篋文件、模块与程序结构
- 了解和学习动态调度关键词`dyn`
- 学习和理解动态与静态调度（Static vs Dynamic Dispatch）
- 衔接类型关键词`trait`作用
- 

rust没有继承，继承不流行了。通过trait缺省函数和覆盖来重用代码，泛型来实现多态。

rust也通过trait object, 特性对象来实现多态。

特性对象，本质上就是不限类型的，动态的特性绑定，run time匹配具体类型

　　
You can only make object-safe traits into trait objects.
您只能将对象安全的特征变成特征对象。

Traits that do not resolve to concrete type of implementation. 
不能解决具体实现类型的特征。

In practice two rules govern if a trait is object-safe.
实际上，有两个规则控制特征是否是对象安全的。

The return type isn’t Self.
There are no generic type parameters.
返回类型不是Self。
没有通用类型参数。


Any trait satisfying these two rules can be used as trait objects.
任何东西都可以用作特质对象。

Example of trait that is object-safe can be used as trait object:
可以将对象安全的特征示例用作特征对象：

trait Draw {
    fn draw(&self);
}

Example of trait that cannot be used as trait object:
不能用作特征对象的特征示例：

trait Draw {
    fn draw(&self) -> Self;
}

## 参考资料
- [rust_testing_mocking/slides/2113/testing_in_rust_by_donald_whyte.pdf](https://archive.fosdem.org/2018/schedule/event/rust_testing_mocking/attachments/slides/2113/export/events/attachments/rust_testing_mocking/slides/2113/testing_in_rust_by_donald_whyte.pdf)
- [std keyword dyn](https://doc.rust-lang.org/std/keyword.dyn.html)
- [dyn-trait-for-trait-objects](https://doc.rust-lang.org/edition-guide/rust-2018/trait-system/dyn-trait-for-trait-objects.html)
- [rust-traits-and-trait-objects](https://joshleeb.com/posts/rust-traits-and-trait-objects/)
- [a_quick_look_at_trait_objects_in_rust](https://tratt.net/laurie/blog/entries/a_quick_look_at_trait_objects_in_rust.html)
- [Code](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=5d573e667aac08f960557b79d6385f6e)
- [what-makes-something-a-trait-object](https://stackoverflow.com/questions/27567849/what-makes-something-a-trait-object)
- [Dynamic_dispatch](https://en.wikipedia.org/wiki/Dynamic_dispatch)
- [reference identifiers](https://doc.rust-lang.org/reference/identifiers.html)
- [rust-traits-and-trait-objects](https://joshleeb.com/posts/rust-traits-and-trait-objects/)
- [traits-on-generics](https://blog.theenginerd.com/blog/2015/06/27/traits-on-generics/)
- [learning-generics-in-rust](https://tutorialedge.net/rust/learning-generics-in-rust/)
- [generic-returns-in-rust](https://blog.jcoglan.com/2019/04/22/generic-returns-in-rust/)
- [writing-a-hashmap-to-struct-procedural-macro-in-rust](https://cprimozic.net/blog/writing-a-hashmap-to-struct-procedural-macro-in-rust/)
- [impls_and_traits](https://learning-rust.github.io/docs/b5.impls_and_traits.html)