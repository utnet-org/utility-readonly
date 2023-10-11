use std::io;
use futures_util::SinkExt;
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::accept_async;
use futures_util::stream::StreamExt;

pub async fn run_server() -> Result<(), io::Error> {
    let server_addr = "127.0.0.1:8080";

    // 创建 WebSocket 服务器监听器
    let listener = TcpListener::bind(server_addr).await?;
    println!("WebSocket server listening on ws://{}", server_addr);

    while let Ok((stream, _)) = listener.accept().await {
        // 处理每个新的 WebSocket 连接
        tokio::spawn(handle_connection(stream));
    }

    Ok(())
}

async fn handle_connection(stream: tokio::net::TcpStream) -> Result<(), io::Error> {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during WebSocket handshake");

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    while let Some(Ok(msg)) = ws_receiver.next().await {
        match msg {
            Message::Text(text) => {
                println!("Received message: {}", text);
                // 回复客户端
                let mut ser_input = String::new();
                io::stdin().read_line(&mut ser_input).expect("无法读取服务器输入");
                ws_sender.send(Message::Text(ser_input.parse().unwrap())).await.expect("????");
            }
            _ => {}
        }
    }
    Ok(())
}
