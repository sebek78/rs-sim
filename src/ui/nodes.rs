use bevy::prelude::*;

pub fn root_node() -> NodeBundle {
    return NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            display: Display::Flex,
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..Default::default()
    };
}

pub fn container(height: f32, background_color: Handle<ColorMaterial>) -> NodeBundle {
    return NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Px(height)),
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        material: background_color,
        ..Default::default()
    };
}
