use std::ops::Mul;

#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
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
}
