use std::fmt;
mod utils;
fn echo <T>(x: T) -> T {
    x
}

struct Point<T> {
    x: T,
    y: T,
}

struct PointI {
    x: i32,
    y: i32,
}

impl fmt::Display for PointI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    println!("Hello, world!");
    let a = echo(5);
    let b = echo("hello");

    println!("{},{}",a, b);

    let p1 = PointI {x: 1, y: 2};
    let p2 = Point {x: 1.1, y: 2.2};
    println!("{}, {}", p1.x, p1.y);
    println!("{}, {}", p2.x, p2.y);
    println!("{}", p1);

    let alice = utils::equal::Person {name: "alice".into(), age: 30};
    let bob = utils::equal::Person {name: "bob".into(), age: 25};

    if alice < bob {
        println!("{} is older than {}", bob, alice);
    } else {
        println!("{} is older than {}", alice, bob);
    }
}


/*
trait:行为约束，类似于Interface
*/

// trait Printable{
//     fn print(&self);
// }

// struct Person {
//     name: String,
// }

// impl Printable for Person {
//     fn print(&self) {
//         println!("{}", self.name);
//     }
// }

// fn show<T: Printable>(item: T) {
//     item.print();
// } 
// 这是典型的泛型+trait约束，T：Printable表示T必须实现Printable trait