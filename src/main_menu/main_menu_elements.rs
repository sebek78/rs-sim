use crate::consts::*;
use bevy::prelude::*;

pub fn title(title_font: Handle<Font>) -> TextBundle {
    TextBundle {
        style: Style {
            ..Default::default()
        },
        text: Text::with_section(
            "Test Example",
            TextStyle {
                font: title_font,
                font_size: H1_SIZE,
                color: PRIMARY_LIGHTER,
            },
            Default::default(),
        ),
        ..Default::default()
    }
}
