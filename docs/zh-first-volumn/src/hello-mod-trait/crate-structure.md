# 文件与模块

　　在这一节里，学习Cargo项目文件、目录与模块相互关系。Rust语言表达模块的三种方式。

## 学习内容
- 了解和学习本软件篋模块文件结构
- 理解和掌握Cargo项目目录与文件关系
- 理解和掌握Cargo项目模块与文件关系
- 理解和掌握Cargo项目文件与文件关系

## 篇目
1. [文件本身表达模块方式](#文件本身表达模块方式)
1. [文件名称表达模块方式](#文件名称表达模块方式)
1. [目录名称表达模块方式](#目录名称表达模块方式)
1. [参考资料](#参考资料)

![image](../../images/hello_mod_trait_00_structure.png)

## 文件本身表达模块方式

　　Ⓓ 使用模块关键词`mod`和代码块`{}`的表达式，在Rust语言程序文件里，可以定义任何一个或者多个不同名称的模块。

　　在文件lib.rs里，创建了称之为`mod_trait`模块，该模块实现代码也在该文件里。

## 文件名称表达模块方式

　　Ⓓ 使用模块关键词`mod`语句，在Rust语言程序文件里，可以定义任何一个或者多个不同的模块。

　　使用关键词`mod`语句，实现模块代码存在两种形式：以文件名称作为模块名称方式和以目录名称为模块名称方式。这里先解释前面一种情况，如程序文件`mod_generics.rs`。

　　实现以文件名称作为模块名称具体方法是，在文件`lib.rs`里，使用语句`pub mod mod_generics;`，且在与文件`lib.rs`相同的目录下，创建名称为`mod_generics.rs`模块程序文件，为了其自身模块，该文件不需要使用模块关键词了。

## 目录名称表达模块方式

　　程序文件`mod.rs`是第三种表达模块方式，即以目录名称为模块名称方式。

　　在程序文件`lib.rs`里，使用语句`pub mod mod_bare;`，说明了该模块是外部文件实现模块代码，但是从中微分确认其实现方式。我们看到在与文件`lib.rs`相同目录下存在目录`mod_bare`，说明了该实现在该目录下的文件，默认情况下就是程序文件`mod.rs`，所有模块目录的入口文件默认情况下都是该文件名称。

## 参考资料
- [how-to-use-one-module-from-another-module-in-a-rust-cargo-project](https://stackoverflow.com/questions/48071513/how-to-use-one-module-from-another-module-in-a-rust-cargo-project)
- [modules-in-rust-programming-language](https://dev.to/saiumesh/modules-in-rust-programming-language-495m)
- [rust-gentle-intro/modules](https://stevedonovan.github.io/rust-gentle-intro/4-modules.html)
- [ch07-01-mod-and-the-filesystem](https://doc.rust-lang.org/1.29.2/book/2018-edition/ch07-01-mod-and-the-filesystem.html)
- [how-to-use-one-module-from-another-module-in-a-rust-cargo-project](https://stackoverflow.com/questions/48071513/how-to-use-one-module-from-another-module-in-a-rust-cargo-project)