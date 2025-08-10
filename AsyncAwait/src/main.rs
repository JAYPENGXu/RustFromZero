use tokio::time::{sleep, Duration};
use tokio::sync::mpsc;

/*
| API                  | 用途             |
| -------------------- | -------------- |
| `tokio::spawn`       | 启动一个新的异步任务（协程） |
| `tokio::join!`       | 并发等待多个 Future  |
| `tokio::time::sleep` | 异步延迟（不会阻塞线程）   |
| `tokio::sync::mpsc`  | 异步多生产者单消费者通道   |
| `tokio::sync::Mutex` | 异步锁（避免阻塞线程）    |

*/

async fn task_a() {
    println!("Task A start");
    sleep(Duration::from_secs(1)).await;
    println!("Task A done");
}

async fn task_b() {
    println!("Task B start");
    sleep(Duration::from_secs(2)).await;
    println!("Task B done");
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    tokio::join!(task_a(), task_b());
    println!("all done.");


    let (tx, mut rx) = mpsc::channel(32);

    // 生产者
    tokio::spawn(async move {
        for i in 1..=5{
            tx.send(i).await.unwrap();
            println!("Sent : {}", i);
            sleep(Duration::from_millis(200)).await;
        }
    });

    // 消费者
    while let Some(val) = rx.recv().await{
        println!("Got {}", val);
    }

}


async fn hello() { // async fn返回的是一个Future,类似JS中的Promise, 但Future是惰性的，必须用执行器运行
    println!("echo hello");
}
