#![cfg(all(feature = "taffy_tree", feature = "calc", feature = "block_layout"))]

#[path = "fixtures/calc_resolver.rs"]
mod fixture;

#[test]
fn calc_resolver_regression() {
    fixture::verify();
}
