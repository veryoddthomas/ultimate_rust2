use criterion::{black_box, criterion_group, criterion_main, Criterion};
use testing::*;

fn bench_sploosh(c: &mut Criterion) {
    c.bench_function("sploosh", |b| {
        b.iter(|| sploosh(black_box(8), black_box(9), black_box(10)))
    });
}
criterion_group!(benches, bench_sploosh);
criterion_main!(benches);

// Challenge: Create a benchmark that measures the speed of sploosh(8, 9, 10)
// - Speed up the implementation of sploosh(8, 9, 10) without breaking the other tests.
// - Hint: See Cargo.toml to get you started
