use criterion::{criterion_group, criterion_main, Criterion, ParameterizedBenchmark};
use your_project_name::write_and_read_leveldb;  // Replace with your actual project name

fn bench_write_and_read(c: &mut Criterion) {
    let sizes = vec![10, 100, 1000];  // Different data sizes

    let bench = ParameterizedBenchmark::new(
        "write_and_read",
        |b, &size| {
            b.iter_with_large_drop(|| write_and_read_leveldb(size));
        },
        sizes,
    );

    c.bench("leveldb_bench", bench);
}

criterion_group!(benches, bench_write_and_read);
criterion_main!(benches);