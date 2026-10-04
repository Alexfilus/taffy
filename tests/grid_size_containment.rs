#![cfg(all(feature = "taffy_tree", feature = "grid"))]

#[path = "fixtures/grid_size_containment.rs"]
mod fixture;

#[test]
fn grid_size_containment_regression() {
    fixture::verify();
}
