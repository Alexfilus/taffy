use taffy::prelude::*;
use taffy::Rect;

fn tree() -> TaffyTree<()> {
    let mut tree = TaffyTree::new();
    tree.disable_rounding();
    tree
}

pub fn scenario() -> Vec<f32> {
    let mut result = Vec::new();
    for direction in [taffy::Direction::Ltr, taffy::Direction::Rtl] {
        let mut t = tree();
        let item = t
            .new_leaf(Style {
                size: Size { width: length(600.0_f32), height: length(20.0_f32) },
                margin: Rect { left: auto(), right: auto(), ..Rect::zero() },
                justify_self: Some(AlignSelf::CENTER),
                ..Style::default()
            })
            .unwrap();
        let root = t
            .new_with_children(
                Style {
                    display: Display::Grid,
                    direction,
                    size: Size { width: length(300.0_f32), height: auto() },
                    grid_template_columns: vec![length(300.0_f32)],
                    ..Style::default()
                },
                &[item],
            )
            .unwrap();
        t.compute_layout(root, Size::MAX_CONTENT).unwrap();
        result.push(t.layout(item).unwrap().location.x);
    }
    result
}

pub fn verify() {
    let actual = scenario();
    let expected = vec![0.0, -300.0];
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.into_iter().zip(expected) {
        assert!((actual - expected).abs() < 0.02, "actual={actual}, expected={expected}");
    }
}
