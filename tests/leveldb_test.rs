extern crate leveldb_sys as leveldb;

#[cfg(test)]
mod tests {
    use std::ffi::{CStr, CString};
    use std::time::{Duration, SystemTime};
    use chrono::NaiveDateTime;
    use std::io::{self, Write};
    use leveldb::{leveldb_create_iterator, leveldb_iter_destroy, leveldb_iter_key, leveldb_iter_next, leveldb_iter_seek_to_first, leveldb_iter_valid, leveldb_iter_value, leveldb_readoptions_create};
    use utility::{delete_data, get_data, open_and_close_leveldb, open_leveldb, put_data, white_for};
    use crate::{delete_callback, put_callback};

    const VALUE_NUM: i32 = 1000000;


    #[test]
    fn test_open_and_close_leveldb() {
        open_and_close_leveldb();
        // Add assertions to check correctness of open_and_close_leveldb function
    }

    // 一分钟普通写入多少条数据
    #[test]
    fn test_put_data_one() {
        // Open LevelDB
        let db_path = "./my_db";
        let db = open_leveldb(db_path);
        assert!(!db.is_null(), "Failed to open LevelDB");


        let put_data_for = || {
            // 循环写入数据，直到超过目标时间
            // 获取当前时间
            let start_time = SystemTime::now();
            let target_duration = Duration::from_secs(60); // 一分钟

            let mut i = 1;
            while SystemTime::now().duration_since(start_time).unwrap() < target_duration {
                let key = format!("key{}", i);
                let value = format!("value{}", i);

                put_data(db, &key, &value).expect("Put Data Error writing to LevelDB ");
                i += 1;
            }
            println!("put-data 1 分钟 Time taken to write {} ", i);
        };
        white_for(put_data_for);
        // Close LevelDB
        unsafe {
            leveldb::leveldb_close(db);
        }
    }

    // 一分钟普通写入多少条数据、读取到多少条数据
    #[test]
    fn test_put_and_get_data_one() {
        // Open LevelDB
        let db_path = "./my_db";
        let db = open_leveldb(db_path);
        assert!(!db.is_null(), "Failed to open LevelDB");

        // 获取当前时间
        let start_time = SystemTime::now();
        let target_duration = Duration::from_secs(60); // 一分钟

        // 循环写入数据，直到超过目标时间
        let mut i = 1;
        while SystemTime::now().duration_since(start_time).unwrap() < target_duration {
            let key = format!("key{}", i);
            let value = format!("value{}", i);

            match put_data(db, &key, &value) {
                Ok(_) => {
                    println!("i:{}, Key: {}, Value: {}", i, key, value);
                    // 刷新标准输出缓冲
                    io::stdout().flush().unwrap();
                }
                Err(_) => eprintln!("Failed to write data for key: {}", key),
            }
            i += 1;
        }

        // 计算写入数据的数量和耗时
        let write_duration = SystemTime::now().duration_since(start_time).unwrap();
        println!("Time taken to write {} entries: {:?}", i, write_duration);

        // 循环读取数据，直到超过目标时间
        let mut j = 0;
        while SystemTime::now().duration_since(start_time).unwrap() < target_duration {
            let key = format!("key{}", j);

            match get_data(db, &key) {
                Ok(val) => {
                    println!("j:{}, Key: {}, Value: {},valueLen:{}", j, key, val.0, val.1);
                    // 刷新标准输出缓冲
                    io::stdout().flush().unwrap();
                }
                Err(_) => eprintln!("Failed to read data for key: {}", key),
            }

            j += 1;
        }

        // 计算读取数据的数量和耗时
        let read_duration = SystemTime::now().duration_since(start_time).unwrap();
        println!("Time taken to read {} entries: {:?}", j, read_duration);

        // Close LevelDB
        unsafe {
            leveldb::leveldb_close(db);
        }
    }

    // 一分钟读取到多少条数据
    #[test]
    fn test_get_data_one() {
        // Open LevelDB
        let db_path = "./my_db";
        let db = open_leveldb(db_path);
        assert!(!db.is_null(), "Failed to open LevelDB");
        let get_data_for = || {
            // 循环写入数据，直到超过目标时间
            // 获取当前时间
            let start_time = SystemTime::now();
            let target_duration = Duration::from_secs(60); // 一分钟

            let mut i = 1;
            while SystemTime::now().duration_since(start_time).unwrap() < target_duration {
                let key = format!("key{}", i);
                get_data(db, &key).expect("Get Data Error writing to LevelDB ");
                i += 1;
            }
            println!("get-data 1 分钟 Time taken to write {} ", i);
        };
        white_for(get_data_for);
        // Close LevelDB
        unsafe {
            leveldb::leveldb_close(db);
        }
    }

