/// ###类型转换
// Rust 使用 trait 解决类型之间的转换问题。最一般的转换会用到 From 和 into 两个 trait。
// 不过，即便常见的情况也可能会用到特别的 trait，尤其是 从 String 转换到别的类型，以及把别的类型转换到 String 时。

// From 和 Into
// From 和 Into 两个 trait 是内部相关联的，实际上这是它们实现的一部分。如果我们能够从类型 B 得到类型 A，那么很容易相信我们也能把类型 B 转换为类型 A。
//
// From
// From trait 允许一种类型定义 “怎么根据另一种类型生成自己”，因此它提供了一种类型转换的简单机制。在标准库中有无数 From 的实现，规定原生类型及其他常见类型的转换功能。
//
// 比如，可以很容易地把 str 转换成 String：
#[allow(unused)]
fn from() {
    let my_str = "hello";
    let my_string = String::from(my_str);
}

// 也可以为我们自己的类型定义转换机制：

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn from_trait() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
}

#[test]
fn t_from_trait() {
    from_trait();
}

/// Into
// Into trait 就是把 From trait 倒过来而已。也就是说，如果你为你的类型实现了 From，那么同时你也就免费获得了 Into。
//
// 使用 Into trait 通常要求指明要转换到的类型，因为编译器大多数时候不能推断它。
// 不过考虑到我们免费获得了 Into，这点代价不值一提。
#[derive(Debug)]
struct Number1 {
    value: i32,
}

impl From<i32> for Number1 {
    fn from(item: i32) -> Self {
        Number1 { value: item }
    }
}

fn into() {
    let int = 5;
    // 试试删除类型说明
    let num: Number1 = int.into();
    println!("My number is {:?}", num);
}

#[test]
fn t_into() {
    into();
}

/// 2.TryFrom and TryInto
// 类似于 From 和 Into，TryFrom 和 TryInto 是 类型转换的通用 trait。
// 不同于 From/Into 的是，TryFrom 和 TryInto trait 用于易出错的转换，也正因如此，其返回值是 Result 型。
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn try_from() {
    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

#[test]
fn t_try_from() {
    try_from();
}

/// 3. ToString 和 FromStr
// ToString
// 要把任何类型转换成 String，只需要实现那个类型的 ToString trait。
// 然而不要直接这么做，您应该实现fmt::Display trait，它会自动提供 ToString，
// 并且还可以用来打印类型，就像 print! 一节中讨论的那样。
use std::string::ToString;

struct Circle {
    radius: i32,
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}

fn to_string() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}

#[test]
fn t_to_string() {
    to_string();
}

/// 解析字符串
// 我们经常需要把字符串转成数字。完成这项工作的标准手段是用 parse 函数。
// 我们得提供要转换到的类型，这可以通过不使用类型推断，或者用 “涡轮鱼” 语法（turbo fish，<>）实现。
//
// 只要对目标类型实现了 FromStr trait，就可以用 parse 把字符串转换成目标类型。
// 标准库中已经给无数种类型实现了 FromStr。如果要转换到用户定义类型，只要手动实现 FromStr 就行。

fn from_str() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println! {"Sum: {:?}", sum};
}

#[test]
fn t_from_str() {
    from_str();
}
