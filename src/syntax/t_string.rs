use std::ops::Add;

pub fn t_string() {
    // string slice
    let s = "Hello,World!";
    println!("{}", s);
    // hide old var s
    // string
    let s = "Hello,Rust!".to_string();
    println!("{}", s);
    let mut s1 = String::new();
    s1 += "hello";
    s1 += &*String::from(",rust");
    println!("{}", s1);
    let mut s2 = String::from("hello");
    println!("{}", s2);
    // push a char
    s2.push('R');
    println!("{}", s2);
    // push a string
    s2.push_str("ust");
    // use format!,format! will return string value,and won't take ownership
    println!("s2 = {}", s2);
    s2 += &*format!("{}", " format!");
    println!("s2 = {}", s2);
    let s3 = format!("{}-{}", s2, "2");
    println!("s3 = {}", s3);
    let str1 = String::from("hello");
    let str2 = String::from(",rust");
    let str3 = str1 + &str2;
    // println!("str3 = {}", str3);
    println!("str2 = {}", str2);
    // error: value borrowed here after move
    // println!("str1 = {}", str1);
    // value borrowed here
    // the first param is self not &self, so the value will be borrowed
    let x = str3.add("!");
    println!("x = {}", x);
    // error: value borrowed here after move
    // println!("str3 = {}", str3);
    let str4 = String::from("hello");
    // hide old param
    // error: the type `std::string::String` cannot be indexed by `{integer}`
    // let str4 = str4[0];
    println!("str4 = {}", str4);
    println!("str4 len = {}", str4.len());
    // error: the size for values of type `str` cannot be known at compilation time
    // println!("str4 len = {}", str4[0..2]);
    for c in str4.chars() {
        println!("{}", c);
    }
}
