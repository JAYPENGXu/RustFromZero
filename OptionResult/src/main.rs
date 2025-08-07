use std::fs::File;
use std::io:: {self, Read};

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process(msg: Message) {
    match msg {
        Message::Quit => println!("Quit!"),
        Message:: Move{x, y} => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: RGB({},{},{})", r, g, b),
    }
}

fn main() {
    let m1 = Message::Quit;
    let m2 = Message::Move{x: 1, y: 2};
    let m3 = Message::Write(String::from("hello world!"));
    let m4 = Message::ChangeColor(0, 255, 0);

    process(m1);
    process(m2);
    process(m3);
    process(m4);


    let res = find_even(6);
    match res {
        Some(n) => println!("found even: {}", n),
        None => println!("Not even"),
    }


    match read_file("hello.txt") {
        Ok(data) => println!("File content: {}", data),
        Err(e) => println!("error reading file: {}", e),
    }
}


/*
Option和Result：安全处理空值和错误，采用显式的方式来处理可能失败和可能不存在的值。
Option<T>: 处理可能为空的值
Result<T, E> : 处理可能出错的操作
*/

//---------------

// enum Option<T> {
//     Some(T),  // 可以理解为 Some(value):有值
//     None,     // 无值
// }

fn find_even(num: i32) -> Option<i32> {
    if num % 2 == 0 {
        Some(num)
    } else {
        None
    }
}

// enum Result<T, E> {
//     Ok(T),  // 成功时的值
//     Err(E),  //失败时的错误
// }

// 快捷方式: ?运算符，自动把错误传播出去

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;  // ?会自动返回错误
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}