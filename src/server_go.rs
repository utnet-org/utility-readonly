use colored::Colorize;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use reqwest;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::io::{self, Write};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::{green, pink, prompt, read_input, red};

#[derive(Debug, Serialize, Deserialize)]
struct MyStruct {
    message: String,
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let input_param = req.uri().query().unwrap_or("");
    let response = Response::new(Body::from(
        serde_json::to_string(&MyStruct {
            message: input_param.to_string(),
        })
        .unwrap(),
    ));
    Ok(response)
}

pub fn start_server(should_stop: Arc<AtomicBool>) {
    // 只创建一次Tokio运行时
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let make_svc =
            make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) }); //? 创建服务

        let addr = ([127, 0, 0, 1], 8080).into(); //? 创建监听地址

        println!("🚀 {} {}", green!("服务器已启动，正在监听:"), addr); //? 打印服务器启动信息

        // 使用Tokio的timer每隔一段时间检查should_stop标志
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(2));
        loop {
            let server = Server::bind(&addr).serve(make_svc);
            tokio::select! {
                _ = server => {
                    eprintln!("🔥 {}", red!("服务器错误"));
                },
                _ = interval.tick() => {
                    if should_stop.load(Ordering::Relaxed) {
                        println!("🚫 {}", green!("服务器接收到停止信号..."));
                        break;
                    }
                }
            }
        }

        println!("👋 {}", green!("服务器正在关闭..."));
    });
}

pub fn start_client() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let mut _input = String::new();
        prompt!(pink!("输入数据(或'exit'退出):"));
        read_input!(_input); //? 读取用户输入

        if _input == "exit" {
            println!("👋 {}", green!("拜拜!"));
            break;
        }

        let url = format!("http://127.0.0.1:8080?{}", _input); //? 构造URL
        let response = reqwest::blocking::get(&url)?; //? 发送请求
        let body = response.text()?; //? 获取响应体
        let my_struct: Result<MyStruct, _> = serde_json::from_str(&body); //? 解析JSON
        match my_struct {
            //? 处理响应
            Ok(structure) => {
                println!("✅ {} {}", green!("服务器响应:"), structure.message); //? 显示响应
            }
            Err(e) => {
                eprintln!("❌ {} {}", red!("JSON解析错误:"), e); //? 处理错误
                return Err(Box::new(e)); //? 处理错误
            }
        }
    }

    Ok(()) //? 处理错误
}

pub fn server_operations() {
    let should_stop = Arc::new(AtomicBool::new(false)); //?

    let server_thread = std::thread::spawn({
        let should_stop = should_stop.clone();
        move || {
            start_server(should_stop);
        }
    });

    // 等待一段时间，以确保服务器先于客户端启动
    std::thread::sleep(std::time::Duration::from_secs(2));

    // 启动客户端
    let _ = start_client();

    // 通知服务器停止
    should_stop.store(true, Ordering::Relaxed);

    // 等待服务器线程结束
    server_thread.join().expect("Server thread panicked");
}
