use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use techscript_package_manager::CapabilityValidator;

fn benchmark_validate_elevation(c: &mut Criterion) {
    let mut group = c.benchmark_group("capability_validation");

    let root_caps: Vec<String> = (0..100).map(|i| format!("cap_{}", i)).collect();
    let dep_caps: Vec<String> = (50..150).map(|i| format!("cap_{}", i)).collect();
    let allowed_elevations: Vec<String> = (0..100).map(|i| format!("dep_{}", i)).collect();

    let dep_name = "dep_50"; // Will be found in allowed_elevations

    group.bench_function("validate_elevation", |b| {
        b.iter(|| {
            let _ = CapabilityValidator::validate_elevation(
                black_box(&root_caps),
                black_box(&dep_caps),
                black_box(&allowed_elevations),
                black_box(dep_name),
            );
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark_validate_elevation);
criterion_main!(benches);
