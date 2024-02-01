use std::ffi::OsStr;

use criterion::{criterion_group, criterion_main, Criterion};
use headless_chrome::Browser;

fn create_connection() {
    let browser = Browser::new({
        headless_chrome::LaunchOptions { headless: false,
                                         sandbox: false,
                                         ignore_certificate_errors: true,
                                         ..Default::default() }
    });

    browser.unwrap()
           .get_tabs()
           .lock()
           .unwrap()
           .iter()
           .for_each(|e| {
               e.close(false).unwrap();
           })
}

fn create_connection_headless() {
    let browser = Browser::new({
        headless_chrome::LaunchOptions { headless: true,
                                         sandbox: false,
                                         ignore_certificate_errors: true,
                                         ..Default::default() }
    });

    browser.unwrap()
           .get_tabs()
           .lock()
           .unwrap()
           .iter()
           .for_each(|e| {
               e.close(false).unwrap();
           })
}

fn create_connection_with_sandbox() {
    let browser = Browser::new({
        headless_chrome::LaunchOptions { headless: false,
                                         sandbox: true,
                                         ignore_certificate_errors: true,
                                         ..Default::default() }
    });

    browser.unwrap()
           .get_tabs()
           .lock()
           .unwrap()
           .iter()
           .for_each(|e| {
               e.close(false).unwrap();
           })
}

fn create_connection_with_extensions() {
    let browser = Browser::new({
        headless_chrome::LaunchOptions { headless: false,
                                         sandbox: true,
                                         ignore_certificate_errors: true,
                                         extensions: vec![OsStr::new("./chrome-ext/adblock")],
                                         ..Default::default() }
    });

    browser.unwrap()
           .get_tabs()
           .lock()
           .unwrap()
           .iter()
           .for_each(|e| {
               e.close(false).unwrap();
           })
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("browser_open");
    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    group.significance_level(0.1)
         .sample_size(10)
         .measurement_time(std::time::Duration::from_secs(20));

    // 1.54 seconds
    group.bench_function("Create new browser basic", |b| b.iter(create_connection));

    // 0.943 seconds
    group.bench_function("Create new browser headless", |b| {
             b.iter(create_connection_headless)
         });

    // 1.48 seconds
    group.bench_function("Create new browser sandbox", |b| {
             b.iter(create_connection_with_sandbox)
         });

    // 1.69 seconds
    group.bench_function("Create new browser adblock ext", |b| {
             b.iter(create_connection_with_extensions)
         });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
