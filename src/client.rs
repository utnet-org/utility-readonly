use std::io;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::connect_async;
use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;

pub async fn run_client() -> Result<(), io::Error> {
    let server_addr = "ws://127.0.0.1:8080";

    // 连接到 WebSocket 服务器
    let (ws_stream, _) = connect_async(server_addr)
        .await
        .expect("无法连接到服务器");

    // 将 WebSocket 流拆分为发送者和接收者
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // 创建一个异步任务来处理终端输入并发送到服务器
    let input_task = tokio::spawn(async move {
        loop {
            let mut user_input = String::new();
            if io::stdin().read_line(&mut user_input).is_err() {
                eprintln!("无法读取用户输入");
                break;
            }

            // 发送用户输入的消息到服务器
            if ws_sender
                .send(Message::Text(user_input.trim().to_string()))
                .await
                .is_err()
            {
                eprintln!("向服务器发送消息失败");
                break;
            }
        }
    });

    // 从服务器接收消息
    while let Some(Ok(msg)) = ws_receiver.next().await {
        match msg {
            Message::Text(text) => {
                println!("从服务器接收到的消息: {}", text);
            }
            _ => {}
        }
    }

    // 等待输入任务完成
    if let Err(err) = input_task.await {
        eprintln!("输入任务失败: {:?}", err);
        return Err(io::Error::new(io::ErrorKind::Other, "输入任务失败"));
    }

    Ok(())
}
