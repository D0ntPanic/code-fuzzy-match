use code_fuzzy_match::fuzzy_match;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("early_match", |b| {
        b.iter(|| {
            fuzzy_match(
                black_box("The quick brown fox jumps over the lazy dog."),
                black_box("the"),
            )
        })
    });
    c.bench_function("late_match", |b| {
        b.iter(|| {
            fuzzy_match(
                black_box("The quick brown fox jumps over the lazy dog."),
                black_box("dog"),
            )
        })
    });
    c.bench_function("no_match", |b| {
        b.iter(|| {
            fuzzy_match(
                black_box("The quick brown fox jumps over the lazy dog."),
                black_box("cat"),
            )
        })
    });
    c.bench_function("long_matching_query", |b| {
        b.iter(|| {
            fuzzy_match(
                black_box("The quick brown fox jumps over the lazy dog."),
                black_box("Quick fox jumps the dog"),
            )
        })
    });
    c.bench_function("long_no_match_query", |b| {
        b.iter(|| {
            fuzzy_match(
                black_box("The quick brown fox jumps over the lazy dog."),
                black_box("Quick fox jumps the cat"),
            )
        })
    });
    c.bench_function("long_early_exit_query", |b| {
        b.iter(|| {
            fuzzy_match(
                black_box("The quick brown fox jumps over the lazy dog."),
                black_box("cat jumps the quick fox"),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
