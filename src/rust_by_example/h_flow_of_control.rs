/// ### 流程控制
// 任何编程语言都包含的一个必要部分就是改变控制流程：if/else，for 等。让我们 谈谈 Rust 语言中的这部分内容。

/// 1 if/else
// if-else 分支判断和其他语言类似。
// 不同的是，Rust 语言中的布尔判断条件不必使用小括号包裹，且每个条件后面都跟着一个代码块。
// if-else 条件选择是一个表达式，并且所有分支都必须返回相同的类型。

fn if_else() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // 这个表达式返回一个 `i32` 类型。
            10 * n
        } else {
            println!(", and is a big number, half the number");

            // 这个表达式也必须返回一个 `i32` 类型。
            n / 2
            // 试一试 ^ 试着加上一个分号来结束这条表达式。
        };
    //   ^ 不要忘记在这里加上一个分号！所有的 `let` 绑定都需要它。

    println!("{} -> {}", n, big_n);
}

#[test]
fn t_if_else() {
    if_else();
}

/// 2. loop 循环
// Rust 提供了 loop 关键字来表示一个无限循环。
//
// 可以使用 break 语句在任何时候退出一个循环，还可以使用 continue 跳过循环体的剩余部分并开始下一轮循环。

fn flow_loop() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // 无限循环
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // 跳过这次迭代的剩下内容
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // 退出循环
            break;
        }
    }
}

#[test]
fn t_flow_loop() {
    flow_loop();
}

/// 2.1 嵌套循环和标签
// 在处理嵌套循环的时候可以 break 或 continue 外层循环。
// 在这类情形中，循环必须用一些 'label（标签）来注明，并且标签必须传递给 break/continue 语句。
#[allow(unreachable_code)]
fn label_loop() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // 这只是中断内部的循环
            //break;

            // 这会中断外层循环
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}

/// 2.2 从 loop 循环中返回
// loop 有个用途是尝试一个操作直到成功为止。若操作返回一个值，则可能需要将其传递给代码的其余部分：
// 将该值放在 break 之后，它就会被 loop 表达式返回。

#[allow(unused)]
fn return_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

/// 3 while 循环
// while 关键字可以用作当型循环（当条件满足时循环）。
//
// 让我们用 while 循环写一下臭名昭著的 FizzBuzz
// （补充：LeetCode 上的 FizzBuzz 问题描述[https://leetcode-cn.com/problems/fizz-buzz/]） 程序。

fn fizz_buzz() {
    // 计数器变量
    let mut n = 1;

    // 当 `n` 小于 101 时循环
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // 计数器值加 1
        n += 1;
    }
}

#[test]
fn t_fizz_buzz() {
    fizz_buzz();
}

/// 4. for 循环
// for 与区间
// for in 结构可以遍历一个 Iterator（迭代器）。
// 创建迭代器的一个最简单的方法是使用区间标记 a..b。这会生成从 a（包含此值） 到 b（不含此值）的，
// 步长为 1 的一系列值。
// 让我们使用 for 代替 while 来写 FizzBuzz 程序。

fn for_fizz_buzz() {
    // `n` 将在每次迭代中分别取 1, 2, ..., 100
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}


#[test]
fn t_for_fizz_buzz() {
    for_fizz_buzz();
    for_fizz_buzz_1();
}

// 或者，可以使用a..=b表示两端都包含在内的范围。上面的代码可以写成：

fn for_fizz_buzz_1() {
    // `n` 将在每次迭代中分别取 1, 2, ..., 100
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

/// 4.1 for 与迭代器
// for in 结构能以几种方式与 Iterator 互动。在 迭代器 trait 一节将会谈到，如果没有特别指定，
// for 循环会对给出的集合应用 into_iter 函数，把它转换成一个迭代器。这并不是把集合变成迭代器的唯一方法，
// 其他的方法有 iter 和iter_mut 函数。
//
// 这三个函数会以不同的方式返回集合中的数据。
//
// iter - 在每次迭代中借用集合中的一个元素。这样集合本身不会被改变，循环之后仍可以使用。

fn for_iter() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}

