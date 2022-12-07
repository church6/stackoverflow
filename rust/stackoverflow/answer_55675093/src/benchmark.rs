use criterion::{criterion_group, criterion_main, Criterion}; // 0.4.0
use rle::*;// 0.2.0

fn criterion_benchmark(c: &mut Criterion) {
    let data = rand_data(4 * 1024 * 1024);

    c.bench_function("encode (procedural)", {
        let data = data.clone();
        move |b| b.iter(|| encode_proc(&data))
    });

    c.bench_function("encode (functional)", {
        let data = data.clone();
        move |b| b.iter(|| encode_iter(&data))
    });

    c.bench_function("encode (fast)", {
        let data = data.clone();
        move |b| b.iter(|| encode_slim(&data))
    });

    c.bench_function("encode (tiny)", {
        let data = data.clone();
        move |b| b.iter(|| encode_tiny(&data))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

