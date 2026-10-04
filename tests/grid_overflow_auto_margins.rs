#![cfg(all(feature = "taffy_tree", feature = "grid"))]

#[path = "fixtures/grid_overflow_auto_margins.rs"]
mod fixture;

#[test]
fn grid_overflow_auto_margins_regression() {
    fixture::verify();
}
