use bevy::prelude::*;

pub fn layer_node(background_color: Handle<ColorMaterial>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..Default::default()
            },
            display: Display::Flex,
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        material: background_color,
        ..Default::default()
    }
}
