/// 闭包和函数有一个重要的区别：闭包可以捕获外部变量，而函数不可以。
/// 闭包是由一个匿名结构体和trait组合来实现的
/// move关键字是强制让环境变量的所有权转移到闭包中而不管是不是发生了所有权的转移
/// move关键字和匿名函数是否是FnOnce没有必然联系，之和匿名函数体有关
/// 当匿名函数体里转移了环境变量的所有权的时候，匿名函数就是FnOnce。
/// 当匿名函数体里改变了环境变量的值的时候，匿名函数就是FnMut。
/// 否则匿名函数就是Fn。
/// 闭包可以作为参数传递，如下：
#[warn(dead_code)]
fn math<F: Fn() -> i32>(op: F) -> i32 {
    op()
}

/// 闭包可以作为返回值
#[allow(dead_code)]
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

/// 关于move修饰的匿名函数需要注意的2点

/// 1.如果函数不是FnOnce，此匿名函数可以重复调用
#[test]
pub fn t_fn() {
    let mut x = vec![1];
    let mut incr_x = move || {
        println!("{:?}", x);
        x.push(1);
    };
    incr_x();
    incr_x();
    incr_x();
}

/// 2.如果捕获变量是复制语义类型，则闭包会实现Copy/Clone。如果捕获变量是移动语义类型，则闭包不会实现Copy/Clone。普通闭包都是移动语义
fn call<F: FnOnce()>(f: F) { f() }

#[test]
fn t_fn_move() {
    // 复制语义 未使用 move （普通闭包都是移动语义）
    let mut x = 0;
    let incr_x = || x += 1;
    call(incr_x);
    // call(incr_x); // ERROR: 'incr_x' moved in the call above.
    println!("x = {}", x);
    // 复制语义 使用 move （x被copy/clone）
    let mut x = 0;
    let incr_x = move || x += 1;
    call(incr_x);
    call(incr_x);
    println!("x = {}", x);
    // 移动语义 使用move （move到闭包中）
    let mut x = vec![];
    let expend_x = move || x.push(42);
    call(expend_x);
    // call(expend_x); //ERROR: use of moved value: 'expend_x'
}
