use std::fs::File;
use std::io::{Error, Read};
use std::option::Option::Some;
use std::panic;
use std::panic::Location;

pub fn read_from_file_1() -> Result<String, Error> {
    let f = File::open("src/resources/hello.txt");
    let mut f = match f {
        Ok(f) => f,
        Err(err) => return Err(err),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(err) => Err(err),
    }
}

pub fn read_from_file_2() -> Result<String, Error> {
    let mut f = File::open("src/resources/hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Recommended way
pub fn read_from_file_3() -> Result<String, Error> {
    let mut s = String::new();
    File::open("src/resources/hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

pub fn panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        match panic_info.location() {
            None => {}
            Some(location) => {
                // println!("{:?}", location)
            }
        }
        if let Some(location) = panic_info.location() {
            println!("{:?}", location);
            println!(
                "panic occurred '{}' at line {}",
                location.file(),
                location.line()
            );
        } else {
            println!("can't get location information...");
        }
    }));
    panic::catch_unwind(|| {
        panic!("panic");
    });
    println!("{}", "continue...")
}

#[test]
fn test_panic_hook() {
    panic_hook();
}
