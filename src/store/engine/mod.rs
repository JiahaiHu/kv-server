use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use std::sync::Mutex;

use super::{Engine, Key, Value};

#[derive(Clone)]
pub struct Kvdb {
    pub db: Arc<Mutex<BTreeMap<Key, Value>>>,
}

impl Kvdb {
    pub fn new() -> Self {
        Kvdb {
            db: Arc::new(Mutex::new(BTreeMap::new())),
        }
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
