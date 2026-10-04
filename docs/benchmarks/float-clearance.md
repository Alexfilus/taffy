# Float clearance

Keep geometric float endpoints and source-order tops stable when later floats subdivide existing segments.

## Provenance and dependencies

- Upstream version: `1c9e37a0a1cf54c8ca020db111caca1ecf4aae86 (Taffy 0.12.1)`.
- Obscura source commits: `8385f49`, `79dba60`.
- Prerequisite branches: None; independent branch from the version listed above.

## Reproduce

```sh
cargo nextest run --release -p taffy --lib --test float_clearance
cargo nextest run --release -p taffy --no-fail-fast
BENCH_ITERS=1000 cargo bench -p taffy --bench float_clearance
```

The standalone regression was also run against unpatched Taffy 0.12.1 and failed as expected.

## Validation

Local macOS arm64, Rust 1.98.1, release profile:

- Summary [   4.295s] 4574 tests run: 4574 passed, 4 skipped
- `float-clearance: median=101.67 ns/op min=98.75 max=145.84 samples=7 iterations=100 (includes fixture construction)`

The benchmark reports seven samples after warm-up. The sample range is included;
these absolute timings are not a before/after speedup claim. Taffy scenarios include
fixture construction. Cosmic-text scenarios reuse a warm FontSystem.
Use a separate target directory per worktree, or clean this package between revisions,
to prevent stale same-version artifacts from another checkout.
