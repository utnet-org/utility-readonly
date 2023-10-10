extern crate leveldb_sys as leveldb;
use std::ffi::CString;
use std::ptr;

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

pub fn write_and_read_leveldb() {
    let options = unsafe { leveldb::leveldb_options_create() };
    unsafe { leveldb::leveldb_options_set_create_if_missing(options, 1) };

    let db_name = CString::new("./my_db").expect("CString::new failed");
    let mut err: *mut i8 = ptr::null_mut();

    let db = unsafe { leveldb::leveldb_open(options, db_name.as_ptr(), &mut err) };
    assert!(err.is_null(), "Failed to open LevelDB: {:?}", unsafe { std::ffi::CStr::from_ptr(err) });

    let key = "my_key\0";
    let value = "my_value\0";

    unsafe {
        leveldb::leveldb_put(
            db,
            leveldb::leveldb_writeoptions_create(),
            key.as_ptr() as *const _,
            key.len() as libc::size_t,
            value.as_ptr() as *const _,
            value.len() as libc::size_t,
            &mut err,
        );
    }
    assert!(err.is_null(), "Error writing to LevelDB: {:?}", unsafe { std::ffi::CStr::from_ptr(err) });

    let read_options = unsafe { leveldb::leveldb_readoptions_create() };
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

    if read_err.is_null() {
        println!(
            "Retrieved value value_len: {:?} for key '{}': {}",
            read_value_len,
            key,
           unsafe{ std::str::from_utf8(&std::slice::from_raw_parts(
                read_value as *const u8,
                read_value_len as usize
            ))}
                .unwrap()
        );
    } else {
        println!("Error retrieving value: {:?}", unsafe { std::ffi::CStr::from_ptr(read_err) });
    }

    //leveldb_free 释放内存 指定的变量的内存
    // leveldb_readoptions_destroy 销毁
    unsafe {
        leveldb::leveldb_free(read_value as *mut std::os::raw::c_void);
        leveldb::leveldb_readoptions_destroy(read_options);
        leveldb::leveldb_free(err as *mut std::os::raw::c_void);
    }

    unsafe {
        leveldb::leveldb_delete(
            db,
            leveldb::leveldb_writeoptions_create(),
            key.as_ptr() as *const _,
            key.len() as libc::size_t,
            &mut err,
        );
    }


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

    // 读取数据 不存在但不会报错 返回0和空的
    if read_err.is_null() {
        println!(
            "Retrieved value value_len: {:?} for key '{}': {}",
            read_value_len,
            key,
            unsafe{ std::str::from_utf8(&std::slice::from_raw_parts(
                read_value as *const u8,
                read_value_len as usize
            ))}
                .unwrap()
        );
    } else {
        println!("Error retrieving value: {:?}", unsafe { std::ffi::CStr::from_ptr(read_err) });
    }

    assert!(err.is_null(), "Error deleting value from LevelDB: {:?}", unsafe { std::ffi::CStr::from_ptr(err) });

    unsafe {
        leveldb::leveldb_free(err as *mut std::os::raw::c_void);
        leveldb::leveldb_close(db);
        leveldb::leveldb_options_destroy(options);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_and_close_leveldb() {
        open_and_close_leveldb();
        // Add assertions to check correctness of open_and_close_leveldb function
    }

    #[test]
    fn test_write_and_read_leveldb() {
        write_and_read_leveldb();
        // Add assertions to check correctness of write_and_read_leveldb function
    }

}

#[cfg(test)]
mod benches {
    // This will include the benchmark defined in benches.rs
    use criterion::criterion_group;
    include!("benches.rs");
}