extern crate grpcio;
extern crate protobuf;

use std::env;
use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

mod protos;

use protos::kvserver::{Status, GetRequest, GetResponse, PutRequest, PutResponse, DeleteRequest, DeleteResponse, ScanRequest, ScanResponse};
use protos::kvserver_grpc::KvClient;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Expected exactly one argument, the port to connect to.")
    }
    let port = args[1]
        .parse::<u16>()
        .expect(format!("{} is not a valid port number", args[1]).as_str());

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(format!("localhost:{}", port).as_str());
    let client = KvClient::new(ch);

    // test put
    let mut req = PutRequest::new();
    req.set_key(String::from("A"));
    req.set_value(String::from("Alex"));
    let response = client.put(&req).expect("grpc: put failed!");
    println!("Received PutResponse {{ {:?} }}", response);
    match response.status {
            Status::success => println!("put: success"),
            _ => println!("put: failed"),
    }
    
    // test get
    let mut req = GetRequest::new();
    req.set_key(String::from("A"));
    let response = client.get(&req).expect("grpc: get failed!");
    println!("Received GetResponse {{ {:?} }}", response);
    match response.status {
            Status::success => println!("get: {}", response.value),
            _ => println!("get: failed"),
    }
}
