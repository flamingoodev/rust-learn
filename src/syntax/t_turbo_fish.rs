use std::borrow::Cow;

fn turbo_fish() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let even_numbers = numbers
        .into_iter()
        .filter(|n| n % 2 == 0)
        .collect::<Vec<i32>>();
    // .collect::<Vec<_>>();
    // .collect();

    println!("{:?}", even_numbers.len());

    let s = "Hello, World!";
    // fn into(self) -> T;
    // let string = s.into::<String>();
    let string1 = <&str as Into<String>>::into(&s);
    let string2 = Into::<String>::into(s);
    let string3 = String::from(s);
    let string4: String = s.into();
    println!("{} {} {} {}", string1, string2, string3, string4);
    println!("{}", s)
}

#[test]
fn t_turbo_fish() {
    turbo_fish();
}
