use taffy::prelude::*;

fn tree() -> TaffyTree<()> {
    let mut tree = TaffyTree::new();
    tree.disable_rounding();
    tree
}

pub fn scenario() -> Vec<f32> {
    let mut t = tree();
    let item = t
        .new_leaf(Style {
            size: Size { width: length(100.0_f32), height: auto() },
            flex_grow: 1.0,
            aspect_ratio: Some(2.0),
            align_self: Some(AlignSelf::START),
            ..Style::default()
        })
        .unwrap();
    let root = t
        .new_with_children(
            Style {
                display: Display::Flex,
                size: Size { width: length(300.0_f32), height: auto() },
                ..Style::default()
            },
            &[item],
        )
        .unwrap();
    t.compute_layout(root, Size::MAX_CONTENT).unwrap();
    vec![t.layout(item).unwrap().size.width, t.layout(item).unwrap().size.height]
}

pub fn verify() {
    let actual = scenario();
    let expected = vec![300.0, 150.0];
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.into_iter().zip(expected) {
        assert!((actual - expected).abs() < 0.02, "actual={actual}, expected={expected}");
    }
}
