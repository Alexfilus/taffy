use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

#[path = "../../tests/fixtures/grid_size_containment.rs"]
mod fixture;

fn benchmark(c: &mut Criterion) {
    fixture::verify();
    c.bench_function("grid-size-containment/build-and-layout", |b| b.iter(|| black_box(fixture::scenario())));
}

criterion_group! {
    name = benches;
    config = taffy_benchmarks::criterion_config();
    targets = benchmark
}
criterion_main!(benches);
