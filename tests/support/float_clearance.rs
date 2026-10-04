use taffy::prelude::*;

pub fn scenario() -> Vec<f32> {
    use taffy::{Clear, FloatContext, FloatDirection};
    let mut c = FloatContext::new();
    c.set_width(1000.0);
    for (width, height, side) in
        [(241.0, 100.0, FloatDirection::Left), (434.0, 73.0, FloatDirection::Left), (80.0, 73.0, FloatDirection::Right)]
    {
        c.place_floated_box(Size { width, height }, 0.0, [0.0, 0.0], side, Clear::None);
    }
    vec![
        c.cleared_threshold(Clear::Left).unwrap(),
        c.cleared_threshold(Clear::Right).unwrap(),
        c.cleared_threshold(Clear::Both).unwrap(),
    ]
}

pub fn verify() {
    let actual = scenario();
    let expected = vec![100.0, 73.0, 100.0];
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.into_iter().zip(expected) {
        assert!((actual - expected).abs() < 0.02, "actual={actual}, expected={expected}");
    }
}
