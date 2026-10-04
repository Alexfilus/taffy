# Intrinsic collapsed margins

Preserve collapsed-margin metadata during intrinsic measurements and distinguish margin-collapse contexts in the measurement cache.

## Provenance and dependencies

- Upstream version: `1c9e37a0a1cf54c8ca020db111caca1ecf4aae86 (Taffy 0.12.1)`.
- Obscura source commits: `839f1c2`.
- Prerequisite branches: None; independent branch from the version listed above.

## Reproduce

```sh
cargo nextest run --release -p taffy --lib --test intrinsic_collapsed_margins
cargo nextest run --release -p taffy --no-fail-fast
BENCH_ITERS=1000 cargo bench -p taffy --bench intrinsic_collapsed_margins
```

The standalone regression was also run against unpatched Taffy 0.12.1 and failed as expected.

## Validation

Local macOS arm64, Rust 1.98.1, release profile:

- Summary [   4.442s] 4570 tests run: 4570 passed, 4 skipped
- `intrinsic-collapsed-margins: median=3381.67 ns/op min=3337.08 max=6011.67 samples=7 iterations=100 (includes fixture construction)`

The benchmark reports seven samples after warm-up. The sample range is included;
these absolute timings are not a before/after speedup claim. Taffy scenarios include
fixture construction. Cosmic-text scenarios reuse a warm FontSystem.
Use a separate target directory per worktree, or clean this package between revisions,
to prevent stale same-version artifacts from another checkout.
