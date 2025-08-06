fn main() {
    let s1 = String::from("hello");  // 创建一个堆分配的字符串，s1是它的所有者
    // let s2 = s1;        //不是复制，而是所有权转移
    println!("{}", s1); // error:Rust的核心规则之一，每个值在任意时刻只能有一个所有者

    let len = calculate_length(&s1);
    println!("s1'length is {}: ", len);

    let mut s = String::from("world");  // mut 实现对s的可变借用
    // let r1 = &s; // 不可变引用
    // lete r2 = &mut s;  // 可变引用（同时存在）
    // println!("{}, {}", r1, r2);  // 在同一作用域内，不能同时存在一个可变引用和一个不可变引用


    let r3 = &s;
    println!("只读引用r1:{}", r3); //r3的生命周期在这里结束
    let r4 = &mut s;
    println!("可变引用 r4: {}", r4); // 现在才创建可变引用

    change(&mut s);  // 传递的是可变借用
    println!("after change : {}", s);

    let t1 = String::from("abc");
    let t2 = String::from("cdefg");
    let res = longest(&t1, &t2);
    println!("the largest is: {}", res);


    let b = Book {  // s1和s的生命周期覆盖在整个main函数，所以Book中的两个引用是安全的
                    // 编译器知道b.title和b.content在整个结构体生命周期内都有效
        title: &s1,
        content: &s,
    };
    println!("{} : {}", b.title, b.content);

    let title = String::from("Rust ownership");
    let post = Post {
        id: 1,
        title: &title,
    };
    post.print();
    println!("title' length: {}", post.title_len());
}


fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("!!");
}


// fn longest(s1: &str, s2: &str) -> &str {  // 报错，它不知道返回的这个&str到底是s1还是s2的引用
//     if s1.len() > s2.len() {
//         s1
//     } else {
//         s2
//     }
// }

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str { // 正确版本，加上生命周期
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
    // let temp = String::from("temp");
    // &temp  // 错误，返回了一个局部变量的引用
}
// <'a>：声明一个声明周期'a
// s1: &'a str和s2: &'a str：s1和s2必须活的至少有'a那么久
// 返回值：也就是&'a str表示返回值的生命周期===参数的生命周期


// 为什么写<'a>，因为结构体中包含了引用，Rust必须知道这些引用的
// 生命周期要至少活得跟结构体实例一样久
struct Book<'a> {
    title: &'a str,
    content: &'a str,
}


struct User {
    name: String,
}

impl User {
    fn greet(&self) {
        println!("hello, {}!", self.name);
    }
}


struct Post<'a> {
    id: u32,
    title: &'a str,
}

impl<'a> Post<'a> {  // 因为Post<'a>中有引用字段，所以impl也必须显式声明并携带整个生命周期
    fn title_len(&self) -> usize {
        self.title.len()
    }

    fn print(&self) {
        println!("#{} - {}", self.id, self.title);
    }
}


/*
String是堆上的可变数据，
所有实现了Copy trait的类型在赋值时是值复制，不是所有权移动
Rust中常见的Copy类型有：
* 所有的整数类型：i32 u32 i64
* 所有浮点数： f32 f64
* bool
* char
* 元组（只要所有元素都实现了Copy）

--------------------------------
String/Vec 赋值后的行为：所有权移动
堆上数据Move，栈上数据Copy
能放进寄存器的就Copy，带资源的就Move

---------------------------------
要么多个只读借用，要么唯一一个可变借用
*/