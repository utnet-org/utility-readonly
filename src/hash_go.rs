use crate::{green, libc, pink, yellow};
use crate::{libc::HashType, prompt, read_input};
use colored::Colorize;
use std::io::{self, Write};

pub fn hash_operations() {
    loop {
        // 1. 获取字符串
        prompt!(pink!("请输入一个字符串: "));
        read_input!(input); //? 读取用户输入

        // 2. 显示菜单并获取选择
        println!("{}", yellow!("请选择一个哈希算法:"));
        println!("{}", green!("1: SHA-256"));
        println!("{}", green!("2: MD5"));
        println!("{}", green!("3: RIPEMD160"));
        println!("{}", green!("4: WHIRLPOOL"));
        prompt!(pink!("请输入您的选择: "));
        read_input!(choice); //? 读取用户输入

        // 3. 根据用户的选择计算并显示哈希值
        match choice {
            "1" => {
                let hash = libc::hash_string(input, HashType::HashSha256);
                println!("{}{:?}", green!("SHA-256 哈希值:"), hash);
                let hash_hex = libc::get_hash_hex(input, HashType::HashSha256);
                println!("{}{}", green!("SHA-256 哈希值 (十六进制): "), hash_hex);
            }
            "2" => {
                let hash = libc::hash_string(input, HashType::HashMd5);
                println!("{} {:?}", green!("MD5 哈希值:"), hash);
                let hash_hex = libc::get_hash_hex(input, HashType::HashMd5);
                println!("{} {}", green!("MD5 哈希值 (十六进制): "), hash_hex);
            }
            "3" => {
                let hash = libc::hash_string(input, HashType::HashRipemd160);
                println!("{} {:?}", green!("RIPEMD160 哈希值:"), hash);
                let hash_hex = libc::get_hash_hex(input, HashType::HashRipemd160);
                println!("{} {}", green!("RIPEMD160 哈希值 (十六进制): "), hash_hex);
            }
            "4" => {
                let hash = libc::hash_string(input, HashType::HashWhirlpool);
                println!("{} {:?}", green!("WHIRLPOOL 哈希值:"), hash);
                let hash_hex = libc::get_hash_hex(input, HashType::HashWhirlpool);
                println!("{} {}", green!("WHIRLPOOL 哈希值 (十六进制): "), hash_hex);
            }
            _ => {
                println!("{}", pink!("无效的选择"));
                continue;
            }
        }
        // 提供退出提示
        println!("{}", pink!("输入 'exit' 退出，或者按任意键继续"));
        read_input!(end_choice);
        if end_choice.trim().eq_ignore_ascii_case("exit") {
            break;
        }
    }
}
