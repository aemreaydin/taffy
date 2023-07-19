#[test]
fn grid_margins_auto_margins_override_stretch() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node1 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node2 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node3 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node4 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node5 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node6 = taffy
        .new_leaf_with_measure(
            taffy::style::Style {
                align_self: Some(taffy::style::AlignSelf::Stretch),
                justify_self: Some(taffy::style::JustifySelf::Stretch),
                margin: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentageAuto::Auto,
                    right: taffy::style::LengthPercentageAuto::Auto,
                    top: taffy::style::LengthPercentageAuto::Auto,
                    bottom: taffy::style::LengthPercentageAuto::Auto,
                },
                ..Default::default()
            },
            taffy::tree::MeasureFunc::Raw(|known_dimensions, available_space| {
                const TEXT: &str = "HH\u{200b}HH";
                crate::generated::measure_standard_text(
                    known_dimensions,
                    available_space,
                    TEXT,
                    crate::generated::WritingMode::Horizontal,
                    None,
                )
            }),
        )
        .unwrap();
    let node7 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node8 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![length(40f32), length(40f32), length(40f32)],
                grid_template_columns: vec![length(40f32), length(40f32), length(40f32)],
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(40f32),
                    right: taffy::style::LengthPercentage::Length(20f32),
                    top: taffy::style::LengthPercentage::Length(10f32),
                    bottom: taffy::style::LengthPercentage::Length(30f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4, node5, node6, node7, node8],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 180f32, "width of node {:?}. Expected {}. Actual {}", node, 180f32, size.width);
    assert_eq!(size.height, 160f32, "height of node {:?}. Expected {}. Actual {}", node, 160f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node0, 40f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node0, 40f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node0, 10f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node1, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node1, 40f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node1, 80f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node1, 10f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node2, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node2, 40f32, size.height);
    assert_eq!(location.x, 120f32, "x of node {:?}. Expected {}. Actual {}", node2, 120f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node2, 10f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node3).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node3, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node3, 40f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node3, 40f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node3, 50f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node4).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node4, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node4, 40f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node4, 80f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node4, 50f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node5).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node5, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node5, 40f32, size.height);
    assert_eq!(location.x, 120f32, "x of node {:?}. Expected {}. Actual {}", node5, 120f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node5, 50f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node6).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node6, 40f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node6, 10f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node6, 40f32, location.x);
    assert_eq!(location.y, 105f32, "y of node {:?}. Expected {}. Actual {}", node6, 105f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node7).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node7, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node7, 40f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node7, 80f32, location.x);
    assert_eq!(location.y, 90f32, "y of node {:?}. Expected {}. Actual {}", node7, 90f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node8).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node8, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node8, 40f32, size.height);
    assert_eq!(location.x, 120f32, "x of node {:?}. Expected {}. Actual {}", node8, 120f32, location.x);
    assert_eq!(location.y, 90f32, "y of node {:?}. Expected {}. Actual {}", node8, 90f32, location.y);
}