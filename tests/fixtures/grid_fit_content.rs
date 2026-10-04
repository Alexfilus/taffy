use taffy::prelude::*;

fn tree() -> TaffyTree<()> {
    let mut tree = TaffyTree::new();
    tree.disable_rounding();
    tree
}

pub fn scenario() -> Vec<f32> {
    let mut t = tree();
    let leaf = t.new_leaf(Style::default()).unwrap();
    let item = t
        .new_with_children(
            Style {
                display: Display::Grid,
                grid_template_columns: vec![fr(1.0_f32)],
                justify_self: Some(AlignSelf::CENTER),
                ..Style::default()
            },
            &[leaf],
        )
        .unwrap();
    let root = t
        .new_with_children(
            Style {
                display: Display::Grid,
                size: Size { width: length(300.0_f32), height: auto() },
                grid_template_columns: vec![length(300.0_f32)],
                ..Style::default()
            },
            &[item],
        )
        .unwrap();
    t.compute_layout_with_measure(root, Size::MAX_CONTENT, |input, _, _, _| {
        let width = input.known_dimensions.width.unwrap_or(match input.available_space.width {
            AvailableSpace::MinContent => 100.0,
            AvailableSpace::Definite(w) => w.clamp(100.0, 600.0),
            _ => 600.0,
        });
        taffy::LayoutOutput::from_outer_size(Size {
            width,
            height: input.known_dimensions.height.unwrap_or((600.0 / width).ceil() * 10.0),
        })
    })
    .unwrap();
    vec![
        t.layout(item).unwrap().size.width,
        t.layout(item).unwrap().location.x,
        t.layout(item).unwrap().size.height,
        t.layout(root).unwrap().size.height,
    ]
}

pub fn verify() {
    let actual = scenario();
    let expected = vec![300.0, 0.0, 20.0, 20.0];
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.into_iter().zip(expected) {
        assert!((actual - expected).abs() < 0.02, "actual={actual}, expected={expected}");
    }
}