    #[test]
    fn test_get_data() {
        let db_path = "./my_db";
        let db = open_leveldb(db_path);
        assert!(!db.is_null(), "Failed to open LevelDB");

        let key = "batch_key1";

        // let data = get_data(db, &key);
        match get_data(db, &key) {
            Ok(val) => {
                println!("data Key: {}, Value: {},valueLen:{}", key, val.0, val.1);
                // 刷新标准输出缓冲
                io::stdout().flush().unwrap();
            }
            Err(err) => eprintln!("Error retrieving data: {}", err),
        }
    }

    #[test]
    fn test_put_data() {
        let db_path = "./my_db";
        let db = open_leveldb(db_path);
        assert!(!db.is_null(), "Failed to open LevelDB");
        // let mut _err: *mut i8 = ptr::null_mut();

        let key = "6";
        // 写入数据
        let value = "这是value";
        put_data(db, &key, &value).unwrap();
    }

    // 一分钟删除多少条数据
    #[test]
    fn test_delete_data_one() {
        // Open LevelDB
        let db_path = "./my_db";
        let db = open_leveldb(db_path);
        assert!(!db.is_null(), "Failed to open LevelDB");

        let delete_data_for = || {
            // 循环写入数据，直到超过目标时间
            // 获取当前时间
            let start_time = SystemTime::now();
            let target_duration = Duration::from_secs(60); // 一分钟

            let mut i = 1;
            while SystemTime::now().duration_since(start_time).unwrap() < target_duration {
                let key = format!("key{}", i);
                delete_data(db, &key).expect("Delete Data Error writing to LevelDB ");
                i += 1;
            }
            println!("get-data 1 分钟 Time taken to write {} ", i);
        };
        white_for(delete_data_for);
        // Close LevelDB
        unsafe {
            leveldb::leveldb_close(db);
        }
    }


    // 删除 VALUE_NUM的数据 耗时
    #[test]
    fn test_delete_data_for() {
        // Open LevelDB
        let db_path = "./my_db";
        let db = open_leveldb(db_path);
        assert!(!db.is_null(), "Failed to open LevelDB");

        let delete_data_for = || {
            for i in 0..VALUE_NUM {
                let key = format!("key{}", i);

                delete_data(db, &key).expect("Error writing to LevelDB");
            }
        };
        white_for(delete_data_for);
        println!("delete_data_for end");
        // Close LevelDB
        unsafe { leveldb::leveldb_close(db) };
    }

    // 读取 VALUE_NUM的数据 耗时
    #[test]
    fn test_get_data_for() {
        // Open LevelDB
        let db_path = "./my_db";
        let db = open_leveldb(db_path);
        assert!(!db.is_null(), "Failed to open LevelDB");

        let get_data_for = || {
            for i in 0..VALUE_NUM {
                let key = format!("key{}", i);
                get_data(db, &key).expect("Error writing to LevelDB");
            }
        };
        white_for(get_data_for);
        println!("get_data_for end");
        // Close LevelDB
        unsafe { leveldb::leveldb_close(db) };
    }

    // 写入 VALUE_NUM的数据 耗时
    #[test]
    fn test_put_data_for() {
        // Open LevelDB
        let db_path = "./my_db";
        let db = open_leveldb(db_path);
        assert!(!db.is_null(), "Failed to open LevelDB");
        let put_data_for = || {
            for i in 0..VALUE_NUM {
                let key = format!("key{}", i);
                let value = format!("value{}", i);

                put_data(db, &key, &value).expect("Error writing to LevelDB");
            }
        };
        white_for(put_data_for);

        println!("put_data_for end");
        // Close LevelDB
        unsafe { leveldb::leveldb_close(db) };
    }

