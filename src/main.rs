mod libc;

fn main() {
    let data = "Utility Super Cool!";
    let hash = libc::hash_string(data); // 使用 lib 模块中的 hash_string 函数
    println!("Hash of '{}': {:?}", data, hash);
    let hash_hex = libc::get_hash_hex(data); // 使用 lib 模块中的 get_hash_hex 函数
    println!("Hash of '{}': {}", data, hash_hex);
}
