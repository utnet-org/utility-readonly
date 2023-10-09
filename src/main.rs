use colored::Colorize;
use std::io::{self, Write};
mod hash_go;
mod libc;
mod server_go;
mod tcp;
#[macro_use]
mod macros;

fn main() {
    //? 无限循环，直到用户输入 'exit' 为止
    loop {
        println!("{}", yellow!("请选择一个操作："));
        println!("{}", green!("1: 字符串哈希"));
        println!("{}", green!("2: 启动HTTP服务器"));
        println!("{}", green!("3: 启动TCP服务"));
        println!("{}", green!("4: 连接TCP服务"));
        println!("{}", red!("10: 退出"));
        prompt!(pink!("请输入您的选择: "));
        read_input!(choice);
        match choice.trim() {
            "1" => hash_go::hash_operations(),
            "2" => server_go::server_operations(),
            "3" => tcp::connection(),
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
