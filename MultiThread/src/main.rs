use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};
/*
thread::spawn 创建新线程
join() 等待线程结束
闭包中变量如果要被线程使用，必须是 'static 生命周期或 move 捕获
*/


fn main() {
    println!("Hello, world!");

    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("子线程：{}", i);
            thread::sleep(Duration::from_millis(200));
        }
    });

    for i in 1..3 {
        println!("主线程： {}", i);
        thread::sleep(Duration::from_millis(200));
    }

    handle.join().unwrap();





    let (tx, rx) = mpsc::channel();  // 创建发送端和接收端

    thread::spawn(move || {
        let vals = vec!["hi", "from", "thread"];
        for val in vals {
            tx.send(val).unwrap(); //发送消息
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx{  // rx在for循环中会自动阻塞直到收到数据
        println!("got: {}", received);
    }



    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move||{
            let mut num = counter.lock().unwrap();
            * num += 1;
        }));
    }
    for h in handles {
        h.join().unwrap();
    }

    println!("最终计数： {}", *counter.lock().unwrap());
}

/*
| 场景          | 推荐工具                            |
| ----------- | ------------------------------- |
| 多线程共享只读数据   | `Arc<T>`                        |
| 多线程共享可变数据   | `Arc<Mutex<T>>`                 |
| 读多写少        | `Arc<RwLock<T>>`                |
| 线程间传递消息     | `std::sync::mpsc` 或 `crossbeam` |
| 单线程多所有者可变数据 | `Rc<RefCell<T>>`                |

*/