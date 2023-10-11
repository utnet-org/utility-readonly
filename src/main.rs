use std::io;
use futures_util::SinkExt;
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::Error;
use tokio_tungstenite::accept_async;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let server_addr = "127.0.0.1:8080";

    // 创建 WebSocket 服务器监听器
    let listener = TcpListener::bind(server_addr).await.expect("Failed to bind");

    println!("WebSocket server listening on ws://{}", server_addr);

    while let Ok((stream, _)) = listener.accept().await {
        // 处理每个新的 WebSocket 连接
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream) -> Result<(), Error> {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during WebSocket handshake");

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    while let Some(Ok(msg)) = ws_receiver.next().await {
        match msg {
            Message::Text(text) => {
                println!("从客服端接收到的信息: {}", text);
                let mut one = String::new();
                io::stdin().read_line(&mut one).expect("无法读取用户输入");
                // 回复客户端
                ws_sender.send(Message::Text(one.parse().unwrap())).await?;
            }
            _ => {}
        }
    }

    Ok(())
}

// mod client;
// mod server;
//
// use client::run_client;
// use server::run_server;
//
// #[tokio::main]
// async fn main() {
//     // 启动客户端和服务器
//     // let client_task = tokio::spawn(run_client());
//     let server_task = tokio::spawn(run_server());
//
//     // 等待两个任务完成
//     // client_task.await.expect("客户端任务失败").expect("TODO: panic message");
//     server_task.await.expect("服务器任务失败").expect("TODO: panic message");
// }
