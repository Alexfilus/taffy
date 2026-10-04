use taffy::prelude::*;

fn tree() -> TaffyTree<()> {
    let mut tree = TaffyTree::new();
    tree.disable_rounding();
    tree
}

pub fn scenario() -> Vec<f32> {
    let mut t = tree();
    let item = t.new_leaf(Style::default()).unwrap();
    let second = t.new_leaf(Style::default()).unwrap();
    let root = t
        .new_with_children(
            Style {
                display: Display::Grid,
                size: Size { width: length(2000.0_f32), height: auto() },
                grid_template_columns: vec![
                    minmax(length(173.68802_f32), length(993.3_f32)),
                    minmax(length(0.0_f32), length(3000.0_f32)),
                ],
                ..Style::default()
            },
            &[item, second],
        )
        .unwrap();
    t.compute_layout(root, Size::MAX_CONTENT).unwrap();
    vec![t.layout(item).unwrap().size.width, t.layout(second).unwrap().size.width]
}

pub fn verify() {
    let actual = scenario();
    let expected = vec![993.3, 1006.7];
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.into_iter().zip(expected) {
        assert!((actual - expected).abs() < 0.02, "actual={actual}, expected={expected}");
    }
}
