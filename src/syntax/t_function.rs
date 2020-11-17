/// 函数自身可以作为函数的参数和返回值使用
/// 函数自身作为参数传递到函数时，称作函数指针类型（Fn-pointer Type）
fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a, b)
}
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
fn product(a: i32, b: i32) -> i32 {
    a * b
}
/// CTFE机制，编译时函数执行（Compile-Time Function Execution，CFTE）
const fn init_len() -> usize {
    5
}
#[test]
fn t_math() {
    let i = math(sum, 5, 10);
    let j = math(product, 5, 10);
    println!("{}, {}", i, j);
}
#[test]
fn t_init_len() {
    let arr = [0; init_len()];
    println!("{:?}", arr);
}