#[test]
fn t_for_iter() {
    for_iter();
    for_iter_1();
    for_iter_2();
}

// into_iter - 会消耗集合。在每次迭代中，集合中的数据本身会被提供。一旦集合被消耗了，之后就无法再使用了，
// 因为它已经在循环中被 “移除”（move）了。

fn for_iter_1() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}

// iter_mut - 可变地（mutably）借用集合中的每个元素，从而允许集合被就地修改。

fn for_iter_2() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);
}

// 在上面这些代码中，注意 match 的分支中所写的类型不同，这是不同迭代方式的关键区别。
// 因为类型不同，能够执行的操作当然也不同。

/// 5 match 匹配
// Rust 通过 match 关键字来提供模式匹配，和 C 语言的 switch 用法类似。
// 第一个匹配分支会被比对，并且所有可能的值都必须被覆盖。

fn flow_match() {
    let number = 13;
    // 试一试 ^ 将不同的值赋给 `number`

    println!("Tell me about {}", number);
    match number {
        // 匹配单个值
        1 => println!("One!"),
        // 匹配多个值
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // 试一试 ^ 将 13 添加到质数列表中
        // 匹配一个闭区间范围
        13..=19 => println!("A teen"),
        // 处理其他情况
        _ => println!("Ain't special"),
        // 试一试 ^ 注释掉这个总括性的分支
    }

    let boolean = true;
    // match 也是一个表达式
    let binary = match boolean {
        // match 分支必须覆盖所有可能的值
        false => 0,
        true => 1,
        // 试一试 ^ 将其中一条分支注释掉
    };

    println!("{} -> {}", boolean, binary);
}

#[test]
fn t_flow_match() {
    flow_match();
}

/// 5.1 解构
// match 代码块能以多种方式解构物件。
//
// 解构元组
// 解构枚举
// 解构指针
// 解构结构体

/// 元组
// 元组可以在 match 中解构，如下所示：

fn destructuring_tuples() {
    let triple = (0, -2, 3);
    // 试一试 ^ 将不同的值赋给 `triple`

    println!("Tell me about {:?}", triple);
    // match 可以解构一个元组
    match triple {
        // 解构出第二个和第三个元素
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
        // `..` 可用来忽略元组的其余部分
        _      => println!("It doesn't matter what they are"),
        // `_` 表示不将值绑定到变量
    }
}

/// 枚举
// 和前面相似，解构 enum 的方式如下：
// 需要 `allow` 来消除警告，因为只使用了枚举类型的一种取值。
#[allow(dead_code)]
enum Color {
    // 这三个取值仅由它们的名字（而非类型）来指定。
    Red,
    Blue,
    Green,
    // 这些则把 `u32` 元组赋予不同的名字，以色彩模型命名。
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn destructuring_enum() {
    let color = Color::RGB(122, 17, 40);
    // 试一试 ^ 将不同的值赋给 `color`

    println!("What color is it?");
    // 可以使用 `match` 来解构 `enum`。
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                     c, m, y, k),
        // 不需要其它分支，因为所有的情形都已覆盖
    }
}

/// 指针和引用
// 对指针来说，解构（destructure）和解引用（dereference）要区分开，因为这两者的概念是不同的，
// 和 C 那样的语言用法不一样。
//
// 解引用使用 *
// 解构使用 &、ref、和 ref mut

