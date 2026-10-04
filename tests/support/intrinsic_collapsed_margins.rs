use taffy::prelude::*;
use taffy::Rect;

fn tree() -> TaffyTree<()> {
    let mut tree = TaffyTree::new();
    tree.disable_rounding();
    tree
}

pub fn scenario() -> Vec<f32> {
    let mut t = tree();
    let lead = t
        .new_leaf(Style {
            display: Display::Block,
            size: Size { width: auto(), height: length(20.0_f32) },
            margin: Rect { top: length(16.0_f32), ..Rect::zero() },
            ..Style::default()
        })
        .unwrap();
    let deep = t
        .new_leaf(Style {
            display: Display::Block,
            size: Size { width: auto(), height: length(100.0_f32) },
            margin: Rect { top: length(64.0_f32), ..Rect::zero() },
            ..Style::default()
        })
        .unwrap();
    let section = t.new_with_children(Style { display: Display::Block, ..Style::default() }, &[deep]).unwrap();
    let item = t.new_with_children(Style { display: Display::Block, ..Style::default() }, &[lead, section]).unwrap();
    let after =
        t.new_leaf(Style { size: Size { width: auto(), height: length(10.0_f32) }, ..Style::default() }).unwrap();
    let root = t
        .new_with_children(
            Style {
                display: Display::Grid,
                size: Size { width: length(300.0_f32), height: auto() },
                gap: Size { width: length(0.0_f32), height: length(20.0_f32) },
                ..Style::default()
            },
            &[item, after],
        )
        .unwrap();
    t.compute_layout(root, Size::MAX_CONTENT).unwrap();
    vec![t.layout(item).unwrap().size.height, t.layout(after).unwrap().location.y, t.layout(root).unwrap().size.height]
}

pub fn verify() {
    let actual = scenario();
    let expected = vec![200.0, 220.0, 230.0];
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.into_iter().zip(expected) {
        assert!((actual - expected).abs() < 0.02, "actual={actual}, expected={expected}");
    }
}
