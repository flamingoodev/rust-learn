use std::panic;
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn t_panic() {
    let result = panic::catch_unwind(|| println!("{}", "hello"));
    assert!(result.is_ok());
    let result1 = panic::catch_unwind(|| panic!("panic"));
    assert!(result1.is_err());
    println!("{}", sum(1, 2));
}
