// db created and exported from here
extern crate lmdb_rs as lmdb;

use log::error;
use lmdb::{EnvBuilder, DbFlags, Environment, DbHandle};

use crate::utils;

pub struct Interface {
    pub env: Environment,
    pub handle: DbHandle,
}

impl  Interface {
    pub fn open() -> Self {
        let env = EnvBuilder::new().open("test-lmdb", 0o777).unwrap(); 
        let handle = env.get_default_db(DbFlags::empty()).unwrap();
        Interface { env, handle }
    }
    pub fn write(e: &Environment, h: &DbHandle, k: &str, v: &str) {
        let txn = e.new_transaction().unwrap();
        {
            // get a database bound to this transaction
            let db = txn.bind(&h); 
            let pair = vec![(k,v)];
            for &(key, value) in pair.iter() { db.set(&key, &value).unwrap(); }
        }
        match txn.commit() {
            Err(_) => error!("failed to commit!"),
            Ok(_) => ()
        }
    }
    pub fn read(e: &Environment, h: &DbHandle, k: &str) -> String {
        let reader = e.get_reader().unwrap();
        let db = reader.bind(&h);
        let value = db.get::<&str>(&k).unwrap_or_else(|_| "");
        let r = String::from(value);
        {
            if r == utils::empty_string() {
                error!("Failed to read from db.")
            }
        }
        r
    }
    pub fn delete(e: &Environment, h: &DbHandle, k: &str) {
        let txn = e.new_transaction().unwrap();
        {
            // get a database bound to this transaction
            let db = txn.bind(&h); 
            db.del::<>(&k).unwrap_or_else(|_| error!("failed to delete"));
        }
        match txn.commit() {
            Err(_) => error!("failed to commit!"),
            Ok(_) => ()
        }
    }
}
