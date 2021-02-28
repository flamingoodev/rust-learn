// use rust_learn::games::guess_number;
use rust_learn::collections::t_map;
use rust_learn::collections::t_vector;
use rust_learn::routes::{role, user};
use rust_learn::syntax::t_channel;
use rust_learn::syntax::t_closure;
use rust_learn::syntax::t_enum;
use rust_learn::syntax::t_error;
use rust_learn::syntax::t_generic::Point;
use rust_learn::syntax::t_iter;
use rust_learn::syntax::t_queue;
use rust_learn::syntax::t_size;
use rust_learn::syntax::t_string;
use rust_learn::syntax::t_thread;
use rust_learn::syntax::t_trait::{NewsArticle, Summary, Summary1, Tweet};

fn main() {
    role::pub_print_role();
    role::pub_print_role();
    // guess_number::start();
    user::called_by_super();
    user::called_by_absolutely_path();
    t_vector::vec_new();
    t_enum::t_enum();
    t_string::t_string();
    t_map::t_map();
    let result1 = t_error::read_from_file_1();
    match result1 {
        Ok(f) => println!("s1 = {}", f),
        Err(err) => println!("err1 = {}", err),
    }
    let result2 = t_error::read_from_file_2();
    match result2 {
        Ok(f) => println!("s2 = {}", f),
        Err(err) => println!("err2 = {}", err),
    }
    let result3 = t_error::read_from_file_3();
    match result3 {
        Ok(f) => println!("s3 = {}", f),
        Err(err) => println!("err3 = {}", err),
    }
    let p = Point { x: 5.0, y: 10.1 };
    println!("p.x = {}", p.x());
    let p1 = Point { x: 5, y: 10 };
    println!("p1.x = {}", p1.usize_x());
    let article = NewsArticle {
        headline: String::from("headline"),
        location: String::from("shanghai"),
        author: String::from("ff"),
        content: String::from("This is content"),
    };
    let article1 = NewsArticle {
        headline: String::from("headline1"),
        location: String::from("shanghai"),
        author: String::from("ff"),
        content: String::from("This is content"),
    };
    println!("New article available! {}", article1.summarize1());
    println!("{:#?}", article);
    println!("{:?}", article);
    let string = article.summarize();
    println!("{}", string);
    let tweet = Tweet {
        username: "ff".to_string(),
        content: "This is content".to_string(),
        reply: false,
        retweet: false,
    };
    let string1 = tweet.summarize();
    println!("{}", string1);
    t_closure::t_closure();
    t_iter::t_iter();
    let i = u8::max_value();
    println!("u8 max_value = {}", i);
    // error: attempt to add with overflow
    // let j = i + 1;
    // println!("u8 max_value + 1 = {}", j);
    t_thread::t_thread();
    t_channel::t_channel();
    println!("---------------------Queue----------------------");
    let mut queue = t_queue::Queue::new(5);
    queue.enqueue(10);
    queue.enqueue(20);
    queue.enqueue(30);
    queue.enqueue(40);
    queue.enqueue(50);
    println!("size = {}, data = {:#?}", queue.size(), queue);
    for _ in 0..10 {
        if let Some(v) = queue.dequeue() {
            println!("dequeue: {}", v);
        } else {
            println!("dequeue failed!");
        }
    }
    println!("size = {}, data = {:#?}", queue.size(), queue);
    println!("---------------------Queue----------------------");
    t_size::test_size();
}
