
use criterion::{criterion_group, criterion_main, Criterion};
use your_rust_project::write_and_read_leveldb;

fn bench_write_and_read(c: &mut Criterion) {
    c.bench_function("write_and_read", |b| b.iter(|| write_and_read_leveldb()));
}

criterion_group!(benches, bench_write_and_read);
criterion_main!(benches);