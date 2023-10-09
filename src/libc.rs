extern crate libc; //? 引入 libc 库
use libc::c_char; //? 引入 libc 库中的 c_char 类型

#[repr(C)]
#[derive(Debug)]
pub enum HashType {
    HashSha256,
    HashMd5,
    HashRipemd160,
    HashWhirlpool,
    // ... 如果您添加了其他的哈希类型
}

extern "C" {
    fn hash_to_hex_string(hash: *const u8) -> *mut libc::c_char;
    //? 使用 extern "C" 声明 C 函数 hash_function ，该函数在 hash.c 中定义
    fn hash_function(input: *const c_char, output: *mut u8, hash_type: HashType);
    //? hash_function 函数接受两个参数，一个是 C 字符串的指针，另一个是 u8 类型的指针
}

/// 使用' hash.c '中定义的C函数散列字符串。
/// 返回一个32字节的数组，其中包含散列值。
/// 请注意，此函数不是线程安全的，因为它使用静态缓冲区。
pub fn hash_string(input: &str, hash_type: HashType) -> [u8; 32] {
    //? 将 Rust 字符串转换为 C 字符串 (以零结尾的字节数组)
    let mut output = [0u8; 32];
    println!("{:?}", hash_type);

    unsafe {
        //? 调用 FFI 函数: 从其他语言（如 C）导入的函数可能不满足 Rust 的安全性要求，因此调用它们需要被标记为不安全 unsafe
        hash_function(
            input.as_ptr() as *const c_char,
            output.as_mut_ptr(),
            hash_type,
        );
        //? input.as_ptr() 是 Rust 字符串的指针，output.as_mut_ptr() 是 output 数组的指针
        //? as_ptr() 和 as_mut_ptr() 方法返回指针，但是不会转移所有权
        //? as_ptr() 方法返回一个不可变的指针，而 as_mut_ptr() 方法返回一个可变的指针
        //? as_ptr() 和 as_mut_ptr() 方法只能用于指向连续数据的类型，如数组、Vec 和 String
        //? as_ptr() 和 as_mut_ptr() 方法返回的指针是有效的，只要 Rust 对象是有效的
        //? input.as_ptr() as *const c_char 将 Rust 字符串的指针转换为 C 字符串的指针
        //? output.as_mut_ptr() 将 output 数组的指针转换为 u8 类型的指针
    }
    output
}

pub fn get_hash_hex(input: &str, hash_type: HashType) -> String {
    let mut output = [0u8; 32]; // 256 bits for SHA256
    unsafe {
        hash_function(
            input.as_ptr() as *const libc::c_char,
            output.as_mut_ptr(),
            hash_type,
        );
        let hex_c_string = hash_to_hex_string(output.as_ptr());
        let hex_string = std::ffi::CString::from_raw(hex_c_string)
            .into_string()
            .unwrap(); // This takes ownership and will deallocate when dropped
        hex_string.replace(":", "")
    }
}
