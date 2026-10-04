#[path = "support/width_only_cache.rs"]
mod support;

#[test]
fn regression() {
    support::verify();
}
