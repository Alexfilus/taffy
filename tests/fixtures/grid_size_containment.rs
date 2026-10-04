use taffy::prelude::*;

fn tree() -> TaffyTree<()> {
    let mut tree = TaffyTree::new();
    tree.disable_rounding();
    tree
}

pub fn scenario() -> Vec<f32> {
    let mut t = tree();
    let child = t
        .new_leaf(Style { size: Size { width: length(240.0_f32), height: length(20.0_f32) }, ..Style::default() })
        .unwrap();
    let item = t
        .new_with_children(
            Style {
                display: Display::Block,
                intrinsic_size_containment: Size { width: true, height: false },
                ..Style::default()
            },
            &[child],
        )
        .unwrap();
    let fixed = t.new_leaf(Style::default()).unwrap();
    let root = t
        .new_with_children(
            Style {
                display: Display::Grid,
                size: Size { width: length(150.0_f32), height: auto() },
                grid_template_columns: vec![auto(), length(100.0_f32)],
                ..Style::default()
            },
            &[item, fixed],
        )
        .unwrap();
    t.compute_layout(root, Size::MAX_CONTENT).unwrap();
    vec![t.layout(item).unwrap().size.width, t.layout(child).unwrap().size.width, t.layout(fixed).unwrap().location.x]
}

pub fn verify() {
    let actual = scenario();
    let expected = vec![50.0, 240.0, 50.0];
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.into_iter().zip(expected) {
        assert!((actual - expected).abs() < 0.02, "actual={actual}, expected={expected}");
    }
}
