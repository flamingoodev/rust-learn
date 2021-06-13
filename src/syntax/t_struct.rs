#[derive(Debug, Copy, Clone)]
struct Book<'a> {
    name: &'a str,
    isbn: i32,
    version: i32,
}

#[derive(Debug)]
struct BookMove {
    name: String,
    isbn: i32,
    version: i32,
}

#[test]
pub fn t_struct_test() {
    let book1 = Book {
        name: "Rust权威指南",
        isbn: 201801,
        version: 1,
    };
    // 结构体更新语法
    let book2 = Book {
        name: "Rust",
        ..book1
    };
    // 因为结构体是复制语义，故输出book1时并未转移所有权
    println!("{:?}", book1);
    println!("{:?}", book2);
}

#[test]
pub fn t_struct_move_test() {
    let book1 = Book {
        name: "Rust权威指南",
        isbn: 201801,
        version: 1,
    };
    // 结构体更新语法
    let book2 = Book {
        name: "Rust",
        ..book1
    };
    // 因为结构体是复制语义，故输出book1时并未转移所有权
    println!("{:?}", book1.name);
    println!("{:?}", book1);
    println!("{:?}", book2);
}
