#![cfg(all(feature = "taffy_tree", feature = "flexbox"))]

#[path = "fixtures/flex_final_aspect_ratio.rs"]
mod fixture;

#[test]
fn flex_final_aspect_ratio_regression() {
    fixture::verify();
}
