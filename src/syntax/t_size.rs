use std::mem::size_of_val;

struct Test<'a> {
    _a: i32,
    _b: &'a [u8],
}

pub fn test_size() {
    let array: [u8; 10] = [1; 10];
    let slice = &array;
    println!("array size = {}", size_of_val(slice));
}