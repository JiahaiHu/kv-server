extern crate grpcio;
extern crate protobuf;

use std::env;
use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

mod protos;

use protos::kvserver::HelloRequest;
use protos::kvserver_grpc::GreeterClient;

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
    let client = GreeterClient::new(ch);

    let mut req = HelloRequest::new();
    req.set_name(String::from("Alex"));
    let response = client.hello(&req);
    println!("{:?}", response);
}
