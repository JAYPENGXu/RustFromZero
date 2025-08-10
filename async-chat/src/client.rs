use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:7000".to_string());
    let stream = TcpStream::connect(&addr).await?;
    println!("Connected to {addr}");
    println!("Commands:");
    println!("  /nick <name>      set or change nickname");
    println!("  /w <name> <msg>   whisper");
    println!("Type your nickname first (or just Enter to use address):");

    let (reader, writer) = stream.into_split();

    // 出站写通道：统一把需要发送的文本（不含换行）发到写泵
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    // 写泵任务：独占 writer，把收到的每条消息加上 \n 后写出
    let write_task = tokio::spawn(async move {
        use tokio::io::AsyncWriteExt;
        let mut w = writer;
        while let Some(line) = rx.recv().await {
            if w.write_all(line.as_bytes()).await.is_err() { break; }
            if w.write_all(b"\n").await.is_err() { break; }
        }
    });

    // 启动时读取一行昵称并发送 /nick（如果非空）
    {
        let mut stdin = BufReader::new(tokio::io::stdin()).lines();
        if let Ok(Some(input)) = stdin.next_line().await {
            if !input.trim().is_empty() {
                let _ = tx.send(format!("/nick {}", input.trim()));
            } else {
                println!("(empty -> use default addr as name)");
            }

            // 后续持续读取 stdin，转发到服务器
            let tx_stdin = tx.clone();
            tokio::spawn(async move {
                let mut stdin = BufReader::new(tokio::io::stdin()).lines();
                while let Ok(Some(line)) = stdin.next_line().await {
                    if tx_stdin.send(line).is_err() { break; }
                }
            });
        } else {
            // 没读到昵称输入，也继续进入正常循环
            let tx_stdin = tx.clone();
            tokio::spawn(async move {
                let mut stdin = BufReader::new(tokio::io::stdin()).lines();
                while let Ok(Some(line)) = stdin.next_line().await {
                    if tx_stdin.send(line).is_err() { break; }
                }
            });
        }
    }

    // 读服务器：遇到 PING 立即通过通道回 PONG，其余打印
    let mut server_reader = BufReader::new(reader).lines();
    while let Ok(Some(line)) = server_reader.next_line().await {
        if line == "PING" {
            // println!("[heartbeat] <- PING");
            let _ = tx.send("PONG".to_string());
            // println!("[heartbeat] -> PONG");
            continue;
        }
        println!("{}", line);
    }

    // 服务器读循环结束：关闭写泵
    drop(tx);
    let _ = write_task.await;
    Ok(())
}
