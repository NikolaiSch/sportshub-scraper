use criterion::{criterion_group, criterion_main, Criterion};
use scraper::db;

fn create_connection() {
    db::helpers::establish_connection().unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    // 43.6 nano seconds
    c.bench_function("Create new db connection", |b| b.iter(create_connection));

    // 228.3 nano seconds
    c.bench_function("Create a new record", |b| {
        let mut conn = db::helpers::establish_connection().unwrap();
        let new_stream = db::models::StreamNew {
            away: "Away",
            home: "Home",
            league: "League",
            country: "Country",
            start_time: "Start Time",
            url: "Url",
            stream_link: "https://www.test.com",
        };
        b.iter(|| db::helpers::create_stream(&mut conn, &new_stream))
    });

    // 356 nano seconds
    c.bench_function("Get all streams", |b| {
        let mut conn = db::helpers::establish_connection().unwrap();
        b.iter(|| db::helpers::get_streams(&mut conn))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
