use code_fuzzy_match::{fuzzy_match, FuzzyMatcher};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("single_early_match", |b| {
        b.iter(|| {
            fuzzy_match(
                black_box("The quick brown fox jumps over the lazy dog."),
                black_box("the"),
            )
        })
    });
    c.bench_function("single_late_match", |b| {
        b.iter(|| {
            fuzzy_match(
                black_box("The quick brown fox jumps over the lazy dog."),
                black_box("dog"),
            )
        })
    });
    c.bench_function("single_no_match", |b| {
        b.iter(|| {
            fuzzy_match(
                black_box("The quick brown fox jumps over the lazy dog."),
                black_box("cat"),
            )
        })
    });
    c.bench_function("single_long_matching_query", |b| {
        b.iter(|| {
            fuzzy_match(
                black_box("The quick brown fox jumps over the lazy dog."),
                black_box("Quick fox jumps the dog"),
            )
        })
    });
    c.bench_function("single_long_no_match_query", |b| {
        b.iter(|| {
            fuzzy_match(
                black_box("The quick brown fox jumps over the lazy dog."),
                black_box("Quick fox jumps the cat"),
            )
        })
    });
    c.bench_function("single_long_early_exit_query", |b| {
        b.iter(|| {
            fuzzy_match(
                black_box("The quick brown fox jumps over the lazy dog."),
                black_box("cat jumps the quick fox"),
            )
        })
    });

    c.bench_function("batch_early_match", |b| {
        let mut matcher = FuzzyMatcher::new();
        b.iter(|| {
            matcher.fuzzy_match(
                black_box("The quick brown fox jumps over the lazy dog."),
                black_box("the"),
            )
        })
    });
    c.bench_function("batch_late_match", |b| {
        let mut matcher = FuzzyMatcher::new();
        b.iter(|| {
            matcher.fuzzy_match(
                black_box("The quick brown fox jumps over the lazy dog."),
                black_box("dog"),
            )
        })
    });
    c.bench_function("batch_no_match", |b| {
        let mut matcher = FuzzyMatcher::new();
        b.iter(|| {
            matcher.fuzzy_match(
                black_box("The quick brown fox jumps over the lazy dog."),
                black_box("cat"),
            )
        })
    });
    c.bench_function("batch_long_matching_query", |b| {
        let mut matcher = FuzzyMatcher::new();
        b.iter(|| {
            matcher.fuzzy_match(
                black_box("The quick brown fox jumps over the lazy dog."),
                black_box("Quick fox jumps the dog"),
            )
        })
    });
    c.bench_function("batch_long_no_match_query", |b| {
        let mut matcher = FuzzyMatcher::new();
        b.iter(|| {
            matcher.fuzzy_match(
                black_box("The quick brown fox jumps over the lazy dog."),
                black_box("Quick fox jumps the cat"),
            )
        })
    });
    c.bench_function("batch_long_early_exit_query", |b| {
        let mut matcher = FuzzyMatcher::new();
        b.iter(|| {
            matcher.fuzzy_match(
                black_box("The quick brown fox jumps over the lazy dog."),
                black_box("cat jumps the quick fox"),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
