use taffy::prelude::*;

fn tree() -> TaffyTree<()> {
    let mut tree = TaffyTree::new();
    tree.disable_rounding();
    tree
}

pub fn scenario() -> Vec<f32> {
    let mut result = Vec::new();
    for alignment in [None, Some(AlignSelf::STRETCH)] {
        let mut t = tree();
        let item = t
            .new_leaf(Style {
                item_is_replaced: true,
                aspect_ratio: Some(2.0),
                justify_self: alignment,
                align_self: alignment,
                ..Style::default()
            })
            .unwrap();
        let root = t
            .new_with_children(
                Style {
                    display: Display::Grid,
                    size: Size { width: length(300.0_f32), height: length(200.0_f32) },
                    grid_template_columns: vec![length(300.0_f32)],
                    grid_template_rows: vec![length(200.0_f32)],
                    ..Style::default()
                },
                &[item],
            )
            .unwrap();
        t.compute_layout_with_measure(root, Size::MAX_CONTENT, |input, _, _, _| {
            let known = input.known_dimensions;
            taffy::LayoutOutput::from_outer_size(Size {
                width: known.width.unwrap_or(120.0),
                height: known.height.unwrap_or(60.0),
            })
        })
        .unwrap();
        result.extend([t.layout(item).unwrap().size.width, t.layout(item).unwrap().size.height]);
    }
    let mut t = tree();
    let item = t
        .new_leaf(Style {
            position: Position::Absolute,
            aspect_ratio: Some(3.0),
            inset: taffy::Rect {
                left: length(20.0_f32),
                right: length(20.0_f32),
                top: length(15.0_f32),
                bottom: length(15.0_f32),
            },
            ..Style::default()
        })
        .unwrap();
    let root = t
        .new_with_children(
            Style {
                display: Display::Grid,
                size: Size { width: length(400.0_f32), height: length(300.0_f32) },
                ..Style::default()
            },
            &[item],
        )
        .unwrap();
    t.compute_layout(root, Size::MAX_CONTENT).unwrap();
    result.extend([t.layout(item).unwrap().size.width, t.layout(item).unwrap().size.height]);
    result
}

pub fn verify() {
    let actual = scenario();
    let expected = vec![120.0, 60.0, 300.0, 200.0, 360.0, 120.0];
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.into_iter().zip(expected) {
        assert!((actual - expected).abs() < 0.02, "actual={actual}, expected={expected}");
    }
}
