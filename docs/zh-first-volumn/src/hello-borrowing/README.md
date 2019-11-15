[![Crates.io](https://img.shields.io/crates/v/borrowing_exerci?label=borrowing_exerci)](https://crates.io/crates/borrowing_exerci)
[![The Crate borrowing_exerci Code](https://img.shields.io/badge/hello--borrowing-code-yellowgreen)](https://github.com/cnruby/learn-rust-by-crates/tree/master/hello-borrowing)

> $$\text{只有心灵的淡定宁静，继而产生的身心愉悦，才是幸福的真正源泉}$$

# 软件篋borrowing_exerci（一）

## 软件篋borrowing_exerci

　　Rust语言的借用机制是其最主要的特点。通过实现借用方法，使得代码更加安全和可靠。

　　Rust语言存在两大数据类型：固定大小类型和可变大小类型。本章重点说明固定大小类型的借用机制。

## 固定大小类型

　　固定大小类型也称之为静态类型、常规固定大小类型或者栈分配值的类型。在编译时知道这种类型的大小，在编译以后，它们大小不可增长或缩小。比如，固定数组是元素的固定大小的列表。

## 可变大小类型

　　可变大小类型也称之为动态类型、可变大小的容器或者堆分配值的类型。在编译时不知道其类型的大小，但是它们可以随时增长或缩小其容量大小。比如，向量是可调整大小的数组，一种连续的可增长数组类型。

## 参考资料
- [split-a-module-across-several-files](https://stackoverflow.com/questions/22596920/split-a-module-across-several-files)
- [ch07-05-separating-modules-into-different-files](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html)
- [splitting-up-modules-in-rust](https://chronicbuildfailure.co/splitting-up-modules-in-rust-5ad7713201d5)
- [references-and-borrowing](https://doc.rust-lang.org/1.8.0/book/references-and-borrowing.html)
- [rust-by-example/scope/borrow](https://doc.rust-lang.org/stable/rust-by-example/scope/borrow.html)
- [rust-borrowing-and-ownership](http://squidarth.com/rc/rust/2018/05/31/rust-borrowing-and-ownership.html)
- [understanding-rust-ownership-borrowing-lifetimes](https://medium.com/@bugaevc/understanding-rust-ownership-borrowing-lifetimes-ff9ee9f79a9c)
- [you-can-t-turn-off-the-borrow-checker-in-rust](https://words.steveklabnik.com/you-can-t-turn-off-the-borrow-checker-in-rust)
- [cant-derive-copy-because-of-string](https://users.rust-lang.org/t/cant-derive-copy-because-of-string/18665/6)
- [whats-the-difference-between-trait-copy-and-clone](https://users.rust-lang.org/t/whats-the-difference-between-trait-copy-and-clone/2609)
- [what-is-the-difference-between-copy-and-clone](https://stackoverflow.com/questions/31012923/what-is-the-difference-between-copy-and-clone)
- [rust-move-copy](https://www.codevamping.com/2018/12/rust-move-copy/)
- [rust-crash-course-02-basics-of-ownership](https://www.snoyman.com/blog/2018/10/rust-crash-course-02-basics-of-ownership)
- [move-clone-copy](https://jeenalee.com/2016/08/29/move-clone-copy.html)
- [the-basics-of-rust-structs](https://facility9.com/2016/04/the-basics-of-rust-structs/)
- [a-single-command-to-compile-and-run-rust-programs](http://blog.joncairns.com/2015/10/a-single-command-to-compile-and-run-rust-programs/)
- [how-to-execute-rust-code-directly-on-unix-systems-using-the-shebang](https://stackoverflow.com/questions/41322300/how-to-execute-rust-code-directly-on-unix-systems-using-the-shebang)
- 