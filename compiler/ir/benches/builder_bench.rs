use criterion::{black_box, criterion_group, criterion_main, Criterion};
use techscript_ir::builder::IRBuilder;
use techscript_ir::types::IRType;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("seal_function 10000 blocks", |b| {
        b.iter(|| {
            let mut builder = IRBuilder::new();
            builder.start_function("bench_func".to_string(), IRType::Void);

            let num_blocks = 10000;
            let mut blocks = vec![];
            for i in 0..num_blocks {
                let block_id = builder.new_block(format!("block_{}", i));
                blocks.push(block_id);
            }

            for i in 0..(num_blocks - 1) {
                builder.enter_block(blocks[i], format!("block_{}", i));
                builder.emit_terminator(
                    techscript_ir::instruction::TerminatorKind::Jump(blocks[i + 1]),
                    techscript_common::Span { start: 0, end: 0 },
                );
            }

            builder.enter_block(blocks[num_blocks - 1], format!("block_{}", num_blocks - 1));
            builder.emit_terminator(
                techscript_ir::instruction::TerminatorKind::Return(None),
                techscript_common::Span { start: 0, end: 0 },
            );

            builder.seal_function();
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
