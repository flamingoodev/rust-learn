use std::any::type_name;
use std::ops::Mul;
use std::sync::Arc;

#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

/// 带有默认范性类型的结构体
#[derive(Debug)]
pub struct PointDefault<T = u32> {
    pub x: T,
    pub y: T,
}

impl<T> PointDefault<T> {
    pub fn x(&self) -> &T {
        &self.x
    }
}

/// 默认实现
impl PointDefault {
    pub fn y(&self) {
        test_type(self.y);
        println!("{}", &self.y)
    }
}

/// 带有默认范性类型的结构体
/// 默认泛型类型参数，表示如果不显示指定泛型类型，就默认泛型类型为Self。
/// 当使用泛型类型参数时，可以为泛型指定一个默认的具体类型。如果默认类型就足够的话，
/// 这消除了为具体类型实现 trait 的需要。
/// 为泛型类型指定默认类型的语法是在声明泛型类型时使用 <PlaceholderType=ConcreteType>。
pub trait Watch<Inner = String> {
    type Item;
    fn inner(&self) -> Option<Self::Item>;
    fn info(&self) -> Inner;
}

struct A {
    data: i32,
}

impl Watch<i32> for A {
    type Item = i32;
    fn inner(&self) -> Option<Self::Item> {
        Some(self.data)
    }
    fn info(&self) -> i32 {
        println!("A inner is {}", self.data);
        self.data
    }
}

struct B {
    data: String,
}

/// 默认实现
impl Watch for B {
    type Item = String;
    fn inner(&self) -> Option<Self::Item> {
        Some(self.data.clone())
    }
    fn info(&self) -> String {
        println!("B inner is {}", self.data);
        self.data.clone()
    }
}

fn main() {
    let a = A { data: 10 };
    let b = B {
        data: String::from("B"),
    };
    assert_eq!(10, a.info());
    assert_eq!(Some(String::from("B")), b.inner());
}

impl<T> Point<T> {
    pub fn x(&self) -> &T {
        &self.x
    }
}

/// 为结构体实现了方法
impl Point<usize> {
    pub fn usize_x(&self) -> &usize {
        &self.x
    }
    pub fn show(&self) {
        println!("{:?}", self)
    }
}

/// 定义了函数
/// Mul有理数限定
fn square<T: Mul + Mul<Output = T>>(x: T, y: T) -> T {
    x * y
}

fn square1<T: Mul<T, Output = T>>(x: T, y: T) -> T {
    x * y
}

fn test_type<T>(_: T) {
    println!("{:?}", { type_name::<T>() });
}

#[test]
fn test_square() {
    let s = square(10, 20);
    let s1 = square1(10, 20);
    let f = square1(10.2, 20.44);
    let f1 = square1(10.2, 20.44);
    assert_eq!(s, s1);
    assert_eq!(f, f1);
    // turbofish operation
    let r = square::<u32>(37, 41);
    let r1 = square::<f32>(37.7, 41.2);
    println!("{}", r);
    println!("{}", r1);
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.usize_x());
    p.show();
    Point::show(&p);
    println!("p.x = {}", p.x());
    let p1 = PointDefault::<u32> { x: 5, y: 10 };
    let p2 = PointDefault { x: 5, y: 10 };
    test_type(p1.x);
    test_type(p2.x());
    p2.y()
}
