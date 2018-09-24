#[macro_use]
extern crate criterion;
use criterion::Criterion;

extern crate wt_rs;
use wt_rs::wt;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("wt-insert", |b| {
        let mut conn = wt::Connection::open("data").expect("open connection");

        let mut session = wt::Session::open(&mut conn).expect("open session");
        session.create_table("rpb").expect("create table");

        let mut cursor = wt::Cursor::open(&mut session, "rpb").expect("open cursor");

        let mut idx = 0;
        b.iter(|| {
            idx += 1;
            cursor
                .put(&format!("rpb-{}", idx), &format!("bench-{}", idx))
                .expect("write cursor");
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
