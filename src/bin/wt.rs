extern crate wt_rs;

use std::time::SystemTime;

use wt_rs::wt;

fn main() {
    let mut conn = wt::Connection::open("data").expect("open connection");

    let mut session = wt::Session::open(&mut conn).expect("open session");
    session.create_table("rpb").expect("create table");

    let mut cursor = wt::Cursor::open(&mut session, "rpb").expect("open cursor");

    let t = SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs() % 10_000;
    cursor
        .put(&format!("rpb-{}", t), &format!("rpb-{}", t * t))
        .expect("write cursor");

    cursor.reset().expect("reset cursor");

    while let Ok(true) = cursor.advance() {
        let (key, value) = cursor.get().expect("read cursor");
        println!("{:?} = {:?}", key, value);
    }
}
