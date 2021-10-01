use crate::consts::*;
use bevy::prelude::*;

pub fn top_bar(material: Handle<ColorMaterial>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Px(40.0)),
            display: Display::Flex,
            flex_direction: FlexDirection::RowReverse,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        material,
        ..Default::default()
    }
}

pub fn top_bar_menu_button(btn_material: Handle<ColorMaterial>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(80.0), Val::Px(40.0)),
            margin: Rect::all(Val::Px(16.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        material: btn_material,
        ..Default::default()
    }
}

pub fn top_bar_button_label(label: String, font: Handle<Font>, color: Color) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            label,
            TextStyle {
                font,
                font_size: MENU_BUTTON_TEXT,
                color,
            },
            Default::default(),
        ),
        ..Default::default()
    }
}

pub fn time_label(label: String, font: Handle<Font>, color: Color) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            label,
            TextStyle {
                font,
                font_size: TEXT_SIZE,
                color,
            },
            Default::default(),
        ),
        style: Style {
            margin: Rect::all(Val::Px(16.0)),
            ..Default::default()
        },
        ..Default::default()
    }
}
