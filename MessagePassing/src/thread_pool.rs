use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/// 任务类型：把闭包装成符合 Send 的 Job
type Job = Box<dyn FnOnce() + Send + 'static>;

/// 发给 worker 的消息：新任务 or 终止
enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// 创建固定数量的 worker
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel::<Message>();
        // 所有 worker 共享同一个 receiver
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /// 提交一个任务
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job: Job = Box::new(f);
        // 发送任务给某个空闲 worker
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 发送终止消息给所有 worker
        for _ in &self.workers {
            // 用 ok() 防止 drop 时通道已关闭导致 panic
            self.sender.send(Message::Terminate).ok();
        }

        // 等待所有 worker 结束
        for worker in &mut self.workers {
            if let Some(handle) = worker.handle.take() {
                // 忽略 join 错误（例如任务中 panic）
                let _ = handle.join();
            }
        }
    }
}

struct Worker {
    #[allow(dead_code)]
    id: usize,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let handle = thread::spawn(move || loop {
            // 将锁的持有范围尽量缩小到 recv 这一步
            let message = {
                let rx = receiver.lock().unwrap();
                rx.recv()
            };

            match message {
                Ok(Message::NewJob(job)) => {
                    job();
                }
                Ok(Message::Terminate) | Err(_) => {
                    break;
                }
            }
        });

        Worker {
            id,
            handle: Some(handle),
        }
    }
}


/*
为何要 Arc<Mutex<Receiver>>？
mpsc::Receiver 不能被克隆；多个 worker 必须共享同一个接收端，所以用 Arc<Mutex<_>> 包起来，让每个线程在 recv() 前先锁住、取到一条消息、立刻释放锁。

为何 Job = Box<dyn FnOnce()>？
任务是一次性的闭包，必须能在线程间移动，所以 Send + 'static。装箱成 trait object 方便在通道里传输。

优雅关闭
在 Drop 里给每个 worker 发送 Terminate，然后 join() 等待收尾，确保主程序退出前所有后台线程都结束。
*/