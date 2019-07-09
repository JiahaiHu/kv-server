use std::collections::HashMap;

pub mod engine;

pub type Key = String;
pub type Value = String;

pub trait Engine {
    fn get(&self, key: &Key) -> Result<Option<Value>, ()>;
    fn put(&mut self, key: &Key, value: &Value) -> Result<Option<Value>, ()>;
    fn delete(&mut self, key: &Key) -> Result<Option<Value>, ()>;
    fn scan(&self, key_start: &Key, key_end: &Key) -> Result<Option<HashMap<Key, Value>>, ()>;
}
