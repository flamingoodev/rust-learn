use std::sync::mpsc;
// use std::sync::mpsc::TryRecvError;
use std::thread;
// use std::thread::sleep;
use std::time::Duration;

pub fn t_channel() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let val = String::from("hi");
        println!("Send: {}", val);
        tx.send(val).unwrap();
        // error will be moved
        // println!("Send: {}", val);
    });
    // await
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);
    // sync
    handle.join().unwrap();
    let received = rx.try_recv();
    match received {
        Ok(received) => {
            println!("Got: {}", received);
        }
        Err(err) => {
            println!("Got error: {}", err);
        }
    };
    println!("--------------------------");
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let v = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for i in v {
            let x = sender.send(i);
            println!("x = {:?}", x);
            thread::sleep(Duration::from_secs(1));
        }
    });
    for re in receiver {
        println!("got: {}", re);
    }
    println!("--------------------------");
}