fn destructuring_ref() {
    // 获得一个 `i32` 类型的引用。`&` 表示取引用。
    let reference = &4;

    match reference {
        // 如果用 `&val` 这个模式去匹配 `reference`，就相当于做这样的比较：
        // `&i32`（译注：即 `reference` 的类型）
        // `&val`（译注：即用于匹配的模式）
        // ^ 我们看到，如果去掉匹配的 `&`，`i32` 应当赋给 `val`。
        // 译注：因此可用 `val` 表示被 `reference` 引用的值 4。
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // 如果不想用 `&`，需要在匹配前解引用。
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // 如果一开始就不用引用，会怎样？ `reference` 是一个 `&` 类型，因为赋值语句
    // 的右边已经是一个引用。但下面这个不是引用，因为右边不是。
    let _not_a_reference = 3;

    // Rust 对这种情况提供了 `ref`。它更改了赋值行为，从而可以对具体值创建引用。
    // 下面这行将得到一个引用。
    let ref _is_a_reference = 3;

    // 相应地，定义两个非引用的变量，通过 `ref` 和 `ref mut` 仍可取得其引用。
    let value = 5;
    let mut mut_value = 6;

    // 使用 `ref` 关键字来创建引用。
    // 译注：下面的 r 是 `&i32` 类型，它像 `i32` 一样可以直接打印，因此用法上
    // 似乎看不出什么区别。但读者可以把 `println!` 中的 `r` 改成 `*r`，仍然能
    // 正常运行。前面例子中的 `println!` 里就不能是 `*val`，因为不能对整数解
    // 引用。
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // 类似地使用 `ref mut`。
    match mut_value {
        ref mut m => {
            // 已经获得了 `mut_value` 的引用，先要解引用，才能改变它的值。
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
}

/// 结构体
// 类似地，解构 struct 如下所示：

fn destructuring_struct() {
    struct Foo { x: (u32, u32), y: u32 }

    // 解构结构体的成员
    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;

    println!("a = {}, b = {},  y = {} ", a, b, y);

    // 可以解构结构体并重命名变量，成员顺序并不重要

    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);

    // 也可以忽略某些变量
    let Foo { y, .. } = foo;
    println!("y = {}", y);

    // 这将得到一个错误：模式中没有提及 `x` 字段
    // let Foo { y } = foo;
}

#[test]
fn destructuring() {
    destructuring_tuples();
    destructuring_enum();
    destructuring_ref();
    destructuring_struct();
}

/// 卫语句
// 可以加上 match 卫语句（guard） 来过滤分支。

#[allow(unused)]
fn match_guard() {
    let pair = (2, -2);
    // 试一试 ^ 将不同的值赋给 `pair`

    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // ^ `if` 条件部分是一个卫语句
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}

/// 绑定
// 在 match 中，若间接地访问一个变量，则不经过重新绑定就无法在分支中再使用它。match 提供了 @ 符号来绑定变量到名称：

// `age` 函数，返回一个 `u32` 值。
#[allow(unused)]
fn age() -> u32 {
    15
}

#[allow(unused)]
fn match_bind() {
    println!("Tell me what type of person you are");

    match age() {
        0             => println!("I haven't celebrated my first birthday yet"),
        // 可以直接匹配（`match`） 1 ..= 12，但那样的话孩子会是几岁？
        // 相反，在 1 ..= 12 分支中绑定匹配值到 `n` 。现在年龄就可以读取了。
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // 不符合上面的范围。返回结果。
        n             => println!("I'm an old person of age {:?}", n),
    }
}

// 你也可以使用绑定来“解构” enum 变体，例如 Option:

#[allow(unused)]
fn some_number() -> Option<u32> {
    Some(42)
}

#[allow(unused)]
fn test_match() {
    match some_number() {
        // 得到 `Some` 可变类型，如果它的值（绑定到 `n` 上）等于 42，则匹配。
        Some(n @ 42) => println!("The Answer: {}!", n),
        // 匹配任意其他数字。
        Some(n)      => println!("Not interesting... {}", n),
        // 匹配任意其他值（`None` 可变类型）。
        _            => (),
    }
}

/// if let
// 在一些场合下，用 match 匹配枚举类型并不优雅。比如：


#[allow(unused)]
fn if_let() {
// 将 `optional` 定为 `Option<i32>` 类型
    let optional = Some(7);

    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
            // ^ 行首需要 2 层缩进。这里从 optional 中解构出 `i`。
            // 译注：正确的缩进是好的，但并不是 “不缩进就不能运行” 这个意思。
        },
        _ => {},
        // ^ 必须有，因为 `match` 需要覆盖全部情况。不觉得这行很多余吗？
    };
}

