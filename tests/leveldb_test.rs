extern crate leveldb_sys as leveldb;

#[cfg(test)]
mod tests {
    use std::ffi::CStr;
    use std::time::{Duration, SystemTime};
    use chrono::NaiveDateTime;
    use std::io::{self, Write};
    use leveldb::{leveldb_create_iterator, leveldb_iter_destroy, leveldb_iter_key, leveldb_iter_next, leveldb_iter_seek_to_first, leveldb_iter_valid, leveldb_iter_value, leveldb_readoptions_create};
    use utility::{delete_data, get_data, open_and_close_leveldb, open_leveldb, put_data};

    const VALUE_NUM: i32 = 1000000;


    #[test]
    fn test_open_and_close_leveldb() {
        open_and_close_leveldb();
        // Add assertions to check correctness of open_and_close_leveldb function
    }

    #[test]
    fn test_put_data_one() {
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
                    // println!("i:{}, Key: {}, Value: {}", i,key, value);
                    // 刷新标准输出缓冲
                    // io::stdout().flush().unwrap();
                }
                Err(err) => eprintln!("Failed to write data for key: {}. Error: {:?}", key, err),
            }
            i += 1;
        }

        // 获取结束时间
        let end_time = SystemTime::now();

        // 计算写入数据的耗时
        let duration = end_time.duration_since(start_time).unwrap();
        println!("Time taken to write {} entries: {:?}", i, duration);

        // Close LevelDB
        unsafe {
            leveldb::leveldb_close(db);
        }
    }

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

    #[test]
    fn test_get_data_one() {
        // Open LevelDB
        let db_path = "./my_db";
        let db = open_leveldb(db_path);
        assert!(!db.is_null(), "Failed to open LevelDB");

        // 获取当前时间
        let start_time = SystemTime::now();
        let target_duration = Duration::from_secs(60); // 一分钟

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

    #[test]
    fn test_get_data() {
        let db_path = "./my_db";
        let db = open_leveldb(db_path);
        assert!(!db.is_null(), "Failed to open LevelDB");

        let key = "batch_key1";

        // let data = get_data(db, &key);
        match get_data(db, &key) {
            Ok(val) => {
                println!("data Key: {}, Value: {},valueLen:{}",  key, val.0, val.1);
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

    #[test]
    fn test_delete_data_one() {
        // Open LevelDB
        let db_path = "./my_db";
        let db = open_leveldb(db_path);
        assert!(!db.is_null(), "Failed to open LevelDB");

        // 获取当前时间
        let start_time = SystemTime::now();
        let target_duration = Duration::from_secs(60); // 一分钟

        // 循环读取数据，直到超过目标时间
        let mut j = 0;
        while SystemTime::now().duration_since(start_time).unwrap() < target_duration {
            let key = format!("key{}", j);

            match delete_data(db, &key) {
                Ok(_val) => {
                    // println!("j:{}, Key: {}, Value: {},valueLen:{}", j,key, val.0,val.1);
                    // 刷新标准输出缓冲
                    // io::stdout().flush().unwrap();
                }
                Err(_) => eprintln!("Failed to read data for key: {}", key),
            }

            j += 1;
        }

        // 计算读取数据的数量和耗时
        let read_duration = SystemTime::now().duration_since(start_time).unwrap();
        println!("delete Time taken to read {} entries: {:?}", j, read_duration);

        // Close LevelDB
        unsafe {
            leveldb::leveldb_close(db);
        }
    }


    #[test]
    fn test_delete_data_for() {
        // Open LevelDB
        let db_path = "./my_db";
        let db = open_leveldb(db_path);
        assert!(!db.is_null(), "Failed to open LevelDB");


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


        // Write VALUE_NUM key-value pairs
        for i in 0..VALUE_NUM {
            let key = format!("key{}", i);
            delete_data(db, &key).expect("Error writing to LevelDB");
        }

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
        // let end_time_str = end_t.format("%Y-%m-%d %H:%M:%S").to_string();


        // Calculate the duration in milliseconds
        let duration = end_time.duration_since(start_time);
        println!("Time taken to write {} ,start_time:{} ,end_time:{}, entries: {:?}", VALUE_NUM, start_t, end_t, duration);

        // Close LevelDB
        unsafe { leveldb::leveldb_close(db) };
    }


    #[test]
    fn test_get_data_for() {
        // Open LevelDB
        let db_path = "./my_db";
        let db = open_leveldb(db_path);
        assert!(!db.is_null(), "Failed to open LevelDB");


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


        // Write VALUE_NUM key-value pairs
        for i in 0..VALUE_NUM {
            let key = format!("key{}", i);
            get_data(db, &key).expect("Error writing to LevelDB");
        }

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
        println!("Time taken to write {} ,start_time:{} ,end_time:{}, entries: {:?}", VALUE_NUM, start_t, end_t, duration);

        // Close LevelDB
        unsafe { leveldb::leveldb_close(db) };
    }

    #[test]
    fn test_put_data_for() {
        // Open LevelDB
        let db_path = "./my_db";
        let db = open_leveldb(db_path);
        assert!(!db.is_null(), "Failed to open LevelDB");


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
        // let start_time_str = start_t.format("%Y-%m-%d %H:%M:%S").to_string();


        // Write 50000 key-value pairs
        for i in 0..VALUE_NUM {
            let key = format!("key{}", i);
            let value = format!("value{}", i);

            put_data(db, &key, &value).expect("Error writing to LevelDB");
        }

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
        println!("put data Time taken to write {} ,start_time:{} ,end_time:{}, entries: {:?}", VALUE_NUM, start_t, end_t, duration);

        // Close LevelDB
        unsafe { leveldb::leveldb_close(db) };
    }

    //迭代数据
    #[test]
    fn test_iterator(){
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

    #[test]
    fn test_batch_put(){
        unsafe {
            // Open a LevelDB instance
            let db_path = "./my_db";
            let db = open_leveldb(db_path);
            assert!(!db.is_null(), "Failed to open LevelDB");
            let mut err: *mut i8 = std::ptr::null_mut();
            // Create a write batch
            let write_batch = leveldb_sys::leveldb_writebatch_create();

            // Add some put operations to the write batch
            let key1 = b"batch_key1".as_ptr() as *const i8;
            let value1 = b"batch_value1".as_ptr() as *const i8;
            let key2 = b"batch_key2".as_ptr() as *const i8;
            let value2 = b"batch_value2".as_ptr() as *const i8;
            // leveldb::leveldb_writebatch_clear(write_batch);

            // 批量写入
            leveldb_sys::leveldb_writebatch_put(write_batch, key1, 10, value1, 12);
            leveldb_sys::leveldb_writebatch_put(write_batch, key2, 10, value2, 12);

            // Commit the write batch
            let write_options = leveldb_sys::leveldb_writeoptions_create();
            // 最终提交批量写入
            leveldb_sys::leveldb_write(db, write_options, write_batch, &mut err);

            // 检查是否发生了错误
            if !err.is_null() {
                let error_message = CStr::from_ptr(err).to_string_lossy().to_string();
                println!("Error: {}", error_message);
            } else {
                println!("Batch write successful");
            }


            // 回滚写入
            // leveldb_sys::leveldb_writebatch_clear(write_batch);
            // Release resources
            leveldb_sys::leveldb_writebatch_destroy(write_batch);
            leveldb_sys::leveldb_writeoptions_destroy(write_options);
            leveldb_sys::leveldb_close(db);
        }
    }
}


