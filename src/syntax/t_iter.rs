pub fn t_iter() {
    // iterator
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    // for val in v1_iter {
    //     println!("val = {}", val);
    // }
    if let Some(v) = v1_iter.next() {
        println!("v1_iter.next() = {}", v);
    }
    // iterator mut reference
    let mut v2 = vec![1, 2, 3];
    let mut v2_iter = v2.iter_mut();
    if let Some(v) = v2_iter.next() {
        *v = 3;
        println!("v1_iter.next() = {}", v);
    }
    let total: i32 = v2.iter().sum();
    println!("total = {}", total);
    let v3 = vec![1, 2, 3, 4, 5];
    println!("v3 = {:?}", v3);
    let v3_i: Vec<_> = v3.iter().map(|x| x + 1).collect();
    println!("v3_i = {:?}", v3_i);
    let v4_i: Vec<_> = v3.iter().filter(|x| *x > &3).collect();
    println!("v4_i = {:?}", v4_i);
}
