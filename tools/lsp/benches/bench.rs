use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn find_boundary_slow(line: &str, word: &str) -> usize {
    let mut count = 0;
    let word_len = word.len();
    let mut start_pos = 0;
    while let Some(pos_in_line) = line[start_pos..].find(word) {
        let actual_pos = start_pos + pos_in_line;
        let char_before = if actual_pos > 0 {
            line.chars().nth(actual_pos - 1)
        } else {
            None
        };
        let char_after = line.chars().nth(actual_pos + word_len);

        let is_boundary_before = char_before.map_or(true, |c| !c.is_alphanumeric() && c != '_');
        let is_boundary_after = char_after.map_or(true, |c| !c.is_alphanumeric() && c != '_');

        if is_boundary_before && is_boundary_after {
            count += 1;
        }
        start_pos = actual_pos + word_len;
    }
    count
}

fn find_boundary_fast(line: &str, word: &str) -> usize {
    let mut count = 0;
    let word_len = word.len();
    let mut start_pos = 0;
    while let Some(pos_in_line) = line[start_pos..].find(word) {
        let actual_pos = start_pos + pos_in_line;
        let char_before = if actual_pos > 0 {
            line[..actual_pos].chars().next_back()
        } else {
            None
        };
        let char_after = line[actual_pos + word_len..].chars().next();

        let is_boundary_before = char_before.map_or(true, |c| !c.is_alphanumeric() && c != '_');
        let is_boundary_after = char_after.map_or(true, |c| !c.is_alphanumeric() && c != '_');

        if is_boundary_before && is_boundary_after {
            count += 1;
        }
        start_pos = actual_pos + word_len;
    }
    count
}

fn criterion_benchmark(c: &mut Criterion) {
    let long_line = "let abc = 123; ".repeat(1000);
    let word = "abc";

    c.bench_function("slow", |b| b.iter(|| find_boundary_slow(black_box(&long_line), black_box(word))));
    c.bench_function("fast", |b| b.iter(|| find_boundary_fast(black_box(&long_line), black_box(word))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
