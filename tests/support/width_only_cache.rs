use taffy::prelude::*;

pub fn scenario() -> Vec<f32> {
    use taffy::{Cache, LayoutInput, LayoutOutput, Line, RequestedAxis, RunMode, SizingMode};
    let mut c = Cache::new();
    let mut input = LayoutInput {
        run_mode: RunMode::ComputeSize,
        sizing_mode: SizingMode::InherentSize,
        axis: RequestedAxis::Horizontal,
        known_dimensions: Size { width: Some(100.0), height: None },
        parent_size: Size { width: Some(100.0), height: None },
        available_space: Size::MIN_CONTENT,
        vertical_margins_are_collapsible: Line::FALSE,
    };
    c.store(&input, LayoutOutput::from_outer_size(Size { width: 100.0, height: 0.0 }));
    let horizontal = c.get(&input).is_some();
    input.axis = RequestedAxis::Vertical;
    let vertical = c.get(&input).is_some();
    vec![u8::from(horizontal) as f32, u8::from(vertical) as f32]
}

pub fn verify() {
    let actual = scenario();
    let expected = vec![1.0, 0.0];
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.into_iter().zip(expected) {
        assert!((actual - expected).abs() < 0.02, "actual={actual}, expected={expected}");
    }
}
