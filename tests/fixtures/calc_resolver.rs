use taffy::prelude::*;

fn tree() -> TaffyTree<()> {
    let mut tree = TaffyTree::new();
    tree.disable_rounding();
    tree
}

pub fn scenario() -> Vec<f32> {
    static TOKEN: u64 = 0;
    let mut t = tree();
    t.set_calc_resolver(|_, basis| basis * 0.5 + 10.0);
    let item = t
        .new_leaf(Style {
            size: Size { width: Dimension::calc((&TOKEN as *const u64).cast()), height: length(20.0_f32) },
            ..Style::default()
        })
        .unwrap();
    let root = t
        .new_with_children(
            Style {
                display: Display::Block,
                size: Size { width: length(300.0_f32), height: auto() },
                ..Style::default()
            },
            &[item],
        )
        .unwrap();
    t.compute_layout(root, Size::MAX_CONTENT).unwrap();
    vec![t.layout(item).unwrap().size.width]
}

pub fn verify() {
    let actual = scenario();
    let expected = vec![160.0];
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.into_iter().zip(expected) {
        assert!((actual - expected).abs() < 0.02, "actual={actual}, expected={expected}");
    }
}
