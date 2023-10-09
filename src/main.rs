use libc::HashType;
use std::io::{self, Write};

mod libc;

/// 读取用户输入的字符串 (不包括换行符) 并返回 String 对象 (堆上分配) 或 &str 对象 (栈上分配)
macro_rules! read_input {
    ($var:ident) => {
        let mut $var = String::new();
        io::stdin().read_line(&mut $var).expect("读取失败");
        let $var = $var.trim();
    };
}

/// 显示提示信息并刷新缓冲区
macro_rules! prompt {
    ($msg:expr) => {
        print!($msg);
        io::stdout().flush().unwrap();
    };
}

fn main() {
    //? 无限循环，直到用户输入 'exit' 为止
    loop {
        // 1. 获取字符串
        prompt!("请输入一个字符串: ");
        read_input!(input); //? 读取用户输入

        // 2. 显示菜单并获取选择
        println!("请选择一个哈希算法:");
        println!("1: SHA-256");
        println!("2: MD5");
        println!("3: RIPEMD160");
        println!("4: WHIRLPOOL");
        prompt!("请输入您的选择: ");
        read_input!(choice); //? 读取用户输入

        // 3. 根据用户的选择计算并显示哈希值
        match choice {
            "1" => {
                let hash = libc::hash_string(input, HashType::HashSha256);
                println!("SHA-256 哈希值: {:?}", hash);
                let hash_hex = libc::get_hash_hex(input, HashType::HashSha256);
                println!("SHA-256 哈希值 (十六进制): {}", hash_hex);
            }
            "2" => {
                let hash = libc::hash_string(input, HashType::HashMd5);
                println!("MD5 哈希值: {:?}", hash);
                let hash_hex = libc::get_hash_hex(input, HashType::HashMd5);
                println!("MD5 哈希值 (十六进制): {}", hash_hex);
            }
            "3" => {
                let hash = libc::hash_string(input, HashType::HashRipemd160);
                println!("RIPEMD160 哈希值: {:?}", hash);
                let hash_hex = libc::get_hash_hex(input, HashType::HashRipemd160);
                println!("RIPEMD160 哈希值 (十六进制): {}", hash_hex);
            }
            "4" => {
                let hash = libc::hash_string(input, HashType::HashWhirlpool);
                println!("WHIRLPOOL 哈希值: {:?}", hash);
                let hash_hex = libc::get_hash_hex(input, HashType::HashWhirlpool);
                println!("WHIRLPOOL 哈希值 (十六进制): {}", hash_hex);
            }
            _ => {
                println!("无效的选择");
            }
        }

        // 提供退出提示
        println!("如果想结束进程，请输入 'exit'，否则按任意键继续...");
        read_input!(end_choice);
        if end_choice.trim().eq_ignore_ascii_case("exit") {
            break;
        }
    }
}
