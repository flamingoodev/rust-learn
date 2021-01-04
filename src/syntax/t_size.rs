use std::mem::size_of_val;
use std::mem::size_of;

struct Test<'a> {
    _a: i32,
    _b: &'a [u8],
}

pub fn test_size() {
    let array: [u8; 10] = [1; 10];
    let slice = &array;
    println!("&i32 size = {}", size_of::<&i32>());
    println!("&i64 size = {}", size_of::<&i64>());
    println!("i32 size = {}", size_of::<i32>());
    println!("i64 size = {}", size_of::<i64>());
    println!("&u8 size = {}", size_of::<&u8>());
    println!("array size = {}", size_of_val(slice));
}