use criterion::{black_box, criterion_group, criterion_main, Criterion};
use techscript_ast::{Expression, LiteralVal, Pattern, Program, Statement};
use techscript_formatter::{DocumentFormatter, Formatter};

fn bench_format(c: &mut Criterion) {
    let source = "
    make var_0 = 0
    say \"Hello 0\"
    make var_1 = 1
    say \"Hello 1\"
    "
    .repeat(2500); // 10000 statements

    let formatter = DocumentFormatter::new(4);

    c.bench_function("format 10000 statements", |b| {
        b.iter(|| formatter.format_source(black_box(&source)))
    });
}

criterion_group!(benches, bench_format);
criterion_main!(benches);
