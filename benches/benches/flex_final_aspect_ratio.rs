use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

#[path = "../../tests/fixtures/flex_final_aspect_ratio.rs"]
mod fixture;

fn benchmark(c: &mut Criterion) {
    fixture::verify();
    c.bench_function("flex-final-aspect-ratio/build-and-layout", |b| b.iter(|| black_box(fixture::scenario())));
}

criterion_group! {
    name = benches;
    config = taffy_benchmarks::criterion_config();
    targets = benchmark
}
criterion_main!(benches);
