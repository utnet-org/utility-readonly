extern crate leveldb_sys as leveldb;

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime};
    use chrono::NaiveDateTime;
    use std::io::{self, Write};
    use std::ptr;
    use utility::{delete_data, get_data, open_and_close_leveldb, open_leveldb, put_data};

    const value_num: i32 = 1000000;


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

        let key = "key12661507";

        let data = get_data(db, &key);

        println!("data:{:?}", data.ok());
    }

    #[test]
    fn test_put_data() {
        let db_path = "./my_db";
        let db = open_leveldb(db_path);
        assert!(!db.is_null(), "Failed to open LevelDB");
        let mut err: *mut i8 = ptr::null_mut();

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
                Ok(val) => {
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
        let start_time_str = start_t.format("%Y-%m-%d %H:%M:%S").to_string();


        // Write value_num key-value pairs
        for i in 0..value_num {
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
        let end_time_str = end_t.format("%Y-%m-%d %H:%M:%S").to_string();


        // Calculate the duration in milliseconds
        let duration = end_time.duration_since(start_time);
        println!("Time taken to write {} ,start_time:{} ,end_time:{}, entries: {:?}", value_num, start_t, end_t, duration);

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
        let start_time_str = start_t.format("%Y-%m-%d %H:%M:%S").to_string();


        // Write value_num key-value pairs
        for i in 0..value_num {
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
        let end_time_str = end_t.format("%Y-%m-%d %H:%M:%S").to_string();


        // Calculate the duration in milliseconds
        let duration = end_time.duration_since(start_time);
        println!("Time taken to write {} ,start_time:{} ,end_time:{}, entries: {:?}", value_num, start_t, end_t, duration);

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
        let start_time_str = start_t.format("%Y-%m-%d %H:%M:%S").to_string();


        // Write 50000 key-value pairs
        for i in 0..value_num {
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
        let end_time_str = end_t.format("%Y-%m-%d %H:%M:%S").to_string();


        // Calculate the duration in milliseconds
        let duration = end_time.duration_since(start_time);
        println!("put data Time taken to write {} ,start_time:{} ,end_time:{}, entries: {:?}", value_num, start_t, end_t, duration);

        // Close LevelDB
        unsafe { leveldb::leveldb_close(db) };
    }

}


