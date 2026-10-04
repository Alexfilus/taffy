#![cfg(all(feature = "taffy_tree", feature = "grid"))]

#[path = "fixtures/grid_replaced_sizing.rs"]
mod fixture;

#[test]
fn grid_replaced_sizing_regression() {
    fixture::verify();
}
