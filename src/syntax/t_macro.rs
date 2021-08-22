macro_rules! unless {
    ($arg: expr => $branch: tt) => {
        if !$arg {
            $branch
        }
    };
}

fn cmp(a: i32, b: i32) {
    unless!(
        (a > b) => { println!("{} < {}", a, b) }
    )
}

#[test]
fn t_macro() {
    cmp(1, 2);
}
