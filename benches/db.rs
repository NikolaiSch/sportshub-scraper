use criterion::{black_box, criterion_group, criterion_main, Criterion};
use scraper::db;

fn create_connection() {
    db::establish_connection();
}

fn criterion_benchmark(c: &mut Criterion) {
    // 43.6 nano seconds
    c.bench_function("Create new db connection", |b| {
        b.iter(|| create_connection())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
