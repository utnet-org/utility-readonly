extern crate leveldb_sys as leveldb;

use std::ffi::{c_char, CStr, CString};
use utility::{delete_data, get_data, open_leveldb, put_data, white_for};
use std::io::{self, Write};
use std::ptr;
use libc::size_t;

fn main() {
    // leveldb_min();
    //
    // leveldb_max();

    // iterator();
    //
    // test();

    leveldb_batch_put_max();

    // leveldb_get_batch_put_max();

    // leveldb_batch_delete_max();
}

// 批量删除
fn leveldb_batch_delete_max(){
    let max = 1000000;
    // Open LevelDB
    unsafe {

        let db_path = "./my_db";
        let db = open_leveldb(db_path);
        assert!(!db.is_null(), "Failed to open LevelDB");

        println!("open_leveldb end value_num:{}", max);
        let write_batch = leveldb_sys::leveldb_writebatch_create();
        let write_options = leveldb_sys::leveldb_writeoptions_create();
        let mut err: *mut i8 = std::ptr::null_mut();

        let put_data_for =  || {
            for i in 0..max {
                let key = format!("batch_key{}", i);
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

        println!("batch_put end");
        // 回滚写入
        // leveldb_sys::leveldb_writebatch_clear(write_batch);
        leveldb_sys::leveldb_writebatch_destroy(write_batch);
        leveldb_sys::leveldb_writeoptions_destroy(write_options);
        leveldb_sys::leveldb_close(db);

        println!("leveldb_batch_put_max_for end，close db");
    }
}

// 批量写入
fn leveldb_batch_put_max(){
    let max = 1000000;
    // Open LevelDB
   unsafe {

       let db_path = "./my_db";
       let db = open_leveldb(db_path);
       assert!(!db.is_null(), "Failed to open LevelDB");

       println!("open_leveldb end value_num:{}", max);
       let write_batch = leveldb_sys::leveldb_writebatch_create();
       let write_options = leveldb_sys::leveldb_writeoptions_create();
       let mut err: *mut i8 = std::ptr::null_mut();

       let put_data_for =  || {
           for i in 0..max {
               let key = format!("batch_key{}", i);
               let value = format!("batch_value{}", i) ;
               // 将传入的键和值转换为 C 字符串
               // let key_cstr = CString::new(key).map_err(|_| "CString::new for key failed".to_string()).unwrap();
               // let value_cstr = CString::new(value).map_err(|_| "CString::new for value failed".to_string()).unwrap();
               leveldb_sys::leveldb_writebatch_put(write_batch,
                                                   key.as_ptr() as *const i8, key.len(), value.as_ptr() as *const i8,value.len());
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

       println!("leveldb_batch_put_max_for end，close db");
   }
}

// 读取批量写入的key
fn leveldb_get_batch_put_max() {
    let max = 1000000;
    // Open LevelDB
    let db_path = "./my_db";
    let db = open_leveldb(db_path);
    assert!(!db.is_null(), "Failed to open LevelDB");

    println!("open_leveldb end value_num:{}", max);


    let get_data_for = || {
        for i in 0..max {
            let key = format!("batch_key{}", i);
            // get_data(db, &key).expect("Error writing to LevelDB");
            match get_data(db, &key) {
                Ok(val) => {
                    println!("i:{}, Key: {}, Value: {},valueLen:{}", i, key, val.0, val.1);
                }
                Err(err) => eprintln!("Error retrieving data: {}", err),
            }
        }
    };
    white_for(get_data_for);
    println!("get_data_for end");

    // Close LevelDB
    unsafe { leveldb::leveldb_close(db) };

    println!("test_put_get_delete_for end，close db");
}

fn leveldb_min() {
    let min = 100;
    // Open LevelDB
    let db_path = "./my_db";
    let db = open_leveldb(db_path);
    assert!(!db.is_null(), "Failed to open LevelDB");

    let put_data_for = || {
        for i in 0..min {
            let key = format!("key{}", i);
            let value = format!("value{}", i);

            put_data(db, &key, &value).expect("Error writing to LevelDB");
        }
    };
    white_for(put_data_for);

    println!("put_data_for end");


    let get_data_for = || {
        for i in 0..min {
            let key = format!("key{}", i);
            // get_data(db, &key).expect("Error writing to LevelDB");
            match get_data(db, &key) {
                Ok(val) => {
                    println!("i:{}, Key: {}, Value: {},valueLen:{}", i, key, val.0, val.1);
                    // 刷新标准输出缓冲
                    io::stdout().flush().unwrap();
                }
                Err(err) => eprintln!("Error retrieving data: {}", err),
            }
        }
    };
    white_for(get_data_for);
    println!("get_data_for end");


    let delete_data_for = || {
        for i in 0..min {
            let key = format!("key{}", i);

            delete_data(db, &key).expect("Error writing to LevelDB");
        }
    };
    white_for(delete_data_for);
    println!("delete_data_for end");
    // Close LevelDB
    unsafe { leveldb::leveldb_close(db) };

    println!("test_put_get_delete_for end，close db");
}


fn leveldb_max() {
    let max = 1000000;
    // Open LevelDB
    let db_path = "./my_db";
    let db = open_leveldb(db_path);
    assert!(!db.is_null(), "Failed to open LevelDB");

    println!("open_leveldb end value_num:{}", max);
    let put_data_for = || {
        for i in 0..max {
            let key = format!("key{}", i);
            let value = format!("value{}", i);

            put_data(db, &key, &value).expect("Error writing to LevelDB");
        }
    };
    white_for(put_data_for);

    println!("put_data_for end");


    let get_data_for = || {
        for i in 0..max {
            let key = format!("key{}", i);
            get_data(db, &key).expect("Error writing to LevelDB");
        }
    };
    white_for(get_data_for);
    println!("get_data_for end");


    let delete_data_for = || {
        for i in 0..max {
            let key = format!("key{}", i);

            delete_data(db, &key).expect("Error writing to LevelDB");
        }
    };
    white_for(delete_data_for);
    println!("delete_data_for end");
    // Close LevelDB
    unsafe { leveldb::leveldb_close(db) };

    println!("test_put_get_delete_for end，close db");
}

fn iterator(){
    unsafe {
        // 创建 Write Batch
        let write_batch = leveldb::leveldb_writebatch_create();

        // 添加 Put 和 Delete 操作到 Write Batch
        let key_put = "key_to_put".as_ptr() as *const c_char;
        let keylen_put = "key_to_put".len() as size_t;
        let value_put = "value_to_put".as_ptr() as *const c_char;
        let vallen_put = "value_to_put".len() as size_t;

        leveldb::leveldb_writebatch_put(write_batch, key_put, keylen_put, value_put, vallen_put);

        // let key_delete = "key_to_delete".as_ptr() as *const c_char;
        // let keylen_delete = "key_to_delete".len() as size_t;

        // leveldb_writebatch_delete(write_batch, key_delete, keylen_delete);

        // 清空 Write Batch
        // leveldb_writebatch_clear(write_batch);

        // 销毁 Write Batch
        leveldb::leveldb_writebatch_destroy(write_batch);
    }
}

