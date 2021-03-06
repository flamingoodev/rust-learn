use std::mem::size_of_val;

// use crate::syntax::t_iter::t_iter;
// this annotation will ignore test when cargo build
#[cfg(test)]
#[test]

fn it_works() {
    assert_eq!(2 + 2, 4);
}
#[test]
#[should_panic(expected = "Make this test fail")]
fn another() {
    // use cargo test -- --nocapture will print
    println!("test another");
    // will pass
    panic!("Make this test fail");
    // will fail
    // panic!("panic");
}
#[test]
fn test_it_works() {
    it_works();
}
#[test]
fn test_iter() {
    // syntax::t_iter();
}
#[test]
fn test_result() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("Error"))
    }
}

#[test]
fn test_size() {
    let array: [u8; 10] = [1; 10];
    let slice = &array;
    println!("array size = {}", size_of_val(slice));
}
