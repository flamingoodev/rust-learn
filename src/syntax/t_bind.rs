// `age` 函数，返回一个 `u32` 值。
fn age() -> u32 {
    15
}

#[test]
fn t_bind_test() {
    println!("Tell me type of person you are");
    match age() {
        0 => println!("I'm not born yet I guess"),
        // 不能直接 `匹配（match）` 1 ... 12，但是孩子是几岁呢？
        // 相反，将 1 ... 12 序列绑定到 `n` 。现在年龄就可以读取了。
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        // 没有绑定。返回结果。
        n => println!("I'm an old person of age {:?}", n),
    }
}
