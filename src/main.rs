use colored::Colorize;
use std::io::{self, Write};
use std::net::TcpStream;
use std::sync::atomic::Ordering;
use std::sync::{atomic::AtomicBool, Arc, Mutex};
mod hash_go;
mod libc;
mod server_go;
mod tcp;
#[macro_use]
mod macros;

fn main() {
    // 在你的主代码或初始化代码中
    let active_threads = Arc::new(Mutex::new(0));
    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new())); // 用于存储客户端的列表
    let running = Arc::new(AtomicBool::new(true)); // 服务器是否正在运行
    let server_started = Arc::new(AtomicBool::new(false)); // 是否已启动TCP服务器

    loop {
        println!("{}", yellow!("请选择一个操作："));
        println!("{}", green!("1: 字符串哈希"));
        println!("{}", green!("2: 启动HTTP服务器"));
        if !server_started.load(Ordering::Relaxed) {
            println!("{}", green!("3: 启动TCP服务"));
        } else {
            println!("{}", red!("3: 关闭TCP服务"));
        }
        println!("{}", green!("4: 连接TCP服务"));
        println!("{}", red!("10: 退出"));
        prompt!(pink!("请输入您的选择: "));
        read_input!(choice);
        match choice.trim() {
            "1" => hash_go::hash_operations(),
            "2" => server_go::server_operations(),
            "3" => {
                if !server_started.load(Ordering::Relaxed) {
                    tcp::connection(running.clone(), clients.clone(), active_threads.clone());
                    server_started.store(true, Ordering::Relaxed);
                } else {
                    tcp::stop_server(running.clone(), clients.clone(), active_threads.clone());
                    server_started.store(false, Ordering::Relaxed);
                }
            }
            "4" => tcp::connecting_to_a_server(),
            "10" => {
                println!("谢谢使用，再见！👋👋👋");
                break;
            }
            _ => println!("无效的选择"),
        }
        // 重新打印菜单
        println!();
    }
}

// use druid::{
//     widget::{Button, CrossAxisAlignment, Flex, Label, MainAxisAlignment},
//     AppLauncher, PlatformError, Widget, WindowDesc,
// };
// mod hash_go;
// mod libc;
// mod server_go;
// mod tcp;
// #[macro_use]
// mod macros;

// fn build_ui() -> impl Widget<()> {
//     let label = Label::new("请选择一个操作：").with_text_size(16.0);
//     let button_1 = Button::new("字符串哈希").on_click(|_ctx, _data, _env| {
//         hash_go::hash_operations();
//     });
//     let button_2 = Button::new("启动HTTP服务器").on_click(|_ctx, _data, _env| {
//         server_go::server_operations();
//     });
//     let button_3 = Button::new("启动TCP服务").on_click(|_ctx, _data, _env| {
//         tcp::connection();
//     });
//     let button_4 = Button::new("连接TCP服务").on_click(|_ctx, _data, _env| {
//         tcp::connecting_to_a_server();
//     });
//     let button_exit = Button::new("退出").on_click(|ctx, _data, _env| {
//         ctx.window().close();
//     });

//     Flex::column()
//         .cross_axis_alignment(CrossAxisAlignment::Center)
//         .main_axis_alignment(MainAxisAlignment::Center)
//         .with_child(label)
//         .with_spacer(8.0)
//         .with_child(button_1)
//         .with_spacer(8.0)
//         .with_child(button_2)
//         .with_spacer(8.0)
//         .with_child(button_3)
//         .with_spacer(8.0)
//         .with_child(button_4)
//         .with_spacer(8.0)
//         .with_child(button_exit)
// }

// fn main() -> Result<(), PlatformError> {
//     let main_window = WindowDesc::new(build_ui()).title("我的应用程序");
//     AppLauncher::with_window(main_window)
//         .log_to_console() // 使用新的方法替代 use_simple_logger
//         .launch(()) // 这个方法启动应用程序
// }
