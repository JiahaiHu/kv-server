extern crate grpcio;
extern crate protobuf;

mod protos;
mod store;

use std::collections::HashMap;
use std::env;
use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

use protos::kvserver::{Status, GetRequest, GetResponse, PutRequest, PutResponse, DeleteRequest, DeleteResponse, ScanRequest, ScanResponse};
use protos::kvserver_grpc::KvClient;
use store::{Key, Value};

struct Client {
    client: KvClient,
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

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Expected exactly one argument, the port to connect to.")
    }
    let port = args[1]
        .parse::<u16>()
        .expect(format!("{} is not a valid port number", args[1]).as_str());

    let client = Client::new(String::from("127.0.0.1"), port);
    // TODO: get input
}
