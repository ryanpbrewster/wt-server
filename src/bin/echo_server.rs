extern crate futures;
extern crate grpcio;
extern crate wt_rs;

use std::sync::Arc;
use std::thread;

use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use wt_rs::proto::echo::{EchoRequest, EchoResponse};
use wt_rs::proto::echo_grpc::{self, EchoService};

#[derive(Clone)]
struct EchoServiceImpl;

impl EchoService for EchoServiceImpl {
    fn echo(&self, ctx: RpcContext, req: EchoRequest, sink: UnarySink<EchoResponse>) {
        println!("echoing: {}", req.get_message());
        let mut resp = EchoResponse::new();
        resp.set_message(format!("echoing: {}", req.get_message()));
        let f = sink
            .success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }
}

fn main() {
    let env = Arc::new(Environment::new(1));
    let service = echo_grpc::create_echo_service(EchoServiceImpl);
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
