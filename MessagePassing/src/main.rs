use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
mod thread_pool;
use thread_pool::ThreadPool;
// Rust 提倡 消息传递并发，而不是数据共享
// std::sync::mpsc（multi-producer, single-consumer）通道

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..=5 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }



       // 共享结果容器
    let results = Arc::new(Mutex::new(Vec::new()));

    // 创建一个 4 线程的线程池
    let pool = ThreadPool::new(4);

    for i in 0..8 {
        let results = Arc::clone(&results);
        pool.execute(move || {
            // 模拟耗时计算
            let ans = i * i;
            std::thread::sleep(Duration::from_millis(200));
            // 收集结果
            results.lock().unwrap().push((i, ans));
            println!("task #{i} -> {ans}");
        });
    }

    // 离开作用域时 pool 被 drop：等待 worker 们全部退出
    drop(pool);

    // 打印结果（排序仅为美观）
    let mut out = results.lock().unwrap().clone();
    out.sort_by_key(|&(i, _)| i);
    println!("results = {:?}", out);
}
