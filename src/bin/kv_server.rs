extern crate futures;
extern crate grpcio;
extern crate wt_rs;

use std::sync::{Arc, Mutex};
use std::thread;

use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use wt_rs::proto::kv::{GetRequest, GetResponse, PutRequest, PutResponse};
use wt_rs::proto::kv_grpc::{self, KvService};
use wt_rs::wt;

#[derive(Clone)]
struct KvServiceImpl {
    db: Arc<Mutex<wt::Connection>>,
}

impl KvService for KvServiceImpl {
    fn put(&self, ctx: RpcContext, req: PutRequest, sink: UnarySink<PutResponse>) {
        println!("putting: {}={}", req.get_key(), req.get_value());
        let resp = PutResponse::new();
        let f = sink
            .success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }
    fn get(&self, ctx: RpcContext, req: GetRequest, sink: UnarySink<GetResponse>) {
        println!("getting: {}", req.get_key());
        let mut resp = GetResponse::new();
        resp.set_key(req.get_key().to_string());
        resp.set_value(String::from("unimplemented"));
        let f = sink
            .success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }
}

fn main() {
    let db = wt::Connection::open("data").expect("open db");
    let env = Arc::new(Environment::new(1));
    let service = kv_grpc::create_kv_service(KvServiceImpl {
        db: Arc::new(Mutex::new(db)),
    });
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50_051)
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    thread::park();
    println!("shutting down...");
    server.shutdown().wait().expect("shutdown");
}
