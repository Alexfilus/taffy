#[path = "support/intrinsic_collapsed_margins.rs"]
mod support;

#[test]
fn regression() {
    support::verify();
}
