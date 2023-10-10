extern crate leveldb_sys as leveldb;

use utility::{delete_data, get_data, open_leveldb, put_data, white_for};
use std::io::{self, Write};

fn main() {
    leveldb_min();

    leveldb_max();
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
            get_data(db, &key).expect("Error writing to LevelDB");
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