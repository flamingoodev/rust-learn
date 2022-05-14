use std::fmt;

pub fn vec_new() {
    // use Vec::new()
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v[0] = {}", v[0]);
    println!("v[1] = {}", v[1]);
    println!("v[2] = {}", v[2]);
    let v0_copy = v[0];
    println!("v0_copy = {}", v0_copy);
    // use vec!
    let v1 = vec![4, 5, 6];
    for item in v1 {
        println!("v1 item = {}", item);
    }
    // scope
    {
        let v2 = vec![4, 5, 6];
        for item in v2 {
            println!("v2 item = {}", item);
        }
    }
    // print v2[0]
    // error: cannot find value `v2` in this scope
    // println!("v2[0] = {}", v2[0]);

    // update v[2]
    v[2] = 1;
    println!("v[2] = {}", v[2]);
    // get v[0]
    let v0 = v.get(0);
    match v0 {
        Some(item) => println!("The v0 value is: {}", item),
        None => println!("There is no v0."),
    }
    match v.get(5) {
        Some(item) => println!("The v5 value is: {}", item),
        None => println!("There is no v5."),
    }
    // does not exist
    // error: index out of bounds
    // let does_not_exist = &v[5];
    let v_first = &v[0];
    // error: cannot borrow `v` as mutable because it is also borrowed as immutable
    // v.push(3);
    println!("The first element is {}", v_first);
    let v3 = vec![100, 200, 300];
    for mut i in v3 {
        // will be auto resolve reference, also like *i
        i += 10;
        println!("{}", i);
    }
}

#[warn(dead_code)]
fn if_let() -> u8 {
    let boolean = true;
    let mut binary = 0;
    if let true = boolean {
        binary = 1;
    }
    binary
}

fn vec_slice() {
    let v = vec![1, 2, 3, 4];

    // Vec 实现了 Deref，&Vec<T> 会被自动解引用为 &[T]，符合接口定义
    print_slice(&v);
    // 直接是 &[T]，符合接口定义
    print_slice(&v[..]);

    // &Vec<T> 支持 AsRef<[T]>
    print_slice1(&v);
    // &[T] 支持 AsRef<[T]>
    print_slice1(&v[..]);
    // Vec<T> 也支持 AsRef<[T]>
    print_slice1(v);

    let arr = [1, 2, 3, 4];
    // 数组虽没有实现 Deref，但它的解引用就是 &[T]
    print_slice(&arr);
    print_slice(&arr[..]);
    print_slice1(&arr);
    print_slice1(&arr[..]);
    print_slice1(arr);
}

// 注意下面的泛型函数的使用
fn print_slice<T: fmt::Debug>(s: &[T]) {
    println!("{:?}", s);
}

fn print_slice1<T, U>(s: T)
where
    T: AsRef<[U]>,
    U: fmt::Debug,
{
    println!("{:?}", s.as_ref());
}

#[test]
fn test_if_let() {
    assert_eq!(if_let(), 1);
}

#[warn(dead_code)]
fn while_let() {
    let mut v = vec![1, 2, 3, 4, 5];
    // loop
    loop {
        match v.pop() {
            Some(x) => println!("loop: {}", x),
            None => break,
        }
    }
    // while let
    while let Some(x) = v.pop() {
        println!("while let: {}", x)
    }
}

#[test]
fn test_while_let() {
    while_let()
}

#[test]
fn test_slice() {
    vec_slice()
}
