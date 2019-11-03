[![Crates.io](https://img.shields.io/crates/v/borrowing_exerci?label=borrowing_exerci)](https://crates.io/crates/borrowing_exerci)
[![The Crate borrowing_exerci Code](https://img.shields.io/badge/hello--borrowing-code-yellowgreen)](https://github.com/cnruby/learn-rust-by-crates/tree/master/hello-borrowing)

> $$\text{只有心灵的淡定宁静，继而产生的身心愉悦，才是幸福的真正源泉}$$

# 软件篋borrowing_exerci

## re-sizable type / dynamical type / resizable containers / a heap allocated value of type:
- their size is not known at compile time, but they can grow or shrink at any time
- capacity / length
- Vectors are re-sizable arrays, A contiguous growable array type
- slices
- String
- (vec, list, map, set, hashmap, string, etc... )

## fixed-size type / static type / a regular fixed size type / a stack allocated value of type: 
- the array is a fixed-size list of elements, A fixed-size array
- length


When comparing pointers they are compared by their address, rather than by what they point to. When comparing pointers to dynamically sized types they also have their addition data compared.



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