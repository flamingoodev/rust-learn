fn inverse(number: f64) -> f64 {
    number * -1.
}

fn double(number: f64) -> Option<f64> {
    Option::from(number * 2.)
}

fn none(number: f64) -> Option<f64> {
    println!("{}", number);
    Option::None
}

#[test]
fn t_option_test() {
    let number: f64 = 64.;
    // let number: f64 = 64.f64;
    println!("{}", number);
    let result = Option::from(number);
    let r1 = result.map(inverse).unwrap();
    let r2 = result.map(|x| x * -1.).unwrap();
    let r3 = result.map(|x| x * -1.).and_then(double).unwrap();
    // Panic 'called `Option::unwrap()` on a `None` value'
    // let r4 = result
    //     .map(|x| x * -1.)
    //     .and_then(none)
    //     .unwrap();
    let r4 = result.map(|x| x * -1.).and_then(none);
    match r4 {
        None => {
            println!("This is failed.")
        }
        Some(x) => {
            println!("Result was {}.", x)
        }
    }
    println!("{}", r1);
    println!("{}", r2);
    println!("{}", r3);
}
