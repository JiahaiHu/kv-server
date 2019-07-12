pub mod log;

use std::collections::{BTreeMap, HashMap};
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::Arc;
use std::sync::Mutex;

use super::{Engine, Key, Value};
use log::Log;

const LOG_PATH: &'static str = "kv.log";

#[derive(Clone)]
pub struct Kvdb {
    pub db: Arc<Mutex<BTreeMap<Key, Value>>>,
    log: Arc<Log>,
}

impl Kvdb {
    pub fn new() -> Self {
        let mut kvdb = Kvdb {
            db: Arc::new(Mutex::new(BTreeMap::new())),
            log: Arc::new(Log::new(LOG_PATH)),
        };
        kvdb.recover();
        kvdb
    }

    pub fn recover(&mut self) {
        println!("recovering...");
        let file = OpenOptions::new().read(true).open(&self.log.path);
        match file {
            Err(_) => println!("log file not found"),
            Ok(f) => {
                let reader = BufReader::new(f);
                let iter = reader.lines().map(|l| l.unwrap());;  // BufRead Trait
                let mut map = self.db.lock().unwrap();
                for line in iter {
                    let v: Vec<&str> = line.split_whitespace().collect();
                    let log_type = v[0];
                    let key = v[1];
                    let value = v[2];
                    match log_type {
                        "0" => map.insert(key.to_owned(), value.to_owned()),
                        "1" => map.remove(key),
                        _ => None,
                    };
                }
            }
        }
        println!("recovery finished!");        
    }
}

impl Engine for Kvdb {
    fn get(&self, key: &Key) -> Result<Option<Value>, ()> {
        let map = self.db.lock().unwrap();
        let ret = map.get(key);
        match ret {
            Some(value) => Ok(Some(value.clone())),
            None => Ok(None),
        }
    }

    /// If the map did not have this key present, Ok(None) is returned.
    /// If the map did have this key present, the value is updated, and OK(Some(old_value)) is returned.
    fn put(&mut self, key: &Key, value: &Value) -> Result<Option<Value>, ()> {
        let mut map = self.db.lock().unwrap();
        let ret = map.insert(key.clone(), value.clone());
        match ret {
            Some(value) => Ok(Some(value.clone())),
            None => Ok(None),
        }
    }

    /// Delete a key from the map, returning Ok(Some(value)) if the key was previously in the map.
    fn delete(&mut self, key: &Key) -> Result<Option<Value>, ()> {
        let mut map = self.db.lock().unwrap();
        let ret = map.remove(key);
        match ret {
            Some(value) => Ok(Some(value.clone())),
            None => Ok(None),
        }
    }

    fn scan(&self, key_start: &Key, key_end: &Key) -> Result<Option<HashMap<Key, Value>>, ()>  {
        let mut kvs = HashMap::new();
        let map = self.db.lock().unwrap();
        for (key, value) in map.range(key_start.clone()..key_end.clone()) {
            kvs.insert(key.clone(), value.clone());
        }
        if map.len() != 0 {
            Ok(Some(kvs))
        }
        else {
            Ok(None)
        }
    }
}
