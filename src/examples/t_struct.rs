use std::f64::consts::PI;

struct Sharpe<T> {
    sharpe: T,
}

struct Rectangle {
    a: f64,
    b: f64,
}

struct Circle {
    r: f64,
}

#[allow(dead_code)]
struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

pub trait Area {
    fn area(&self) -> f64;
}

impl Area for Sharpe<Rectangle> {
    fn area(&self) -> f64 {
        self.sharpe.a * self.sharpe.b
    }
}

impl Area for Sharpe<Circle> {
    fn area(&self) -> f64 {
        PI * self.sharpe.r * self.sharpe.r
    }
}

#[test]
fn t_sharpe_test() {
    let rec = Rectangle { a: 10f64, b: 20f64 };
    let sharpe = Sharpe { sharpe: rec };
    let area = sharpe.area();
    println!("Rectangle area is {}", area);
    //
    let cir = Circle { r: 10f64 };
    let sharpe1 = Sharpe { sharpe: cir };
    let area1 = sharpe1.area();
    println!("Circle area is {}", area1);
}
