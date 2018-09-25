extern crate futures;
extern crate grpcio;
extern crate wt_rs;

use std::sync::{Arc, Mutex};
use std::thread;

use futures::stream;
use futures::{Future, Sink};

use grpcio::{Environment, RpcContext, ServerBuilder, ServerStreamingSink, UnarySink, WriteFlags};

use wt_rs::proto::kv::{GetRequest, Item, PutRequest, PutResponse, ScanRequest};
use wt_rs::proto::kv_grpc::{self, KvService};
use wt_rs::wt;

#[derive(Clone)]
struct KvServiceImpl {
    db: Arc<Mutex<wt::Connection>>,
}

impl KvService for KvServiceImpl {
    fn put(&self, ctx: RpcContext, req: PutRequest, sink: UnarySink<PutResponse>) {
        println!(
            "putting: {}={}",
            req.get_item().get_key(),
            req.get_item().get_value()
        );
        let mut db = self.db.lock().expect("lock db");
        let mut session = wt::Session::open(&mut db).expect("open session");
        session.create_table("kv").expect("create table");
        let mut cursor = wt::Cursor::open(&mut session, "kv").expect("open cursor");
        cursor
            .put(req.get_item().get_key(), req.get_item().get_value())
            .expect("put kv");
        let resp = PutResponse::new();
        let f = sink
            .success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f);
    }
    fn get(&self, ctx: RpcContext, req: GetRequest, sink: UnarySink<Item>) {
        println!("getting: {}", req.get_key());
        let mut db = self.db.lock().expect("lock db");
        let mut session = wt::Session::open(&mut db).expect("open session");
        session.create_table("kv").expect("create table");
        let mut cursor = wt::Cursor::open(&mut session, "kv").expect("open cursor");
        let mut resp = Item::new();
        match cursor.search(req.get_key()) {
            Ok(()) => {
                let (k, v) = cursor.get().expect("read cursor");
                resp.set_key(k);
                resp.set_value(v);
            }
            Err(_) => {
                println!("element not found");
            }
        };
        let f = sink
            .success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f);
    }

    fn scan(&self, ctx: RpcContext, req: ScanRequest, sink: ServerStreamingSink<Item>) {
        println!("scanning: {}", req.get_prefix());
        let mut db = self.db.lock().expect("lock db");
        let mut session = wt::Session::open(&mut db).expect("open session");
        session.create_table("kv").expect("create table");
        let mut cursor = wt::Cursor::open(&mut session, "kv").expect("open cursor");
        let pos = cursor.search_near(req.get_prefix()).expect("place cursor");
        if pos < 0 {
            cursor.advance().expect("advance cursor");
        }
        let mut items = Vec::new();
        loop {
            let (k, v) = cursor.get().expect("read cursor");
            println!("retrieved: {} = {}", k, v);
            if !k.starts_with(req.get_prefix()) {
                println!("too far, break");
                break;
            }
            let mut item = Item::new();
            item.set_key(k);
            item.set_value(v);
            items.push((item, WriteFlags::default()));
            if let Ok(true) = cursor.advance() {
                continue;
            } else {
                break;
            }
        }

        let f = sink
            .send_all(stream::iter_ok::<_, grpcio::Error>(items))
            .map(|_| ())
            .map_err(|e| println!("failed to handle scan: {:?}", e));

        ctx.spawn(f);
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
