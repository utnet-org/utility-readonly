// examples/basic_operations.rs

extern crate leveldb_sys as leveldb;

use std::ffi::{CString};
use std::ptr;

fn main() {
    // LevelDB 配置
    let options = unsafe { leveldb::leveldb_options_create() };
    unsafe { leveldb::leveldb_options_set_create_if_missing(options, 1); } // 设置为true

    let db_name = CString::new("./my_db").expect("CString::new failed");
    let mut err: *mut i8 = ptr::null_mut();

    // 打开 LevelDB 数据库 LevelDB 实例指针
    let db = unsafe { leveldb::leveldb_open(options, db_name.as_ptr(), &mut err) };
    if err.is_null() {
        println!("Successfully opened the LevelDB database.");
        // 写入数据
        let key = "这是value\0";
        let value = "这是hello\0";
        unsafe {
            leveldb::leveldb_put(
                db,
                leveldb::leveldb_writeoptions_create(),//创建用于写入
                key.as_ptr() as *const _,
                key.len() as libc::size_t, //获取 key 字符串的长度，并转换为 libc::size_t 类型，表示 key 的长度。
                value.as_ptr() as *const _,//获取 value 字符串的指针
                value.len() as libc::size_t,
                &mut err, // 错误指针，用于接收写入操作的错误信息
            );
        }

        // 查询数据
        let  read_options = unsafe { leveldb::leveldb_readoptions_create() };
        let mut read_err: *mut i8 = ptr::null_mut();
        let mut read_value_len: libc::size_t = 0;
        let read_value = unsafe {
            leveldb::leveldb_get(
                db,
                read_options,
                key.as_ptr() as *const _,
                key.len() as libc::size_t,
                &mut read_value_len,
                &mut read_err,
            )
        };

        // // 将返回的 C 风格字符串转换为 Rust 字符串
        // let value_cstr = unsafe { CStr::from_ptr(read_value) };
        // let value_str = value_cstr.to_str().expect("Invalid UTF-8 sequence");
        //
        // println!("read_value_len: {:?} read_value:{} ", read_value_len,value_str);


        if read_err.is_null() {
            unsafe {
                println!(
                    "Retrieved value value_len: {:?} for key '{}': {}",read_value_len,key,
                    std::str::from_utf8(&std::slice::from_raw_parts(
                        read_value as *const u8,
                        read_value_len as usize
                    )).unwrap()
                );
            }
        } else {
            println!("Error retrieving value: {:?}", unsafe {
                std::ffi::CStr::from_ptr(read_err)
            });
        }

        // 修改数据
        let new_value = "hello修改new_value\0";
        unsafe {
            leveldb::leveldb_put(
                db,
                leveldb::leveldb_writeoptions_create(),
                key.as_ptr() as *const _,
                key.len(),
                new_value.as_ptr() as *const _,
                new_value.len(),
                &mut err,
            );
        }
        println!("Value for key 'my_key' modified.");
        let mut modified_read_err: *mut i8 = ptr::null_mut();
        let mut modified_read_value_len: libc::size_t = 0;
        let modified_read_value = unsafe {
            leveldb::leveldb_get(
                db,
                read_options,
                key.as_ptr() as *const _,
                key.len() as libc::size_t,
                &mut modified_read_value_len,
                &mut modified_read_err,
            )
        };

        if modified_read_err.is_null() {
            unsafe {
                println!(
                    "modified_Retrieved value value_len: {:?} for key '{}': {}",modified_read_value_len,key,
                    std::str::from_utf8(&std::slice::from_raw_parts(
                        modified_read_value as *const u8,
                        modified_read_value_len as usize
                    )).unwrap()
                );
            }
        } else {
            println!("Error retrieving value: {:?}", unsafe {
                std::ffi::CStr::from_ptr(modified_read_err)
            });
        }

        // 删除数据
        unsafe {
            leveldb::leveldb_delete(
                db,
                leveldb::leveldb_writeoptions_create(),
                key.as_ptr() as *const _,
                key.len() as libc::size_t,
                &mut err,
            );
        }
        println!("Value for key 'my_key' deleted.");

        // 释放资源
        unsafe {
            leveldb::leveldb_free(read_value as *mut std::os::raw::c_void);
            leveldb::leveldb_readoptions_destroy(read_options);
            leveldb::leveldb_free(err as *mut std::os::raw::c_void);
        }
    } else {
        unsafe {
            println!("Failed to open LevelDB: {:?}", std::ffi::CStr::from_ptr(err));
            leveldb::leveldb_free(err as *mut std::os::raw::c_void);
        }
    }

    // 关闭数据库并释放资源
    unsafe {
        leveldb::leveldb_close(db);
        leveldb::leveldb_options_destroy(options);
    }
}