extern crate leveldb_sys as leveldb;

#[cfg(test)]
mod tests {
    use utility::{delete_data, get_data, open_leveldb, put_data, white_for};

    const VALUE_NUM: i32 = 1000000;

    #[test]
    fn test_put_get_delete_for() {
        // Open LevelDB
        let db_path = "./my_db";
        let db = open_leveldb(db_path);
        assert!(!db.is_null(), "Failed to open LevelDB");

        println!("open_leveldb end value_num:{}", VALUE_NUM);
        let put_data_for = || {
            for i in 0..VALUE_NUM {
                let key = format!("key{}", i);
                let value = format!("value{}", i);

                put_data(db, &key, &value).expect("Error writing to LevelDB");
            }
        };
        white_for(put_data_for);

        println!("put_data_for end");


        let get_data_for = || {
            for i in 0..VALUE_NUM {
                let key = format!("key{}", i);
                let value = format!("value{}", i);

                get_data(db, &key).expect("Error writing to LevelDB");
            }
        };
        white_for(get_data_for);
        println!("get_data_for end");


        let delete_data_for = || {
            for i in 0..VALUE_NUM {
                let key = format!("key{}", i);
                let value = format!("value{}", i);

                delete_data(db, &key).expect("Error writing to LevelDB");
            }
        };
        white_for(delete_data_for);
        println!("delete_data_for end");
        // Close LevelDB
        unsafe { leveldb::leveldb_close(db) };

        println!("test_put_get_delete_for end，close db");
    }


}