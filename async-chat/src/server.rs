use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc, Mutex};

/// 群聊广播：写任务订阅它并写回到客户端
type RoomTx = broadcast::Sender<(SocketAddr, String)>;

/// 在线用户信息（供私聊用）
struct User {
    name: String,
    tx: mpsc::UnboundedSender<String>, // 该用户的私聊写队列
}

/// 共享在线状态（按地址/昵称检索）
#[derive(Default)]
struct State {
    by_addr: HashMap<SocketAddr, User>,
    by_name: HashMap<String, SocketAddr>,
}

type SharedState = Arc<Mutex<State>>;

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "127.0.0.1:7000";
    let listener = TcpListener::bind(addr).await?;
    println!("Chat server listening on {addr}");

    // 群聊广播通道
    let (room_tx, _room_rx) = broadcast::channel::<(SocketAddr, String)>(200);
    let state: SharedState = Arc::new(Mutex::new(State::default()));

    loop {
        let (socket, peer) = listener.accept().await?;
        println!("+ Client connected: {peer}");

        let room_tx = room_tx.clone();
        let mut room_rx = room_tx.subscribe();
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            if let Err(e) = handle_conn(socket, peer, room_tx.clone(), &mut room_rx, state.clone()).await {
                eprintln!("! Connection {peer} error: {e}");
            }

            // 连接结束：清理状态并广播离开
            let left_name = {
                let mut st = state.lock().await;
                if let Some(User { name, .. }) = st.by_addr.remove(&peer) {
                    st.by_name.remove(&name);
                    Some(name)
                } else {
                    None
                }
            };
            if let Some(name) = left_name {
                let _ = room_tx.send((peer, format!("-- {name} left")));
            } else {
                let _ = room_tx.send((peer, format!("-- {peer} left")));
            }
            println!("- Client disconnected: {peer}");
        });
    }
}

async fn handle_conn(
    socket: TcpStream,
    peer: SocketAddr,
    room_tx: RoomTx,
    room_rx: &mut broadcast::Receiver<(SocketAddr, String)>,
    state: SharedState,
) -> io::Result<()> {
    // 拆分读写半端
    let (reader, writer) = socket.into_split();
    let mut lines = BufReader::new(reader).lines();

    // 私聊队列：往这个 sender 发的消息只写给该连接
    let (priv_tx, mut priv_rx) = mpsc::unbounded_channel::<String>();

    // 写任务：同时消费【群聊广播】与【私聊队列】并写回
    let mut rx_for_writer = room_rx.resubscribe();
    let write_task = tokio::spawn(async move {
        let mut w = writer; // 移动所有权，避免 clone 报错
        loop {
            tokio::select! {
                // 收群聊（排除自己）
                Ok((from, msg)) = rx_for_writer.recv() => {
                    if from != peer {
                        if w.write_all(format!("{msg}\n").as_bytes()).await.is_err() { break; }
                    }
                }
                // 收到给自己的私聊
                Some(pm) = priv_rx.recv() => {
                    if w.write_all(format!("{pm}\n").as_bytes()).await.is_err() { break; }
                }
                else => break,
            }
        }
    });

    // 默认显示名用地址
    let mut display_name = format!("{peer}");

    // 处理首条输入：如果是 /nick 则尝试设置，否则注册默认名并把首条当作普通消息
    if let Ok(Some(first)) = lines.next_line().await {
        if let Some(nick) = parse_nick(&first) {
            if let Some(ok_name) = try_set_nick(&state, peer, nick.to_string(), priv_tx.clone()).await {
                display_name = ok_name.clone();
                let _ = room_tx.send((peer, format!("-- {display_name} joined")));
            } else {
                // 昵称被占用：注册默认地址名并提示
                register_default(&state, peer, display_name.clone(), priv_tx.clone()).await;
                let _ = priv_tx.send(format!("** Nick '{nick}' is taken. You are {display_name}"));
                let _ = room_tx.send((peer, format!("-- {display_name} joined")));
            }
        } else {
            // 没有 /nick：注册默认名，并广播这条消息
            register_default(&state, peer, display_name.clone(), priv_tx.clone()).await;
            let _ = room_tx.send((peer, format!("-- {display_name} joined")));
            if !first.trim().is_empty() {
                let _ = room_tx.send((peer, format!("[{display_name}] {first}")));
            }
        }
    } else {
        // 未输入任何内容即断开
        write_task.abort();
        return Ok(());
    }

    // 后续循环：命令(/nick /w) 或 群聊
    while let Some(line) = lines.next_line().await? {
        let line = line.trim();
        if line.is_empty() { continue; }

        // 改昵称
        if let Some(nick) = parse_nick(line) {
            if let Some(new_name) = try_change_nick(&state, peer, nick.to_string()).await {
                let old = std::mem::replace(&mut display_name, new_name.clone());
                let _ = room_tx.send((peer, format!("-- {old} -> {new_name}")));
            } else {
                let _ = priv_tx.send(format!("** Nick '{nick}' is taken"));
            }
            continue;
        }

        // 私聊 /w <name> <msg>
        if let Some((to, msg)) = parse_whisper(line) {
            if let Some(target_tx) = find_user_tx_by_name(&state, to).await {
                let _ = target_tx.send(format!("[whisper from {display_name}] {msg}"));
                let _ = priv_tx.send(format!("[whisper to {to}] {msg}"));
            } else {
                let _ = priv_tx.send(format!("** User '{to}' not found"));
            }
            continue;
        }

        // 普通群聊
        let _ = room_tx.send((peer, format!("[{display_name}] {line}")));
    }

    write_task.abort();
    Ok(())
}

