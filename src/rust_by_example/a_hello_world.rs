/// 1.hello world
// This is a function
fn hello_world() {
    // Statements here are executed when the compiled binary is called
    // Print text to the console
    println!("Hello World!");
}

// println! 是一个宏（macros），可以将文本输出到控制台（console）。
// 使用 Rust 的编译器 rustc 可以从源程序生成可执行文件：
// ```$ rustc hello.rs```
// 使用 rustc 编译后将得到可执行文件 hello。
// ```$ ./hello
// Hello World!```

#[test]
fn t_hello_world() {
    hello_world();
}

/// 2.注释

fn comment() {
    // 这是行注释的例子
    // 注意有两个斜线在本行的开头
    // 在这里面的所有内容都不会被编译器读取

    // println!("Hello, world!");

    // 请运行一下，你看到结果了吗？现在请将上述语句的两条斜线删掉，并重新运行。

    /*
     * 这是另外一种注释——块注释。一般而言，行注释是推荐的注释格式，
     * 不过块注释在临时注释大块代码特别有用。/* 块注释可以 /* 嵌套, */ */
     * 所以只需很少按键就可注释掉这些 main() 函数中的行。/*/*/* 自己试试！*/*/*/
     */

    /*
    注意，上面的例子中纵向都有 `*`，这只是一种风格，实际上这并不是必须的。
    */

    // 观察块注释是如何简单地对表达式进行修改的，行注释则不能这样。
    // 删除注释分隔符将会改变结果。
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}

#[test]
fn t_comment() {
    comment();
}

/// 3.格式化输出

// 打印操作由 std::fmt 里面所定义的一系列宏来处理，包括：
//
// format!：将格式化文本写到字符串。
// print!：与 format! 类似，但将文本输出到控制台（io::stdout）。
// println!: 与 print! 类似，但输出结果追加一个换行符。
// eprint!：与 print! 类似，但将文本输出到标准错误（io::stderr）。
// eprintln!：与 eprint! 类似，但输出结果追加一个换行符。

fn formatted_print() {
    // 通常情况下，`{}` 会被任意变量内容所替换。
    // 变量内容会转化成字符串。
    println!("{} days", 31);

    // 不加后缀的话，31 就自动成为 i32 类型。
    // 你可以添加后缀来改变 31 的类型（例如使用 31i64 声明 31 为 i64 类型）。

    // 用变量替换字符串有多种写法。
    // 比如可以使用位置参数。
    let alice = "Alice";
    let bob = "Bob";
    println!("{0}, this is {1}. {1}, this is {0}", alice, bob);

    // 可以使用命名参数。
    let obj = "the lazy dog";
    let sub = "the quick brown fox";
    let verb = "jumps over";
    println!(
        "{subject} {verb} {object}",
        object = obj,
        subject = sub,
        verb = verb
    );

    // 可以在 `:` 后面指定特殊的格式。
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // 你可以按指定宽度来右对齐文本。
    // 下面语句输出 "     1"，5 个空格后面连着 1。
    println!("{number:>width$}", number = 1, width = 6);

    // 你可以在数字左边补 0。下面语句输出 "000001"。
    println!("{number:>0width$}", number = 1, width = 6);

    // println! 会检查使用到的参数数量是否正确。
    // println!("My name is {0}, {1} {0}", "Bond");
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // 改正 ^ 补上漏掉的参数："James"

    // 创建一个包含单个 `i32` 的结构体（structure）。命名为 `Structure`。
    #[allow(dead_code)]
    struct Structure(i32);

    // 但是像结构体这样的自定义类型需要更复杂的方式来处理。
    // 下面语句无法运行。
    // println!("This struct `{}` won't print...", Structure(3));
    // 改正 ^ 注释掉此行。
}

// std::fmt 包含多种 trait（特质）来控制文字显示，其中重要的两种 trait 的基本形式如下：
//
// fmt::Debug：使用 {:?} 标记。格式化文本以供调试使用。
// fmt::Display：使用 {} 标记。以更优雅和友好的风格来格式化文本。

#[test]
fn t_formatted_print() {
    formatted_print();
}

/// 3.1调试（Debug）

// 所有的类型，若想用 std::fmt 的格式化打印，都要求实现至少一个可打印的 traits。 自动的实现只为一些类型提供，比如 std 库中的类型。所有其他类型 都必须手动实现。
//
// fmt::Debug 这个 trait 使这项工作变得相当简单。所有类型都能推导（derive，即自 动创建）fmt::Debug 的实现。但是 fmt::Display 需要手动实现。

// 这个结构体不能使用 `fmt::Display` 或 `fmt::Debug` 来进行打印。
struct UnPrintable(i32);

// `derive` 属性会自动创建所需的实现，使这个 `struct` 能使用 `fmt::Debug` 打印。
#[derive(Debug)]
struct DebugPrintable(i32);

// 推导 `Structure` 的 `fmt::Debug` 实现。
// `Structure` 是一个包含单个 `i32` 的结构体。
#[derive(Debug)]
struct Structure(i32);

