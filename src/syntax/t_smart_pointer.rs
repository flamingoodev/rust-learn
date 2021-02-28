use std::borrow::Cow;
use std::cell::{Cell, RefCell};

/// 内部可变性 Cell<T> RefCell<T>
/// 对于没有实现copy的类型，使用Cell<T>有许多不便，RefCell<T>没有对T进行copy限制
/// RefCell有运行时开销，因为它内部维护这一个运行时借用检查器，如果在运行时违反了借用检查规则，则会触发panic
/// * Cell<T>和RefCell<T>使用最多的场景就是配合只读引用来使用
/// 区别：
/// 1、Cell<T>使用get/set方法来操作被包裹的值，RefCell<T>通过borrow/borrow_mut返回包装过的
/// 引用Ref<T>和RefMut<T>来操作被包裹的值。
/// 2、Cell<T>一般适合复制语义类型（实现了copy），RefCell<T>一般适合移动语义类型（未实现copy）。
/// 3、Cell<T>无运行时开销，并且永远不会在运行时触发panic错误。RefCell<T>需要在运行时执行借用检查，
/// 所以有运行时开销，一旦发现违反借用检查的情况，则会引发线程panic而退出当前线程。
///
#[allow(dead_code)]
pub fn t_smart_pointer_1() {
    let x = RefCell::new(String::from("RefCell<T>"));
    println!("{:?}", x.borrow());
    x.borrow_mut().push('T');
    println!("x = {:?}", x);
    println!("x.borrow() = {:?}", x.borrow());
    let y1 = x.borrow_mut();
    // will panic, thread main will be panic
    // panicked at 'already borrowed: BorrowMutError'
    // let y2 = x.borrow_mut();
}
struct Foo {
    x: u32,
    y: Cell<u32>,
}

/// Cell<T>：实现了copy的类型能够任意读取和写入，未实现copy的则可使用get_mut()方法来进行操作
/// Cell<T>没有运行时开销，但是每次get/set都会执行一次按位复制，故不建议包裹大的结构体
pub fn t_smart_pointer_2() {
    let foo = Foo {
        x: 1,
        y: Cell::new(2),
    };
    assert_eq!(1, foo.x);
    assert_eq!(2, foo.y.get());
    foo.y.set(3);
    assert_eq!(3, foo.y.get());
}

/// 写时复制（copy on write）Cow<T>
/// Cow<T>表示的是所有权的借用和拥有
/// Cow<T>实现了Deref，这意味着可以直接调用其包含数据的不可变方法
/// Cow<T>提供的功能是：以不可变的方式访问借用内容，以及在可变借用或所有权的时候再克隆一份数据
/// Cow<T>旨在减少复制操作，提高性能，一般用于读多写少的场景。
/// Cow<T>是一个枚举体的智能指针，包括两个可选值：
/// Borrowed，用于包裹引用。
/// Owned，用于包裹所有者。
///
pub fn t_smart_pointer_3() {
    // 这里没有可变需求，所以不会克隆
    let s1 = [1, 2, 3];
    let mut i1 = Cow::from(&s1[..]);
    abs_all(&mut i1);
    println!("IN: {:?}", s1);
    println!("OUT: {:?}", i1);
    // 这里有可变需求，所以会克隆
    // 注意：借用数据被克隆为了新的对象
    // s2 != i2，实际上s2不可变，也不会被改变
    let s2 = [1, 2, 3, -4, -5, -6];
    let mut i2 = Cow::from(&s2[..]);
    abs_all(&mut i2);
    println!("IN: {:?}", s2);
    println!("OUT: {:?}", i2);
    // 这里不会克隆，因为数据本身就拥有所有权
    // 注意：在本例中，i3本身就是可变的
    let mut i3 = Cow::from(vec![1, 2, -3, 4]);
    abs_all(&mut i3);
    println!("IN/OUT: {:?}", i3);
}

fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            input.to_mut()[i] = -v;
        }
    }
}
fn abs_sum(ns: &[i32]) -> i32 {
    let mut lst = Cow::from(ns);
    abs_all(&mut lst);
    lst.iter().fold(0, |acc, &n| acc + n)
}
#[test]
fn test_1() {
    t_smart_pointer_1();
    t_smart_pointer_2();
    t_smart_pointer_3();
}
