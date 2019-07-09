extern crate futures;
extern crate grpcio;
extern crate protobuf;

use std::sync::Arc;

use futures::Future;    // trait for UnarySinkResult, oneshot::Receiver, oneshot::Sender
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink, Server as GrpcServer};

use crate::protos::kvserver::{Status, GetRequest, GetResponse, PutRequest, PutResponse, DeleteRequest, DeleteResponse, ScanRequest, ScanResponse};
use crate::protos::kvserver_grpc::{self, Kv};
use crate::store::Engine;
use crate::store::engine::Kvdb;

#[derive(Clone)]
struct KvService {
    kvdb: Kvdb,
}

impl KvService {
    pub fn new() -> Self {
        KvService {
            kvdb: Kvdb::new(),
        }
    }
}

impl Kv for KvService {
    fn get(&mut self, ctx: RpcContext, req: GetRequest, sink: UnarySink<GetResponse>) {
        let mut response = GetResponse::new();
        println!("Received GetRequest {{ {:?} }}", req);
        let ret = self.kvdb.get(&req.key);
        match ret {
            Ok(option) => match option {
                Some(value) => {
                    response.set_status(Status::success);
                    response.set_value(value);
                },
                None => response.set_status(Status::keyNotFound),
            },
            Err(_) => response.set_status(Status::failed),
        }
        let f = sink.success(response.clone())
            .map(move |_| println!("Responded with  {{ {:?} }}", response))
            .map_err(move |err| eprintln!("Failed to response: {:?}", err));
        ctx.spawn(f)
    }

    fn put(&mut self, ctx: RpcContext, req: PutRequest, sink: UnarySink<PutResponse>) {
        let mut response = PutResponse::new();
        println!("Received PutRequest {{ {:?} }}", req);
        let ret = self.kvdb.put(&req.key, &req.value);
        match ret {
            Ok(_) => response.set_status(Status::success),
            Err(_) => response.set_status(Status::failed),
        }
        let f = sink.success(response.clone())
            .map(move |_| println!("Responded with  {{ {:?} }}", response))
            .map_err(move |err| eprintln!("Failed to response: {:?}", err));
        ctx.spawn(f)
    }

    fn delete(&mut self, ctx: RpcContext, req: DeleteRequest, sink: UnarySink<DeleteResponse>) {
        let mut response = DeleteResponse::new();
        println!("Received DeleteRequest {{ {:?} }}", req);
        let ret = self.kvdb.delete(&req.key);
        match ret {
            Ok(option) => match option {
                Some(_) => {
                    response.set_status(Status::success);
                },
                None => response.set_status(Status::keyNotFound),
            },
            Err(_) => response.set_status(Status::failed),
        }
        let f = sink.success(response.clone())
            .map(move |_| println!("Responded with  {{ {:?} }}", response))
            .map_err(move |err| eprintln!("Failed to response: {:?}", err));
        ctx.spawn(f)
    }

    fn scan(&mut self, ctx: RpcContext, req: ScanRequest, sink: UnarySink<ScanResponse>) {
        let mut response = ScanResponse::new();
        println!("Received ScanRequest {{ {:?} }}", req);
        let ret = self.kvdb.scan(&req.key_start, &req.key_end);
        match ret {
            Ok(option) => match option {
                Some(value) => {
                    response.set_status(Status::success);
                    response.set_kvs(value);
                },
                None => response.set_status(Status::keyNotFound),
            },
            Err(_) => response.set_status(Status::failed),
        }
        let f = sink.success(response.clone())
            .map(move |_| println!("Responded with  {{ {:?} }}", response))
            .map_err(move |err| eprintln!("Failed to response: {:?}", err));
        ctx.spawn(f)
    }
}

pub struct Server {
    pub server: GrpcServer,
}

impl Server {
    pub fn new(host: String, port: u16) -> Self {
        let env = Arc::new(Environment::new(1));
        let service = kvserver_grpc::create_kv(KvService::new());
        let server = ServerBuilder::new(env)
            .register_service(service)
            .bind(host.as_ref(), port.clone()).build().unwrap();
    
        Server {
            server,
        }
    }

    pub fn start(&mut self) {
        self.server.start();
        for &(ref host, port) in self.server.bind_addrs() {
            println!("listening on {}:{}", host, port);
        }
    }

    pub fn stop(&mut self) {
        println!("stoping server...");
        self.server.shutdown();
    }
}
