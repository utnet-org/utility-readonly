use criterion::{criterion_group, criterion_main, Criterion};
use utility::{open_leveldb, put_data};  // Replace with your actual project name and import statements

fn write_data_benchmark(c: &mut Criterion) {
    let db_path="./my_db";
    let db = unsafe { open_leveldb(db_path) };
    let mut group = c.benchmark_group("write_data_benchmark");

    for i in 0..10000 {
        group.bench_function(format!("write_data_{}", i), |b| {
            b.iter(|| {
                let key = format!("key{}", i);
                let value = format!("value{}", i);
                put_data(db, &key, &value).unwrap();
            });
        });
    }

    group.finish();
}


criterion_group!(benches, write_data_benchmark);
criterion_main!(benches);