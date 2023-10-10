use crate::{green, pink, prompt, read_input};
use colored::Colorize;
use std::io::{self};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let addr = stream.peer_addr().unwrap().to_string();
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => {
                // println!("{} 断开了连接", addr);
                break;
            }
            Ok(bytes_read) => bytes_read,
            Err(e) => {
                eprintln!("与 {} 的连接出现错误: {}", addr, e); // Better error logging
                break;
            }
        };

        let msg = &buffer[..bytes_read];
        let message_with_address = format!(
            "{}:--:{}",
            String::from_utf8_lossy(msg),
            addr.split(":").last().unwrap()
        );

        // Lock the clients list once for the entire broadcast
        let mut clients_lock = clients.lock().unwrap();
        // Clean up invalid clients first
        clients_lock.retain(|s| s.peer_addr().is_ok());
        for mut client in clients_lock.iter() {
            if client.peer_addr().unwrap() != stream.peer_addr().unwrap() {
                let _ = client.write(message_with_address.as_bytes());
            }
        }
    }

    // Remove the client after disconnecting
    clients
        .lock()
        .unwrap()
        .retain(|s| s.peer_addr().unwrap() != stream.peer_addr().unwrap());
}

pub fn connection(
    running: Arc<AtomicBool>,
    clients: Arc<Mutex<Vec<TcpStream>>>,
    active_threads: Arc<Mutex<usize>>,
) {
    thread::spawn({
        let clients_clone: Arc<Mutex<Vec<TcpStream>>> = clients.clone();
        move || {
            let bind_result: Result<TcpListener, io::Error> = TcpListener::bind("0.0.0.0:7878");
            let listener: TcpListener = match bind_result {
                Ok(listener) => {
                    println!(
                        "{}\n{}\n{}",
                        green!("服务器已经启动!"),
                        green!("127.0.0.1:7878"),
                        green!("192.168.10.35:7878")
                    );
                    listener
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::AddrInUse {
                        eprintln!("错误: 服务器已经在运行!");
                    } else {
                        eprintln!("错误: 无法启动服务器. 原因: {}", e);
                    }
                    return; // 立即从此线程返回
                }
            };

            // 使用while循环替代for循环来检查running的状态
            while running.load(Ordering::Relaxed) {
                match listener.accept() {
                    Ok((stream, _)) => {
                        let active_threads_clone = active_threads.clone();
                        *active_threads_clone.lock().unwrap() += 1;
                        clients_clone
                            .lock()
                            .unwrap()
                            .push(stream.try_clone().unwrap());
                        let clients_inner_clone = clients_clone.clone();
                        thread::spawn(move || {
                            handle_client(stream, clients_inner_clone);
                            *active_threads_clone.lock().unwrap() -= 1;
                        });
                    }
                    Err(e) => {
                        if e.kind() != std::io::ErrorKind::WouldBlock {
                            eprintln!("连接失败: {}", e);
                        }
                    }
                }
            }

            // 关闭TcpListener以释放端口资源
            drop(listener);
        }
    });

    // 等待一段时间，以确保服务器先于客户端启动
    thread::sleep(std::time::Duration::from_secs(2));
}

// 在stop_server中
pub fn stop_server(
    running: Arc<AtomicBool>,
    clients: Arc<Mutex<Vec<TcpStream>>>,
    active_threads: Arc<Mutex<usize>>,
) {
    println!("正在关闭...");

    running.store(false, Ordering::Relaxed);

    {
        let mut clients_guard = clients.lock().unwrap();
        for client in clients_guard.iter_mut() {
            let _ = client.shutdown(Shutdown::Both); // shutdown the client connection
        }
        clients_guard.clear(); // Clear the clients
    }

    // 循环检查直到所有线程都完成
    while *active_threads.lock().unwrap() > 0 {
        thread::sleep(Duration::from_millis(100));
    }
    // 等待一段时间，以确保服务器先于客户端启动
    thread::sleep(std::time::Duration::from_secs(2));
    println!("服务器已关闭！");
}

pub fn connecting_to_a_server() {
    // 连接到服务器
    let connection = TcpStream::connect("192.168.10.35:7878");
    let mut stream = match connection {
        Ok(stream) => stream,
        Err(e) => {
            return eprintln!("错误: 无法连接到服务器。原因: {}", e);
        }
    };
    let mut read_stream = stream.try_clone().expect("Failed to clone stream");

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    // 一个独立的线程负责读取服务器的消息
    thread::spawn(move || {
        let mut buffer = [0; 1024];
        while running_clone.load(Ordering::Relaxed) {
            match read_stream.read(&mut buffer) {
                Ok(0) => {
                    prompt!(green!("👋 与服务器断开连接..."));
                    break;
                }
                Ok(bytes_read) => {
                    let received_msg = String::from_utf8_lossy(&buffer[..bytes_read]);
                    println!("");
                    prompt!(green!("用户"));
                    prompt!(green!("{}: ", received_msg.split(":--:").last().unwrap()));
                    prompt!(received_msg.split(":--:").next().unwrap());
                }
                Err(e) => {
                    eprintln!("读取错误: {}", e);
                    break;
                }
            }
        }
    });

    loop {
        let mut _msg = String::new();
        prompt!(pink!("输入消息(或'exit'退出):"));
        read_input!(_msg);

        if _msg.trim() == "exit" {
            running.store(false, Ordering::Relaxed);
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            thread::sleep(std::time::Duration::from_secs(2));
            break;
        }

        // 向服务器发送数据
        stream.write(_msg.as_bytes()).expect("无法写入服务器");
    }
}