// if let 在这样的场合要简洁得多，并且允许指明数种失败情形下的选项：
#[allow(unused)]
fn if_let_1() {
    // 全部都是 `Option<i32>` 类型
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // `if let` 结构读作：若 `let` 将 `number` 解构成 `Some(i)`，则执行
    // 语句块（`{}`）
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // 如果要指明失败情形，就使用 else：
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // 解构失败。切换到失败情形。
        println!("Didn't match a number. Let's go with a letter!");
    };

    // 提供另一种失败情况下的条件。
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
        // 解构失败。使用 `else if` 来判断是否满足上面提供的条件。
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // 条件的值为 false。于是以下是默认的分支：
        println!("I don't like letters. Let's go with an emoticon :)!");
    };
}

// 同样，可以用 if let 匹配任何枚举值：
// 以这个 enum 类型为例
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

#[allow(unused)]
fn if_let_2() {
    // 创建变量
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // 变量 a 匹配到了 Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // 变量 b 没有匹配到 Foo::Bar，因此什么也不会打印。
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // 变量 c 匹配到了 Foo::Qux，它带有一个值，就和上面例子中的 Some() 类似。
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }
}

// 另一个好处是：if let 允许匹配枚举非参数化的变量，即枚举未注明 #[derive(PartialEq)]，
// 我们也没有为其实现 PartialEq。在这种情况下，通常 if Foo::Bar==a 会出错，因为此类枚举的实例不具有可比性。
// 但是，if let 是可行的。
//
// 你想挑战一下吗？使用 if let修复以下示例：

// 该枚举故意未注明 `#[derive(PartialEq)]`，
// 并且也没为其实现 `PartialEq`。这就是为什么下面比较 `Foo::Bar==a` 会失败的原因。

#[allow(unused)]
fn if_let_3() {
    let a = Foo::Bar;
    // 修改前：
    // 变量匹配 Foo::Bar
    // if Foo::Bar == a {
    //     // ^-- 这就是编译时发现的错误。使用 `if let` 来替换它。
    //     println!("a is foobar");
    // }
    // 修改后：
    // 变量匹配 Foo::Bar
    if let Foo::Bar = a {
        // ^-- 这就是编译时发现的错误。使用 `if let` 来替换它。
        println!("a is foobar");
    }
}

/// while let
// 和 if let 类似，while let 也可以把别扭的 match 改写得好看一些。考虑下面这段使 i 不断增加的代码：

#[allow(unused)]
fn while_let() {
// 将 `optional` 设为 `Option<i32>` 类型
    let mut optional = Some(0);

// 重复运行这个测试。
    loop {
        match optional {
            // 如果 `optional` 解构成功，就执行下面语句块。
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
                // ^ 需要三层缩进！
            },
            // 当解构失败时退出循环：
            _ => { break; }
            // ^ 为什么必须写这样的语句呢？肯定有更优雅的处理方式！
        }
    }
}

// 使用 while let 可以使这段代码变得更加优雅：

#[allow(unused)]
fn while_let_1() {
    // 将 `optional` 设为 `Option<i32>` 类型
    let mut optional = Some(0);

    // 这读作：当 `let` 将 `optional` 解构成 `Some(i)` 时，就
    // 执行语句块（`{}`）。否则就 `break`。
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ 使用的缩进更少，并且不用显式地处理失败情况。
    }
    // ^ `if let` 有可选的 `else`/`else if` 分句，
    // 而 `while let` 没有。
}

#[test]
fn t_flow() {
    if_let();
    if_let_1();
    if_let_2();
    if_let_3();
    while_let();
    while_let_1();
}

