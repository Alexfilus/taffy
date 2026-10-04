# Width only cache

Prevent width-only measurements with placeholder heights from satisfying vertical or both-axis cache queries.

## Provenance and dependencies

- Upstream version: `1c9e37a0a1cf54c8ca020db111caca1ecf4aae86 (Taffy 0.12.1)`.
- Obscura source commits: `5fe47ef`.
- Prerequisite branches: `feature-intrinsic-collapsed-margins`

## Reproduce

```sh
cargo nextest run --release -p taffy --lib --test width_only_cache
cargo nextest run --release -p taffy --no-fail-fast
BENCH_ITERS=1000 cargo bench -p taffy --bench width_only_cache
```

The standalone regression was also run against unpatched Taffy 0.12.1 and failed as expected.

## Validation

Local macOS arm64, Rust 1.98.1, release profile:

- Summary [   5.303s] 4571 tests run: 4571 passed, 4 skipped
- `width-only-cache: median=23.75 ns/op min=23.75 max=43.33 samples=7 iterations=100 (includes fixture construction)`

The benchmark reports seven samples after warm-up. The sample range is included;
these absolute timings are not a before/after speedup claim. Taffy scenarios include
fixture construction. Cosmic-text scenarios reuse a warm FontSystem.
Use a separate target directory per worktree, or clean this package between revisions,
to prevent stale same-version artifacts from another checkout.
