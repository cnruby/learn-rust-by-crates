# 应用篋：变量借用方法


## 复制（Copy）和克隆（Clone）有什么区别？

### 表达方式不同
Copies happen implicitly, for example as part of an assignment y = x. The behavior of Copy is not overloadable; it is always a simple bit-wise copy.

Rust is explicit first!!!
Cloning is an explicit action, x.clone(). The implementation of Clone can provide any type-specific behavior necessary to duplicate values safely. For example, the implementation of Clone for String needs to copy the pointed-to string buffer in the heap. A simple bitwise copy of String values would merely copy the pointer, leading to a double free down the line. For this reason, String is Clone but not Copy.

Clone is a supertrait of Copy, so everything which is Copy must also implement Clone. If a type is Copy then its Clone implementation only needs to return *self (see the example above).

By the way, every Copy type is also required to be Clone. However, they are not required to do the same thing! For your own types, .clone() can be an arbitrary method of your choice, whereas implicit copying will always trigger a memcpy, not the clone(&self) implementation.

复制：
y = x

克隆
y = x.clone()


Arrays with Copy elements are intrinsically Copy themselves, so the Clone implementation can trivially dereference and return self by-value.

具有Copy元素的数组本质上就是复制自身，因此Clone实现可以琐碎地取消引用并返回自身按值。

### 内部实现不同
Clone is designed for arbitrary duplications: a Clone implementation for a type T can do arbitrarily complicated operations required to create a new T. It is a normal trait (other than being in the prelude), and so requires being used like a normal trait, with method calls, etc.

The Copy trait represents values that can be safely duplicated via memcpy: things like reassignments and passing an argument by-value to a function are always memcpys, and so for Copy types, the compiler understands that it doesn't need to consider those a move.

### 类型区别不同
Copy is meant to be implemented for "cheap" types, such as u8 in the example. If you write a quite heavyweight type, for which you think a move is more efficient than a copy, make it not impl Copy. Note that in the u8 case, you cannot possibly be more efficient with a move, since under the hood it would probably at least entail a pointer copy -- which is already as expensive as a u8 copy, so why bother.

## 问题：When should my type be Copy?

Always. If it warns add Copy unless you have a really good reason not to do that.
总是。 如果警告显示，请添加“复制”，除非您有充分的理由不这样做。

According to the docs, Copy should be implemented for all types that is possible. 

Even if your huge struct is Copy there is nothing that prevents you from passing it by reference. Copy just makes copying more convenient. Besides, if you do something like this:

```rust
struct Huge {}
let a = Huge::new(); // and Huge implements Copy
let b = a;
// continue to work with 'b' here, never touch 'a' again.
```

then LLVM will happily optimize the copy away. Personally I would recommend deriving Copy for all types that can, with the exception of types that control access to some kind of resource (but in that case perhaps you don't want them to implement Clone either).

Right, but what if I accidentally continued to use a after is was copied? I would prefer to have the compiler tell me that I (possibly) unnecessarily copied a large struct and that if I really intended on doing that, I should do it explicitly with .clone().

## 为什么存在克隆？
That’s reserved for Clone. There are two reasons:

a Copy copy can be caused by something innocuous like a = b or a function call; it should not be allowed to execute arbitrary code

a move (which is possible for all types) is (under the hood) always the same as a copy, just that the compiler doesn’t let you access the source anymore

Copy requires that the value can be copied using a simple memcpy of the bytes on the stack. But copying a String not only needs to copy the value on the stack (which is just capacity, length and a pointer to the contents) but also to create a new allocation that duplicates the contents.

This is also why Rc, while cheap, cannot be Copy: it needs to increment the reference counter.

o ensure your remark:
- String is copyable, use .clone()
- String is not implicitly copyable, because that would cause non-obvious memory allocations to occur

## 两种不同特质实现
This explicitness shows in the two traits you have here:

Clone is about indicating how to create a new instance, and must be called explicitly. Most types (but not all) can be copied using it.

Copy is a specific compiler trait which indicates that the developer wishes to activate implicit copying for the type; it is only available if a shallow copy is equivalent to a deep copy, which ensures that no memory allocation will occur as part of those implicit copies

## 代码说明

```rust
#[derive(Debug, Clone, Copy)]
struct Point { x: i32, y: i32 }

fn main() {
    let a = Point{x:3,y:4};
    let b = a; // line#1:  call copy method
    
    let c = b.clone(); // is some difference here with last statement line#1 ?

   println!("origin={:?} copied={:?} cloned={:?}",a,b,c);
}
```

The short explanation is “Clone is a way of copying a type that can run arbitrary code. Copy is a way of copying a type that just takes a memcpy.” Since Clone is more general than Copy, you can automatically make anything Copy Clone as well.

In your case, they’re the same. you’re just using a different method.


### 问题
Can I understand as Clone is a deep-copy, and Copy is shadow-copy? 

Clone opens the possibility that the type might do either a deep or shallow copy: "arbitrarily complicated". 


## 参考资料
- [std/marker/trait.Copy](https://doc.rust-lang.org/std/marker/trait.Copy.html)
- [whats-the-difference-between-copy-and-clone](https://doc.rust-lang.org/std/marker/trait.Copy.html#whats-the-difference-between-copy-and-clone)
- [what-is-the-difference-between-copy-and-clone](https://stackoverflow.com/questions/31012923/what-is-the-difference-between-copy-and-clone)
- [whats-the-difference-between-trait-copy-and-clone](https://users.rust-lang.org/t/whats-the-difference-between-trait-copy-and-clone/2609)
- [when_should_my_type_be_copy](https://www.reddit.com/r/rust/comments/2xxjda/when_should_my_type_be_copy/)
- [cant-derive-copy-because-of-string](https://users.rust-lang.org/t/cant-derive-copy-because-of-string/18665/11)
- [how-do-i-implement-copy-and-clone-for-a-type-that-contains-a-string](https://stackoverflow.com/questions/38215753/how-do-i-implement-copy-and-clone-for-a-type-that-contains-a-string)
- [how-to-define-a-copyable-struct-containing-a-string](https://stackoverflow.com/questions/38304666/how-to-define-a-copyable-struct-containing-a-string)

 