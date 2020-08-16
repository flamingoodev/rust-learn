enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn t_enum() {
    let row = vec![
        SpreadsheetCell::Int(10),
        SpreadsheetCell::Float(10.10),
        SpreadsheetCell::Text(String::from("blue")),
    ];
    for cell in row {
        match cell {
            SpreadsheetCell::Int(i) => println!("Int value is {}", i),
            SpreadsheetCell::Float(f) => println!("Float value is {}", f),
            SpreadsheetCell::Text(t) => println!("Text value is {}", t),
        }
    }
}
