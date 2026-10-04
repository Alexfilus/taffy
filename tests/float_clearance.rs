#[path = "support/float_clearance.rs"]
mod support;

#[test]
fn regression() {
    support::verify();
}