    //迭代数据
    #[test]
    fn test_iterator() {
        let mut length: usize = 0;
        let db_path = "./my_db";
        let db = open_leveldb(db_path);
        let read_options = unsafe { leveldb_readoptions_create() };
        assert!(!db.is_null(), "Failed to open LevelDB");
        let iterator = unsafe { leveldb_create_iterator(db, read_options) };
        unsafe { leveldb_iter_seek_to_first(iterator) };
        while unsafe { leveldb_iter_valid(iterator) } != 0 {
            let key_ptr = unsafe { leveldb_iter_key(iterator, &mut length) };
            let value_ptr = unsafe { leveldb_iter_value(iterator, &mut length) };
            let key = unsafe { std::slice::from_raw_parts(key_ptr as *const u8, length) };
            let value = unsafe { std::slice::from_raw_parts(value_ptr as *const u8, length) };
            println!("Key: {:?}, Value: {:?}", std::str::from_utf8(key).unwrap(), std::str::from_utf8(value).unwrap());
            unsafe { leveldb_iter_next(iterator) };
        }
        unsafe { leveldb_iter_destroy(iterator) };
    }

    // 批量写入 VALUE_NUM的数据 耗时
    #[test]
    fn test_batch_put_for() {
        unsafe {
            // Open a LevelDB instance
            let db_path = "./my_db";
            let db = open_leveldb(db_path);
            assert!(!db.is_null(), "Failed to open LevelDB");
            let write_batch = leveldb_sys::leveldb_writebatch_create();
            let write_options = leveldb_sys::leveldb_writeoptions_create();
            let mut err: *mut i8 = std::ptr::null_mut();

            let put_data_for = || {
                for i in 0..VALUE_NUM {
                    let key = format!("batch_test_key{}", i);
                    let value = format!("batch_value{}", i);
                    // 将传入的键和值转换为 C 字符串
                    // let key_cstr = CString::new(key).map_err(|_| "CString::new for key failed".to_string()).unwrap();
                    // let value_cstr = CString::new(value).map_err(|_| "CString::new for value failed".to_string()).unwrap();
                    leveldb_sys::leveldb_writebatch_put(write_batch,
                                                        key.as_ptr() as *const i8, key.len(), value.as_ptr() as *const i8, value.len());
                }
                // Commit the write batch
                // 最终提交批量写入
                leveldb_sys::leveldb_write(db, write_options, write_batch, &mut err);

                // 检查是否发生了错误
                if err.is_null() {
                    println!("Batch write successful");
                } else {
                    let error_message = CStr::from_ptr(err).to_string_lossy().to_string();
                    println!("Error: {}", error_message);
                }
            };


            white_for(put_data_for);

            println!("batch_put end");
            // 回滚写入  清空写批量，即删除批量中的所有操作，但不影响 LevelDB 数据库中的数据
            // leveldb_sys::leveldb_writebatch_clear(write_batch);
            leveldb_sys::leveldb_writebatch_destroy(write_batch);
            leveldb_sys::leveldb_writeoptions_destroy(write_options);
            leveldb_sys::leveldb_close(db);
        }
    }

    // 读取批量写入的，（普通读取一样）不同的是key。
    #[test]
    fn test_batch_get_for() {
        // Open a LevelDB instance
        let db_path = "./my_db";
        let db = open_leveldb(db_path);
        assert!(!db.is_null(), "Failed to open LevelDB");

        let get_data_for = || {
            for i in 0..VALUE_NUM {
                let key = format!("batch_test_key{}", i);
                get_data(db, &key).expect("Error writing to LevelDB");
                // match get_data(db, &key) {
                //     Ok(val) => {
                //         println!("i:{}, Key: {}, Value: {},valueLen:{}", i, key, val.0, val.1);
                //     }
                //     Err(err) => eprintln!("Error retrieving data: {}", err),
                // }
            }
        };
        white_for(get_data_for);
        println!("test_batch_get_for end");
    }

