use crate::consts::*;
use crate::ui::*;
use bevy::app::AppExit;
use bevy::prelude::*;

pub fn main_menu_buttons(
    button_materials: Res<MenuButtonMaterials>,
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
                    text if text == NEW_GAME_TEXT => app_state.set(AppState::InGame).unwrap(),
                    text if text == EXIT_TO_DESKTOP_TEXT => exit.send(AppExit),
                    _ => (),
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
