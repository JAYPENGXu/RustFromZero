use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = std::env::args().nth(1).unwrap_or_else(|| "127.0.0.1".to_string());
    let stream = TcpStream::connect(&addr).await?;
    println!("Connected to {}", addr);
    println!("Type messages and press Enter to send. Ctrl+C to quit.");

    let (reader, mut writer) = stream.into_split();

    let stdin_task = tokio::spawn(async move {
        let mut stdin = BufReader::new(tokio::io::stdin()).lines();
        while let Ok(Some(line)) = stdin.next_line().await {
            if writer.write_all(line.as_bytes()).await.is_err() { break; }
            if writer.write_all(b"\n").await.is_err() { break; }
        }
    });

    // 从服务器读 打印
    let mut server_reader = BufReader::new(reader).lines();
    let server_task = tokio::spawn(async move {
        while let Ok(Some(line)) = server_reader.next_line().await{
            println!("{}", line);
        }
    });

    let _ = tokio::join!(stdin_task, server_task);
    Ok(())
}