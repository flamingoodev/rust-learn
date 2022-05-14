use std::str::FromStr;

use strum::{Display, EnumCount, EnumDiscriminants, EnumString};

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

#[derive(Debug, Eq, PartialEq, EnumString, Display, EnumCount, EnumDiscriminants)]
enum Color {
    /// Random Docs
    #[strum(to_string = "RedRed")]
    Red,
    #[strum(serialize = "b", to_string = "blue")]
    Blue {
        hue: usize,
    },
    #[strum(serialize = "y", serialize = "yellow")]
    Yellow,
    White,
    #[strum(disabled)]
    Green(String),
}

#[test]
fn t_strum() {
    let red = format!("{:?}", Color::Red);
    println!("{}", red); // Red
    println!("{:?}", Color::Red); // Red
    let color: String = Color::Red.to_string();
    println!("{}", color); // RedRed

    let new_color = "y";
    let s = Color::try_from(new_color).unwrap();
    println!("{:?}", s);

    let new_color1 = "White";
    let s1 = Color::try_from(new_color1).unwrap();
    println!("{:?}", s1);

    let new_color2 = "White";
    let s2 = Color::from_str(new_color2).unwrap();
    println!("{:?}", s2);
}

pub fn t_enum() {
    let row = vec![
        SpreadsheetCell::Int(10),
        SpreadsheetCell::Float(10.10),
        SpreadsheetCell::Text(String::from("blue")),
    ];
    for cell in row {
        match cell {
            SpreadsheetCell::Int(i) => println!("Int value is {}", i),
            SpreadsheetCell::Float(f) => println!("Float value is {}", f),
            SpreadsheetCell::Text(t) => println!("Text value is {}", t),
        }
    }
}
