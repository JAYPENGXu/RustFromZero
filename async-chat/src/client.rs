use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = std::env::args().nth(1).unwrap_or_else(|| "127.0.0.1:7000".to_string());
    let stream = TcpStream::connect(&addr).await?;
    println!("Connected to {addr}");
    println!("Commands:");
    println!("  /nick <name>   set or change nickname");
    println!("  /w <name> <msg>  whisper");
    println!("Type your nickname first:");

    let (reader, mut writer) = stream.into_split();

    // 启动时先输入昵称并发送一次
    {
        let mut input = String::new();
        let mut stdin = BufReader::new(tokio::io::stdin()).lines();
        // 读一行昵称
        input = stdin.next_line().await?.unwrap_or_default();
        if input.trim().is_empty() {
            println!("(empty -> use default addr as name)");
        } else {
            let nick_cmd = format!("/nick {input}");
            writer.write_all(nick_cmd.as_bytes()).await?;
            writer.write_all(b"\n").await?;
        }

        // 把 stdin task 放到后台继续输入
        tokio::spawn(async move {
            let mut stdin = BufReader::new(tokio::io::stdin()).lines();
            while let Ok(Some(line)) = stdin.next_line().await {
                if writer.write_all(line.as_bytes()).await.is_err() { break; }
                if writer.write_all(b"\n").await.is_err() { break; }
            }
        });
    }

    // 读服务器并打印
    let mut server_reader = BufReader::new(reader).lines();
    while let Ok(Some(line)) = server_reader.next_line().await {
        println!("{}", line);
    }
    Ok(())
}
