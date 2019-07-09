extern crate grpcio;
extern crate protobuf;

use std::collections::HashMap;
use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

use crate::protos::kvserver::{Status, GetRequest, PutRequest, DeleteRequest, ScanRequest};
use crate::protos::kvserver_grpc::KvClient;
use crate::store::{Key, Value};

pub struct Client {
    pub client: KvClient,
}

impl Client {
    pub fn new (host: String, port: u16) -> Self {
        let addr = format!("{}:{}", host, port);
        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect(addr.as_str());
        let client = KvClient::new(ch);

        Client {
            client,
        }
    }

    pub fn get(&self, key: Key) -> Option<Value> {
        let mut req = GetRequest::new();
        req.set_key(key);
        let response = self.client.get(&req).expect("get: grpc failed!");
        println!("Received GetResponse {{ {:?} }}", response);
        match response.status {
            Status::success => Some(response.value),
            _ => None,
        }
    }

    pub fn put(&self, key: Key, value: Value) {
        let mut req = PutRequest::new();
        req.set_key(key);
        req.set_value(value);
        let response = self.client.put(&req).expect("put: grpc failed!");
        println!("Received PutResponse {{ {:?} }}", response);
    }

    pub fn delete(&self, key: Key) {
        let mut req = DeleteRequest::new();
        req.set_key(key);
        let response = self.client.delete(&req).expect("delete: grpc failed!");
        println!("Received DeleteResponse {{ {:?} }}", response);
    }

    pub fn scan(&self, key_start: Key, key_end: Key) -> Option<HashMap<Key, Value>> {
        let mut req = ScanRequest::new();
        req.set_key_start(key_start);
        req.set_key_end(key_end);
        let response = self.client.scan(&req).expect("scan: grpc failed!");
        println!("Received ScanResponse {{ {:?} }}", response);
        match response.status {
            Status::success => Some(response.kvs),
            _ => None,
        }
    }
}
