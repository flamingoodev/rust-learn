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
