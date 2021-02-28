use std::thread;
use std::time::Duration;

pub fn t_thread() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("thread1: number = {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    println!("--------------------------");
    // wait child thread finish
    handle.join().unwrap();
    println!("--------------------------");
    thread::spawn(|| {
        for i in 10..20 {
            println!("thread2: number = {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 20..30 {
        println!("main thread: number = {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    let v = vec![1, 2, 3];
    thread::spawn(move || println!("{:?}", v));
    // error value borrowed here after move
    // println!("{:?}", v)
}

pub fn t_thread2() {
    // 创建一个不可变变量
    let x = vec![1, 2, 3];
    // 闭包捕获变量的所有权，并传递到子线程中，因为变量不可变，所以多线程之间共享数据是安全的
    thread::spawn(|| x);
}

pub fn t_thread3() {
    // error
    // closure may outlive the current function, but it borrows `x`, which is owned by the current function
    // let mut x = vec![1, 2, 3];
    // thread::spawn(|| {
    //     x.push(4)
    // });
    // x.push(5);
}

// fix t_thread3
pub fn t_thread4() {
    let mut x = vec![1, 2, 3];
    // vec为原生数据类型，默认实现了send和sync标签trait，所以可以正常move所有权
    // 若用Rc容器包装数组，由于Rc容器未实现send和sync标签trait，move所有权将会报错
    // Rc是用于引用计数的智能指针
    // let mut x = Rc::new(vec![1, 2, 3]);
    // move关键字将所有权转移给了子线程，主线程将无法访问该变量
    thread::spawn(move || x.push(4));
    // error
    // x.push(5);
}

pub fn t_thread5() {
    let a = "Hello".to_string();
    let b = " World".to_string();
    // String的右值必须是&str
    // 但是由于String默认实现了Deref，&String会被默认转换为&str，故可以正常运行
    let c = a + &b;
    println!("{:?}", c);
}

#[test]
pub fn t_thread2_test() {
    t_thread2();
    t_thread4();
    t_thread5();
}
