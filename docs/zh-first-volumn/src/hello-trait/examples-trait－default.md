# 题外话：标准库默认衔接特质Default


## 学习内容
- 理解标准库默认衔接特质Default

## 篇目

1. [默认衔接特质`Default`功能](#默认衔接特质Default功能)
1. [使用默认衔接特质`Default`](#使用默认衔接特质Default)
1. [实现自己默认衔接特质`Default`](#实现自己默认衔接特质Default)
1. [参考资料](#参考资料)


## 默认衔接特质`Default`功能

　　标准库默认衔接特质`Default`也是一个有用的可衍生特质，为类型提供默认值的特质，也就是为类型提供默认的实例化手法。

## 使用默认衔接特质`Default`

　　在使用默认衔接特质`Default`的函数default()定义变量时，必须说明变量类型。

{{#playpen ../../../../hello-trait/lib-hello/examples/trait_default.rs editable}}

## 实现自己默认衔接特质`Default`

```rust
#[derive(Debug)]
pub struct Person {
    name: String,
    age: u32,
}

impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("Leo"),
            age: 24,
        }
    }
}

fn main() {
    let instance: Person = Default::default();
    println!("{:?}", instance);
    let mut instance = Person { age: 23, ..Default::default() };
    instance.age = 24;
    println!("{:?}", instance);
}
```

## 参考资料
- [std::default::Default](https://doc.rust-lang.org/std/default/trait.Default.html)
