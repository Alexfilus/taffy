#[path = "../tests/support/intrinsic_collapsed_margins.rs"]
mod support;

fn main() {
    support::verify();
    let iterations = std::env::var("BENCH_ITERS").ok().and_then(|s| s.parse::<u32>().ok()).unwrap_or(1000);
    assert!(iterations > 0);
    for _ in 0..100 {
        std::hint::black_box(support::scenario());
    }
    let mut samples = Vec::new();
    for _ in 0..7 {
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(support::scenario());
        }
        samples.push(start.elapsed().as_nanos() as f64 / f64::from(iterations));
    }
    samples.sort_by(f64::total_cmp);
    println!("intrinsic-collapsed-margins: median={} ns/op min={} max={} samples=7 iterations={iterations} (includes fixture construction)", samples[3], samples[0], samples[6]);
}
