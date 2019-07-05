extern crate futures;
extern crate grpcio;
extern crate protobuf;

use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;    // trait for UnarySinkResult, oneshot::Receiver, oneshot::Sender
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

mod protos;

use protos::kvserver::{HelloRequest, HelloReply};
use protos::kvserver_grpc::{self, Greeter};

#[derive(Clone)]
struct HelloService;

impl Greeter for HelloService {
    fn hello(&mut self, ctx: RpcContext, req: HelloRequest, sink: UnarySink<HelloReply>) {
        let mut response = HelloReply::new();
        println!("Received GetRequest {{ {:?} }}", req);
        response.set_message(format!("hello, {}!", req.get_name()));
        let f = sink.success(response.clone())
            .map(move |_| println!("Responded with  {{ {:?} }}", response))
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err));
        ctx.spawn(f)
    }
}

fn main () {
    let env = Arc::new(Environment::new(1));
    let service = kvserver_grpc::create_greeter(HelloService);  // trait Clone required
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 0)
        .build()
        .unwrap();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    server.start();
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
