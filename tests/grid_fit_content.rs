#![cfg(all(feature = "taffy_tree", feature = "grid"))]

#[path = "fixtures/grid_fit_content.rs"]
mod fixture;

#[test]
fn grid_fit_content_regression() {
    fixture::verify();
}
