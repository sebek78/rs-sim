use crate::consts::*;
use bevy::app::AppExit;
use bevy::prelude::*;

pub struct ButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(PRIMARY_DARK.into()),
            hovered: materials.add(PRIMARY.into()),
            pressed: materials.add(PRIMARY_LIGHT.into()),
        }
    }
}

const NEW_GAME_TEXT: &str = "New game";
const EXIT_TO_DESKTOP_TEXT: &str = "Exit to desktop";

pub enum MenuButtons {
    NewGame,
    ExitToDesktop,
}

impl MenuButtons {
    pub fn name(&self) -> String {
        match self {
            Self::NewGame => NEW_GAME_TEXT.to_string(),
            Self::ExitToDesktop => EXIT_TO_DESKTOP_TEXT.to_string(),
        }
    }
}

pub fn menu_button(btn_material: Handle<ColorMaterial>) -> ButtonBundle {
    return ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(160.0), Val::Px(40.0)),
            margin: Rect::all(Val::Px(8.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        material: btn_material,
        ..Default::default()
    };
}

pub fn button_label(label: String, font: Handle<Font>) -> TextBundle {
    return TextBundle {
        text: Text::with_section(
            label,
            TextStyle {
                font: font,
                font_size: MENU_BUTTON_TEXT,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
            Default::default(),
        ),
        ..Default::default()
    };
}

pub fn main_menu_buttons(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut app_state: ResMut<State<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let button = text_query.get_mut(children[0]).unwrap();
        let target = &button.sections[0].value[..];

        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                match target {
                    NEW_GAME_TEXT => app_state.set(AppState::InGame).unwrap(),
                    EXIT_TO_DESKTOP_TEXT => exit.send(AppExit),
                    _ => unreachable!(),
                }
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}