    // 批量删除 VALUE_NUM的数据 耗时
    #[test]
    fn test_batch_delete_for() {
        // Open LevelDB
        unsafe {
            let db_path = "./my_db";
            let db = open_leveldb(db_path);
            assert!(!db.is_null(), "Failed to open LevelDB");

            println!("open_leveldb end value_num:{}", VALUE_NUM);
            let write_batch = leveldb_sys::leveldb_writebatch_create();
            let write_options = leveldb_sys::leveldb_writeoptions_create();
            let mut err: *mut i8 = std::ptr::null_mut();

            let put_data_for = || {
                for i in 0..VALUE_NUM {
                    let key = format!("batch_test_key{}", i);
                    // 将传入的键和值转换为 C 字符串
                    // let key_cstr = CString::new(key).map_err(|_| "CString::new for key failed".to_string()).unwrap();
                    leveldb_sys::leveldb_writebatch_delete(write_batch,
                                                           key.as_ptr() as *const i8, key.len());
                }
                // Commit the write batch
                // 最终提交批量写入
                leveldb_sys::leveldb_write(db, write_options, write_batch, &mut err);

                // 检查是否发生了错误
                if err.is_null() {
                    println!("Batch delete successful");
                } else {
                    let error_message = CStr::from_ptr(err).to_string_lossy().to_string();
                    println!("Error: {}", error_message);
                }
            };


            white_for(put_data_for);

            println!("test_batch_delete_for end");
            // 回滚写入
            // leveldb_sys::leveldb_writebatch_clear(write_batch);
            leveldb_sys::leveldb_writebatch_destroy(write_batch);
            leveldb_sys::leveldb_writeoptions_destroy(write_options);
            leveldb_sys::leveldb_close(db);

            println!("leveldb_batch_delete_max_for end，close db");
        }
    }

    // 用来迭代批量写入操作的函数，可以处理批量写入中的每个操作，无论是写入还是删除操作
    #[test]
    fn test_iterate() {

        let db_path = "./my_db";
        let db = open_leveldb(db_path);
        assert!(!db.is_null(), "Failed to open LevelDB");
        // 创建批量写入对象
        let write_batch = unsafe { leveldb_sys::leveldb_writebatch_create() };

        // 添加多个写入操作到批量写入对象
        let max_operations = 50;
        for i in 0..max_operations {
            let key = format!("key_to_write{}", i);
            let value = format!("value_to_write{}", i);

            // 转换为 C 字符串
            let key_cstr = CString::new(key).expect("Failed to create CString for key");
            let value_cstr = CString::new(value).expect("Failed to create CString for value");

            unsafe {
                leveldb_sys::leveldb_writebatch_put(
                    write_batch,
                    key_cstr.as_ptr(),
                    key_cstr.as_bytes().len(),
                    value_cstr.as_ptr(),
                    value_cstr.as_bytes().len(),
                );
            }
        }

        // 添加多个删除操作到批量写入对象
        for i in 0..max_operations {
            let key = format!("batch_test_key{}", i);

            // 转换为 C 字符串
            let key_cstr = CString::new(key).expect("Failed to create CString for key");

            unsafe {
                leveldb_sys::leveldb_writebatch_delete(
                    write_batch,
                    key_cstr.as_ptr(),
                    key_cstr.as_bytes().len(),
                );
            }
        }


        let state: *mut std::os::raw::c_void = std::ptr::null_mut();

        // 调用迭代函数
        unsafe {
            let write_options = leveldb_sys::leveldb_writeoptions_create();

            leveldb::leveldb_writebatch_iterate(
                write_batch,
                state,
                put_callback,
                delete_callback,
            );
            let mut err: *mut i8 = std::ptr::null_mut();

            // 提交批量写入和删除的操作
            // 最终提交批量写入
            leveldb_sys::leveldb_write(db, write_options, write_batch, &mut err);
            // 销毁批量写入对象
            leveldb_sys::leveldb_writebatch_destroy(write_batch);
            leveldb_sys::leveldb_writeoptions_destroy(write_options);
            leveldb::leveldb_close(db);
        }
    }
}


extern "C" fn put_callback(_: *mut std::os::raw::c_void, key: *const i8, klen: usize, val: *const i8, vlen: usize) {
    // 在这里处理写入操作，这里只是简单打印
    unsafe {
        let key_str = std::str::from_utf8_unchecked(std::slice::from_raw_parts(key as *const u8, klen));
        let value_str = std::str::from_utf8_unchecked(std::slice::from_raw_parts(val as *const u8, vlen));
        println!("Put: {} => {}", key_str, value_str);
    }
}

extern "C" fn delete_callback(_: *mut std::os::raw::c_void, key: *const i8, klen: usize) {
    // 在这里处理删除操作，这里只是简单打印
    unsafe {
        let key_str = std::str::from_utf8_unchecked(std::slice::from_raw_parts(key as *const u8, klen));
        println!("Delete: {}", key_str);
    }
}

