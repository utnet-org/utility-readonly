extern crate leveldb_sys as leveldb;

use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;
use std::time::SystemTime;
use chrono::NaiveDateTime;
use leveldb::leveldb_t;

// 传入db路径 返回db指针
pub fn open_leveldb(db_path: &str) -> *mut leveldb_t {
    // 将传入的数据库路径转换为 CString
    let db_path_cstr = CString::new(db_path).expect("CString::new failed");

    // LevelDB 选项
    let options = unsafe { leveldb::leveldb_options_create() };
    unsafe { leveldb::leveldb_options_set_create_if_missing(options, 1) }; // 设置 create_if_missing 为 true

    // 打开 LevelDB 数据库
    let mut err: *mut i8 = ptr::null_mut();
    let db = unsafe {
        leveldb::leveldb_open(options, db_path_cstr.as_ptr(), &mut err)
    };

    if err.is_null() {
        db
    } else {
        unsafe {
            leveldb::leveldb_free(err as *mut c_void);
            panic!("Unable to open LevelDB: {:?}", std::ffi::CStr::from_ptr(err))
        };
    }
}

// 关闭数据库
pub fn close_leveldb(db: *mut leveldb_t) {
    unsafe {
        leveldb::leveldb_close(db);
    }
}

// 写入数据
pub fn put_data(db: *mut leveldb_t, key: &str, value: &str) -> Result<(), String> {
    // 将传入的键和值转换为 C 字符串
    // let key_cstr = CString::new(key).map_err(|_| "CString::new for key failed".to_string())?;
    // let value_cstr = CString::new(value).map_err(|_| "CString::new for value failed".to_string())?;

    // 写入数据
    let mut err: *mut i8 = ptr::null_mut();
    unsafe {
        leveldb::leveldb_put(
            db,
            leveldb::leveldb_writeoptions_create(),
            key.as_ptr() as *const _,
            key.len(),
            value.as_ptr() as *const _,
            value.len(),
            &mut err,
        );
    }

    // 检查是否写入成功
    if err.is_null() {
        Ok(())
    } else {
        unsafe {
            leveldb_sys::leveldb_free(err as *mut std::os::raw::c_void);
        }
        Err("Error writing to LevelDB".to_string())
    }
}

// 读取数据
pub fn get_data(db: *mut leveldb_t, key: &str) -> Result<(String, usize), String> {
    let mut read_err: *mut i8 = ptr::null_mut();
    let mut read_value_len: libc::size_t = 0;

    let read_value = unsafe {
        leveldb::leveldb_get(
            db,
            leveldb::leveldb_readoptions_create(),
            key.as_ptr() as *const _,
            key.len() as libc::size_t,
            &mut read_value_len,
            &mut read_err,
        )
    };

    if read_err.is_null() {
        let value = unsafe {
            std::str::from_utf8(&std::slice::from_raw_parts(
                read_value as *const u8,
                read_value_len as usize,
            ))
                .unwrap()
                .to_string()
        };

        unsafe {
            leveldb::leveldb_free(read_value as *mut std::os::raw::c_void);
        }

        Ok((value, read_value_len))
    } else {
        unsafe {
            leveldb::leveldb_free(read_err as *mut std::os::raw::c_void);
        }

        Err("无法读取数据".to_string())
    }
}

// 删除数据
pub fn delete_data(db: *mut leveldb_t, key: &str) -> Result<(), String> {
    let key_cstr = CString::new(key).expect("CString::new failed");

    let mut err: *mut i8 = ptr::null_mut();
    unsafe {
        leveldb::leveldb_delete(
            db,
            leveldb::leveldb_writeoptions_create(),
            key_cstr.as_ptr() as *const _,
            key.len() as libc::size_t,
            &mut err,
        );
    }

    if err.is_null() {
        Ok(())
    } else {
        unsafe {
            leveldb::leveldb_free(err as *mut std::os::raw::c_void);
        }
        Err("Unable to delete data".to_string())
    }
}


pub fn open_and_close_leveldb() {
    let options = unsafe { leveldb::leveldb_options_create() };
    unsafe { leveldb::leveldb_options_set_create_if_missing(options, 1) };

    let db_name = CString::new("./my_db").expect("CString::new failed");
    let mut err: *mut i8 = ptr::null_mut();

    let db = unsafe { leveldb::leveldb_open(options, db_name.as_ptr(), &mut err) };
    assert!(err.is_null(), "Failed to open LevelDB: {:?}", unsafe { std::ffi::CStr::from_ptr(err) });

    unsafe {
        leveldb::leveldb_close(db);
        leveldb::leveldb_options_destroy(options);
    }
}


pub fn white_for<F>(c: F)
    where
        F: Fn()
{

// 获取当前时间
    let start_time = SystemTime::now();

// 计算时间戳（以纳秒为单位）
    let start_t = match start_time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => {
            let seconds = duration.as_secs() as i64;
            let nanos = duration.subsec_nanos() as u32;
            NaiveDateTime::from_timestamp(seconds, nanos)
        }
        Err(err) => {
            // Handle error
            eprintln!("Error: {:?}", err);
            return;
        }
    };

    // 格式化为特定格式的日期时间字符串
    let _start_time_str = start_t.format("%Y-%m-%d %H:%M:%S").to_string();

    c();

// 获取当前时间
    let end_time = SystemTime::now();

// 计算时间戳（以纳秒为单位）
    let end_t = match end_time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => {
            let seconds = duration.as_secs() as i64;
            let nanos = duration.subsec_nanos() as u32;
            NaiveDateTime::from_timestamp(seconds, nanos)
        }
        Err(err) => {
            // Handle error
            eprintln!("Error: {:?}", err);
            return;
        }
    };

    // 格式化为特定格式的日期时间字符串
    let _end_time_str = end_t.format("%Y-%m-%d %H:%M:%S").to_string();


    // Calculate the duration in milliseconds
    let duration = end_time.duration_since(start_time);
    println!("Time taken to write ,start_time_str:{} ,end_time_str:{}, entries: {:?}", _start_time_str, _end_time_str, duration);
    println!("Time taken to write ,start_time:{} ,end_time:{}, entries: {:?}", start_t, end_t, duration);
}