// 将 `Structure` 放到结构体 `Deep` 中。然后使 `Deep` 也能够打印。
#[derive(Debug)]
struct Deep(Structure);

// 所有 std 库类型都天生可以使用 {:?} 来打印：
fn std_print() {
    // 使用 `{:?}` 打印和使用 `{}` 类似。
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // `Structure` 也可以打印！
    println!("Now {:?} will print!", Structure(3));

    // 使用 `derive` 的一个问题是不能控制输出的形式。
    // 假如我只想展示一个 `7` 怎么办？
    println!("Now {:?} will print!", Deep(Structure(7)));
}

#[test]
fn t_std_print() {
    std_print();
}

/// 3.2显示（Display）
// fmt::Debug 通常看起来不太简洁，因此自定义输出的外观经常是更可取的。这需要通过 手动实现 fmt::Display 来做到。fmt::Display 采用 {} 标记。实现方式看 起来像这样：

#[allow(unused)]
fn display() {
    // （使用 `use`）导入 `fmt` 模块使 `fmt::Display` 可用
    use std::fmt;

    // 定义一个结构体，咱们会为它实现 `fmt::Display`。以下是个简单的元组结构体
    // `Structure`，包含一个 `i32` 元素。
    struct Structure(i32);

    // 为了使用 `{}` 标记，必须手动为类型实现 `fmt::Display` trait。
    impl fmt::Display for Structure {
        // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
            // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。
            write!(f, "{}", self.0)
        }
    }
}

#[test]
fn t_display() {
    display();
}

// 对于任何非泛型的容器类型， fmt::Display 都能够实现
use std::fmt; // 导入 `fmt`

// 带有两个数字的结构体。推导出 `Debug`，以便与 `Display` 的输出进行比较。
#[derive(Debug)]
struct MinMax(i64, i64);

// 实现 `MinMax` 的 `Display`。
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用 `self.number` 来表示各个数据。
        write!(f, "({}, {})", self.0, self.1)
    }
}

// 为了比较，定义一个含有具名字段的结构体。
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// 类似地对 `Point2D` 实现 `Display`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 自定义格式，使得仅显示 `x` 和 `y` 的值。
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn display_1() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // 报错。`Debug` 和 `Display` 都被实现了，但 `{:b}` 需要 `fmt::Binary`
    // 得到实现。这语句不能运行。
    // println!("What does Point2D look like in binary: {:b}?", point);
}

#[test]
fn t_display1() {
    display_1();
}

/// 3.3测试实例：List
// 对一个结构体实现 fmt::Display，其中的元素需要一个接一个地处理到，这可能会很麻 烦。问题在于每个 write! 都要生成一个 fmt::Result。正确的实现需要 处理所有的 Result。Rust 专门为解决这个问题提供了 ? 操作符。
//
// 在 write! 上使用 ? 会像是这样：

// 对 `write!` 进行尝试（try），观察是否出错。若发生错误，返回相应的错误。
// 否则（没有出错）继续执行后面的语句。
// write!(f, "{}", value)?;

// 另外，你也可以使用 try! 宏，它和 ? 是一样的。这种写法比较罗嗦，故不再推荐， 但在老一些的 Rust 代码中仍会看到。使用 try! 看起来像这样：
// try!(write!(f, "{}", value));

// 有了 ?，对一个 Vec 实现 fmt::Display 就很简单了：

// 定义一个包含单个 `Vec` 的结构体 `List`。
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用元组的下标获取值，并创建一个 `vec` 的引用。
        let vec = &self.0;

        write!(f, "[")?;

        // 使用 `v` 对 `vec` 进行迭代，并用 `count` 记录迭代次数。
        for (count, v) in vec.iter().enumerate() {
            // 对每个元素（第一个元素除外）加上逗号。
            // 使用 `?` 或 `try!` 来返回错误。
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }

        // 加上配对中括号，并返回一个 fmt::Result 值。
        write!(f, "]")
    }
}

#[test]
fn t_list_display() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}

/// 3.4格式化
// 我们已经看到，格式化的方式是通过格式字符串来指定的：
//
// format!("{}", foo) -> "3735928559"
// format!("0x{:X}", foo) -> "0xDEADBEEF"
// format!("0o{:o}", foo) -> "0o33653337357"

// 根据使用的参数类型是 X、o 还是未指定，同样的变量（foo）能够格式化 成不同的形式。
//
// 这个格式化的功能是通过 trait 实现的，每种参数类型都对应一种 trait。最常见的格式 化 trait 就是 Display，它可以处理参数类型为未指定的情况，比如 {}。
use std::fmt::{Display, Formatter};

struct City {
    name: &'static str,
    // 纬度
    lat: f32,
    // 经度
    lon: f32,
}

impl Display for City {
    // `f` 是一个缓冲区（buffer），此方法必须将格式化后的字符串写入其中
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` 和 `format!` 类似，但它会将格式化后的字符串写入
        // 一个缓冲区（即第一个参数f）中。
        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn format() {
    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", *city);
    }
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        // 在添加了针对 fmt::Display 的实现后，请改用 {} 检验效果。
        println!("{:?}", *color)
    }
}

#[test]
fn t_format() {
    format();
}
