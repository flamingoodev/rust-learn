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
