/* # use macro to reuse code
学习资料1：https://doc.rust-lang.org/1.7.0/book/macros.html
学习资料2：https://doc.rust-lang.org/book/ch19-06-macros.html

## WhatWhyHow

### 什么是宏
类似Java注解(annotations)，编译器编译时会检查宏去自动生成一些代码

### 为什么需要宏
更高级的复用代码，实现eval?
If you visually identify a pattern of repeated code,
you may find it’s difficult or cumbersome(笨重) to express
that pattern as a generic function, a trait, or anything else within Rust’s semantics

### 宏的drawback(弊端)
降低可读性，类似ruby元编程，过度的元编程导致别人难以理解代码，你自己写元编程很爽代码量很少，别的同事却很难看懂宏，这也是ruby不够工程化的原因。
错误检查不仔细，相比普通代码，宏内部的代码难以Debug，建议调试OK的代码再抽取为宏进行复用

# 入门rust的宏

目标一：复用相似函数的传参语句

## 需要借助的std宏

### stringify!(逆eval)

如果说eval是解析字符串将其转为表达式并执行(求值)，

那么stringify!则是将输入的表达式转为字符串

应用1：编写宏时有时需要获取输入变量的名称
应用2：用于Debug时，打印`变量名=变量值`的格式，例如"{}={},stringify!(var),var"

不过应用2的情景一般用dbg!就可以了

*/

macro_rules! new_order {
    (ask => $e:expr) => {
        println!("new_ask_order: {}", $e)
    };
    (bid => $e:expr) => {
        println!("new_bid_order: {}", $e)
    };
}

macro_rules! get_input_identifier {
    ($i:ident) => {
        println!("{}", $i);
    }
}

fn main() {
    // stringify!宏将Rust的
    assert_eq!(stringify!(1 + 1), "1 + 1");

    new_order!(ask => 1);
    new_order!(bid => 1);
    let is_ask = true;
    get_input_identifier!(is_ask);
}