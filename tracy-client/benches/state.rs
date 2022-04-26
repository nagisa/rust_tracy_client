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

fn bench_start_stop(c: &mut Criterion) {
    c.bench_function("start_stop", |b| b.iter(|| Client::start()));
}

fn bench_counting(c: &mut Criterion) {
    let client = Client::start();
    c.bench_function("counting", |b| b.iter(|| Client::start()));
    drop(client);
}

criterion_group!(
    benches,
    bench_start,
    bench_clone,
    bench_start_stop,
    bench_counting,
);
criterion_main!(benches);
