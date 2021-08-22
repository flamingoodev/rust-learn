#[allow(dead_code)]
fn t_type() {
    // num
    let num = 42u32;
    let num: u32 = 42;
    let num = 0x2A; // 十六进制
    let num = 0o106; // 八进制
    let num = 0b1101_1011; // 二进制
    println!("num: {}", num);
    assert_eq!(b'*', 42u8); //字节字面量
    assert_eq!(b'\'', 39u8);
    let num = 3.55f64;
    assert_eq!(-3.55, -3.55f64);
    assert_eq!(2., 2.0f64);
    assert_eq!(2e4, 20000f64);
    println!("{:?}", std::f32::INFINITY);
    println!("{:?}", std::f32::NEG_INFINITY);
    println!("{:?}", std::f32::NAN);
    println!("{:?}", std::f32::MIN);
    println!("{:?}", std::f32::MAX);
    // array
    // 当数组调用slice方法时会强制转换为slice类型，反之则不会，
    // 因为slice是动态的大小，数组是固定大小，不能由slice转换到数组。
    // 所以arr.iter()相当于(arr[..]).iter() 或 (&a[..]).iter()
    let arr: [i32; 3] = [1, 2, 3];
    let mut mut_arr = [1, 2, 3];
    let init_arr = [0; 10];
    assert_eq!(arr[0], mut_arr[0]);
    for i in init_arr.iter() {
        println!("{}", i);
    }
    // range
    assert_eq!((1..5), std::ops::Range { start: 1, end: 5 });
    assert_eq!((1..=5), std::ops::RangeInclusive::new(1, 5));
    assert_eq!(3 + 4 + 5, (3..6).sum());
    for i in 0..5 {
        println!("{}", i);
    }
    // slice
    // 切片（Slice）类型是对一个数组（包括固定大小数组和动态数组）的引用片段，有利于安全有效地访问数组的一部分，
    // 而不需要拷贝，切片引用的是已经存在的变量
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    assert_eq!(&arr, &[1, 2, 3, 4, 5]);
    println!("&arr[1..].len(): {}", &arr[1..].len());
    let arr1 = &mut arr;
    // &mut 定义可变切片，可以通过索引直接修改相应位置的值
    arr1[0] = 0;
    for i in arr1.iter() {
        println!("mut slice: {}", i);
    }
    // String
    // &str是不可变长度的字符串，String是可变长度的字符串
    let truth: &'static str = "Rust";
    // 原生指针（Raw pointer）：
    // 不可变指针 *const T
    // 可变指针 *mut T
    let ptr = truth.as_ptr();
    let len = truth.len();
    assert_eq!(4, len);
    let s = unsafe {
        let slice = std::slice::from_raw_parts(ptr, len);
        std::str::from_utf8(slice)
    };
    assert_eq!(s, Ok(truth));
    // 原生指针示例
    {
        // 可变指针
        let mut x = 10;
        let ptr_x = &mut x as *mut i32;
        // 不可变指针
        let y = Box::new(20);
        let ptr_y = &*y as *const i32;
        // 操作原生指针要使用unsafe
        unsafe {
            // 解应用
            *ptr_x += *ptr_y;
        }
        assert_eq!(x, 30);
    }
}

#[test]
fn test_t_type() {
    t_type();
}
