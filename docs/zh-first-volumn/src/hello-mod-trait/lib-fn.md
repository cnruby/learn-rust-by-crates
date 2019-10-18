# 共享篋：简单三层结构实现

　　在前面学习概念基础之上，在这一节里，将会实现共享篋的简单三层结构。

## 学习内容
- 理解和掌握知识模块的实现

## 篇目
- [静态函数的知识模块实现](#静态函数的知识模块实现)
- [动态函数的知识模块实现](#动态函数的知识模块实现)
- [应用实例](#应用实例)
- [参考资料](#参考资料)

## 静态函数的知识模块实现

　　下面模块`mod_static_fn`代码，利用衔接特质`TraitCanal`，实现了两个不同功能的静态函数：`get_static_type_ref()`和`print_static_all_daten()`。

　　关于函数`get_static_type_ref()`，输入实例是类型`StructType`或者`TupleType`的指针，输出是类型`tuple`，其元素值是它们属性`data`值。

　　关于函数`print_static_all_daten()`，输入实例是`StructType`或者`TupleType`数组的指针，程序接受输入实例以后，打印输入实例及其属性内容。

　　为了使用函数`print_static_all_daten()`里的打印宏，程序里需要做到两点：

- 使用语句：`use std::fmt::Debug;`;
- 函数参数里增加该特质`Debug`;

但是第一条语句应该不需要，因为类型`StructType`和`TupleType`已经声明过了特质`Debug`。如特质`PartialEq`就没有使用`use`语句，这是因为类型`StructType`和`TupleType`也已经声明过了`PartialEq`。但是有一点是肯定的：先要声明特质，然后还要在静态函数里使用它们，才能真正实现使用这些特质。

{{#playpen ../../../../hello-mod-trait/lib-hello/src/mod_static_fn.rs editable}}

## 动态函数的知识模块实现

　　下面模块`mod_dynamic_fn`代码的函数，与前面的说明完全类似。

{{#playpen ../../../../hello-mod-trait/lib-hello/src/mod_dynamic_fn.rs editable}}

## 应用实例

　　

{{#playpen ../../../../hello-mod-trait/lib-hello/examples/trait_fn_hello.rs editable}}




## 参考资料