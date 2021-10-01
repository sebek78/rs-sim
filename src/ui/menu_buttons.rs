use crate::consts::*;
use bevy::prelude::*;

pub const EXIT_TO_DESKTOP_TEXT: &str = "Exit to desktop";
pub const EXIT_TO_MAIN_MENU_TEXT: &str = "Exit to main menu";
pub const NEW_GAME_TEXT: &str = "New game";
pub const RESUME_TEXT: &str = "Resume";

pub enum MenuButtons {
    ExitToDesktop,
    ExitToMainMenu,
    NewGame,
    Resume,
}

impl MenuButtons {
    pub fn name(&self) -> &str {
        match self {
            Self::ExitToDesktop => EXIT_TO_DESKTOP_TEXT,
            Self::ExitToMainMenu => EXIT_TO_MAIN_MENU_TEXT,
            Self::NewGame => NEW_GAME_TEXT,
            Self::Resume => RESUME_TEXT,
        }
    }
}

pub fn menu_button(material: Handle<ColorMaterial>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(160.0), Val::Px(40.0)),
            margin: Rect::all(Val::Px(8.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        material,
        ..Default::default()
    }
}

pub fn button_label(label: &str, font: Handle<Font>) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            label.to_string(),
            TextStyle {
                font,
                font_size: MENU_BUTTON_TEXT,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
            Default::default(),
        ),
        ..Default::default()
    }
}
