# 前言

虽然现在有很多AI编写代码的工具，比如我自己用的cursor，claude code这些，我甚至都不需要知道rust的语法基础

都可以完成工作中的任务，但是我的内心告诉我这个是不对的，不应该这样。

我最早在学校掌握的是Java，但是毕业后用了2～3年的python，记得那个时候还有一些工程是py2的版本，

用的最新额也是3.6，3.7，3.8这些，后面工作中慢慢接触到了golang，我迫切的希望能掌握go这门新语言

就像现在我要掌握rust一样，不过那个时候AI还没这么牛逼，还是古法coding的方式，迫使我去学习了go，

慢慢的用的多了，我也就慢慢掌握了go（其实还是go很好上手，规范很清晰的原因）

但是现在AI代码工具太好用了，导致我甚至都不会写rust代码，也能完成工作中的任务，这让我学习rust的进度一直停滞不前

现在我要改变这个事实:D

# 如何掌握rust

## 入门(fundamentals)
我认为入门一个新的语言，需要先了解下面这些基础的信息（大概率每个语言都差不多）

- 会在自己电脑上安装他
- 会写hello world，并且运行他
- 会基础的变量，函数声明和使用
- 会基础的流程控制，for，if这些的使用
- 会定义class（py/java）/struct（go）这种面向对象的结构
- 会安装第三方的包/库(这里只是简单安装和使用)
- 会打包

## 熟练(core)

然后了解Rust和其他语言不一样的地方

- 所有权&借用&生命周期
- 能够使用常见数据结构（Vec、HashMap、String 等）
- 掌握 enum 和 pattern matching（match）的使用
- 熟悉错误处理（Result、Option）并能写出健壮代码
- 理解模块系统（mod / crate）并能组织项目结构
- 能够使用常见第三方库（如 serde、reqwest 等）
- 熟悉 iterator、闭包（closure）等常用抽象

## 精通(advanced)

接下来就是熟悉 Rust 的设计哲学和底层原理

- 熟练使用 trait（泛型、trait bound、trait object）
- 掌握并发编程（线程、async/await、tokio 等）


# 后记

之前都说，掌握一门语言的前提是掌握好另一门语言，我觉得说的很有道理，因为要懂对比，才会学的快，

所以本教程，我会尽量用学过的语言比如go、python等来对比rust，这样既巩固了我的旧语法(哈哈哈)又学习了rust，一石二鸟，美滋滋～

# QuickStart

在本项目的根目录是可以直接运行子目录的代码的，这样方便一边在IDEA上看README和代码，也能直接在根目录跑对应的命令

避免还有cd到对应子目录再运行，下面以01下的第一个hello world作为例子

```
wangyu2@60305278M [16:44:48] [~/github/rust-learning] [main *]
-> % cargo run -p hello_world
   Compiling hello_world v0.1.0 (/Users/wangyu2/github/rust-learning/01_fundamentals/02-HelloWorld/hello_world)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/hello_world`
Hello, world!
```

-p 这里后面接的是子目录下Cargo.toml里面定义的名字，例如下面的hello_world，和文件/文件夹名称无关
```
[package]
name = "hello_world"
version = "0.1.0"
edition = "2024"

[dependencies]
```