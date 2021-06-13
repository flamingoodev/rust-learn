use std::num::ParseIntError;

#[test]
fn t_result_test() {
    let n = "2";
    // let n = "a";
    let r = n.parse::<i32>();
    match r {
        Ok(x) => {
            println!("{}", x);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

#[test]
fn t_result_handle() {
    match square("10") {
        Ok(x) => {
            println!("{}", x);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn square(number_str: &str) -> Result<i32, ParseIntError> {
    number_str.parse::<i32>().map(|x| x.pow(2))
}

type ParseResult<T> = Result<T, ParseIntError>;

fn square_1(number_str: &str) -> ParseResult<i32> {
    number_str.parse::<i32>().map(|x| x.pow(2))
}

#[test]
fn t_result_type() {
    let s = match square_1("10") {
        Ok(x) => {
            x
        }
        Err(_) => {
            0
        }
    };
    println!("{}", s);
    match i32::from_str_radix("10a", 10) {
        Ok(x) => {
            println!("{}", x);
        }
        Err(err) => {
            println!("{}", err)
        }
    }
}
