/// 闭包和函数有一个重要的区别：闭包可以捕获外部变量，而函数不可以。
/// 闭包是由一个匿名结构体和trait组合来实现的
/// 闭包可以作为参数传递，如下：
fn math<F: Fn() -> i32>(op: F) -> i32 {
    op()
}

/// 闭包可以作为返回值
fn two_times_impl() -> impl Fn(i32) -> i32 {
    let i = 2;
    // move关键字，捕获i的所有权转移到闭包中，避免使用引用进行捕获变量（垂悬指针）
    move |j| j * i
}
#[test]
fn t_two() {
    let result = two_times_impl();
    assert_eq!(result(2), 4);
}
#[test]
fn t_math() {
    let a = 2;
    let b = 3;
    assert_eq!(math(|| a + b), 5);
    assert_eq!(math(|| a * b), 6);
}

struct Catchier<T>
where
    T: Fn(u32) -> u32,
{
    cal: T,
    val: Option<u32>,
}

impl<T> Catchier<T>
where
    T: Fn(u32) -> u32,
{
    fn new(cal: T) -> Catchier<T> {
        Catchier { cal, val: None }
    }
    fn val(&mut self, arg: u32) -> u32 {
        match self.val {
            None => {
                let v = (self.cal)(arg);
                self.val = Some(v);
                v
            }
            Some(val) => val,
        }
    }
}

pub fn add_one_v1(x: u32) -> u32 {
    x + 1
}

pub fn t_closure() {
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let a = add_one_v1(5);
    let b = add_one_v2(a);
    let c = add_one_v3(b);
    //error
    // let d = add_one_v3(String::from("hello"));
    println!("a = {}, b= {}, c = {}", a, b, c);
    let mut c = Catchier::new(|x| x + 1);
    let v = c.val(1);
    println!("v = {}", v);
}
