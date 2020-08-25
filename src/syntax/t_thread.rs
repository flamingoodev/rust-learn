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
