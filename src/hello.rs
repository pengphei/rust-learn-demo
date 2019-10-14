fn foo(x: i32) -> i32 {

    // rust 为基于表达式的语言

    let mut ifx = 10;
    
    if x==100 {
        ifx = 10;
    } 
    else {
        ifx = 20
    };


    println!("ifx is {}", ifx);

    // 代码块
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    ifx
}

fn main() {
    // 变量不可边,let表明一个语句
    let x = "hello world!";

    // 变量可变
    let mut y = 100;
    
    foo(y);

    y = 200;

    println!("Hello, world!");
    println!("{}", x);

    foo(y);

    println!("hello {}", y);

    let fb = true;
    println!("bool is {}", fb);

    // 元组类型
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tup is {} {} {}", x, y, z);
    println!("tupx is {} {} {}", tup.0, tup.1, tup.2);

    // 数组处理, for
    let a = [1, 2, 3, 4, 5];
    for ai in a.iter() {
        println!("{}", ai);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    // while 处理
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    // loop 循环
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}