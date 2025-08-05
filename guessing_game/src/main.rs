use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    let mut cnt = 0;
    let secret = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("please enter a number between 1 and 100: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("read error");
        let trimmed = guess.trim();
        if trimmed.eq_ignore_ascii_case("exit") {
            println!("you choose to exit game. bye!");
            break;
        }
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a valid number!");
                continue;
            }
        };
        cnt += 1;
        if cnt > 5 {
            println!("you have used 5 times, it's all over.");
            break;
        }
        match guess.cmp(&secret) {
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("bigger"),
            Ordering::Equal => {
                println!("got it, you guessed {} times.", cnt);
                break;
            }
        }
    }
}


/*
.parse() 的返回值是这个类型：Result<u32, std::num::ParseIntError>
它代表：可能成功，也可能失败。
Rust 不允许你“假装 parse 一定成功”，这就是它强调安全性的体现。
*/