/// 解析 `/nick <name>`
fn parse_nick(s: &str) -> Option<&str> {
    let s = s.trim();
    s.strip_prefix("/nick ")?.trim().split_whitespace().next()
}

/// 解析 `/w <name> <msg>`
fn parse_whisper(s: &str) -> Option<(&str, &str)> {
    let s = s.trim();
    let rest = s.strip_prefix("/w ")?; // whisper
    let mut it = rest.splitn(2, char::is_whitespace);
    let to = it.next()?.trim();
    let msg = it.next().unwrap_or("").trim();
    if to.is_empty() || msg.is_empty() { return None; }
    Some((to, msg))
}

/// 尝试设置昵称（首次注册）。成功返回最终昵称。
async fn try_set_nick(
    state: &SharedState,
    peer: SocketAddr,
    name: String,
    tx: mpsc::UnboundedSender<String>,
) -> Option<String> {
    let mut st = state.lock().await;
    if st.by_name.contains_key(&name) {
        return None;
    }
    st.by_name.insert(name.clone(), peer);
    st.by_addr.insert(peer, User { name: name.clone(), tx });
    Some(name)
}

/// 注册默认昵称（用地址字符串）
async fn register_default(
    state: &SharedState,
    peer: SocketAddr,
    name: String,
    tx: mpsc::UnboundedSender<String>,
) {
    let mut st = state.lock().await;
    st.by_name.insert(name.clone(), peer);
    st.by_addr.insert(peer, User { name, tx });
}

/// 尝试修改昵称。成功返回新昵称。
async fn try_change_nick(state: &SharedState, peer: SocketAddr, new_name: String) -> Option<String> {
    let mut st = state.lock().await;

    if st.by_name.contains_key(&new_name) {
        return None; // 新昵称已被占用
    }

    // 先拿旧名（只读拷贝，避免可变借用冲突）
    let old_name = match st.by_addr.get(&peer) {
        Some(user) => user.name.clone(),
        None => return None,
    };

    // 更新 name -> addr 映射
    st.by_name.remove(&old_name);
    st.by_name.insert(new_name.clone(), peer);

    // 再更新 addr -> user.name
    if let Some(user) = st.by_addr.get_mut(&peer) {
        user.name = new_name.clone();
        Some(new_name)
    } else {
        None
    }
}

/// 按昵称查找其私聊 sender
async fn find_user_tx_by_name(state: &SharedState, name: &str) -> Option<mpsc::UnboundedSender<String>> {
    let st = state.lock().await;
    let &peer = st.by_name.get(name)?;
    let tx = st.by_addr.get(&peer)?.tx.clone();
    Some(tx)
}



/*
cargo run --bin server
cargo run --bin client -- 127.0.0.1:7000
*/