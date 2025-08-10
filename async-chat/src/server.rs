use std::net::SocketAddr;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "127.0.0.1:7000";
    let listener = TcpListener::bind(addr).await?;
    println!("Chat server listening on {}", addr);

    // 房间广播：(发送者地址, 文本)
    let (tx, _rx) = broadcast::channel::<(SocketAddr, String)>(100);

    loop {
        let (socket, peer) = listener.accept().await?;
        println!("+ Client connected: {}", peer);

        // 为本连接准备各自的 sender / receiver
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        // 进入连接专属任务
        tokio::spawn(async move {
            // 进房提示
            let _ = tx.send((peer, format!("-- {} joined", peer)));

            // 给 handle_conn 单独克隆一份 tx，外层仍保留 tx 用于离线提示
            let tx_for_conn = tx.clone();

            if let Err(e) = handle_conn(socket, peer, tx_for_conn, &mut rx).await {
                eprintln!("! Connection {} error: {}", peer, e);
            }

            // 离开提示（此处使用的是外层保留的 tx）
            let _ = tx.send((peer, format!("-- {} left", peer)));
            println!("- Client disconnected: {}", peer);
        });
    }
}

async fn handle_conn(
    socket: TcpStream,
    peer: SocketAddr,
    tx: broadcast::Sender<(SocketAddr, String)>,
    rx: &mut broadcast::Receiver<(SocketAddr, String)>,
) -> io::Result<()> {
    // into_split：拿到拥有所有权的读/写半端
    let (reader, writer) = socket.into_split();
    let mut lines = BufReader::new(reader).lines();

    // 写任务：把"广播"写回给该客户端（排除自己）
    // 注意：这里把 writer **移动**进任务里，不再 clone
    let mut rx_for_writer = rx.resubscribe();
    let write_task = tokio::spawn(async move {
        let mut w = writer; // 移动所有权
        while let Ok((from, msg)) = rx_for_writer.recv().await {
            if from != peer {
                if w.write_all(format!("[{}] {}\n", from, msg).as_bytes())
                    .await
                    .is_err()
                {
                    break; // 客户端断开
                }
            }
        }
    });

    // 读循环：把该客户端发来的每一行广播出去
    while let Some(line) = lines.next_line().await? {
        if !line.trim().is_empty() {
            let _ = tx.send((peer, line));
        }
    }

    // 读完/断开：停止写任务
    write_task.abort();
    Ok(())
}


/*
cargo run --bin server
cargo run --bin client -- 127.0.0.1:7000
*/