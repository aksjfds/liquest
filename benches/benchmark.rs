#![allow(unused)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use liquest::request::{HttpRequest, Method};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| benchmark_request()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn benchmark_request() {
    for _ in (0..20) {
        let mut http_request = HttpRequest::build("https://www.baidu.com/index.html", Method::Get)
            .header("Accept", "text/html,application/xhtml+xml,application/json")
            .header("Connection", "close");
    }
}
