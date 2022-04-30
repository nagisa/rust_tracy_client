use criterion::{criterion_group, criterion_main, Criterion};
use tracy_client::Client;

fn bench_start(c: &mut Criterion) {
    let mut clients = Vec::<Client>::with_capacity(1_000_000_000);
    c.bench_function("start", |b| b.iter(|| clients.push(Client::start())));
}

fn bench_clone(c: &mut Criterion) {
    let mut clients = Vec::<Client>::with_capacity(1_000_000_000);
    let client = Client::start();
    c.bench_function("clone", |b| b.iter(|| clients.push(client.clone())));
}

fn bench_running(c: &mut Criterion) {
    let _client = Client::start();
    c.bench_function("running", |b| b.iter(|| Client::running()));
}

criterion_group!(benches, bench_start, bench_clone, bench_running,);
criterion_main!(benches);
