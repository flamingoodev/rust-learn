fn test_master_pointer() {
    // 装箱的字符串字面量会进行复制
    let a = Box::new("Hello");
    // 装箱的是引用语义，必须转移所有权
    let b = Box::new("Rust".to_string());
    let len = a.len();
    let c = b;
    // let d = a;
    // - value moved here
    let e = *a;
    println!("Box len is : {}", len);
    println!("Box deref : {}", e);
    println!("a : {}", a);
    // println!("b : {}", b); //   let c = b; - value moved here
    println!("c : {}", c);
}

#[test]
fn t_test_master_pointer() {
    test_master_pointer();
}
