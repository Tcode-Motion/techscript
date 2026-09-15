use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let s = String::from("warning");
    let target = "warning";

    c.bench_function("contains", |b| {
        b.iter(|| std::hint::black_box(&s).contains(std::hint::black_box(target)))
    });

    c.bench_function("eq", |b| {
        b.iter(|| std::hint::black_box(&s) == std::hint::black_box(target))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
