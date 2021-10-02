use crate::consts::*;
use crate::resources::Turn;
use crate::ui::CustomId;
use bevy::prelude::*;

pub const MENU_TEXT: &str = "Menu";
pub const NEXT_TURN: &str = "Next turn";

pub struct TopBarButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromWorld for TopBarButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        TopBarButtonMaterials {
            normal: materials.add(PRIMARY_DARK.into()),
            hovered: materials.add(PRIMARY.into()),
            pressed: materials.add(PRIMARY_LIGHT.into()),
        }
    }
}

pub fn game_view_buttons(
    top_bar_button_materials: Res<TopBarButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text, Without<CustomId>>,
    mut app_state: ResMut<State<AppState>>,
    mut turn: ResMut<Turn>,
    mut time_label_query: Query<&mut Text, With<CustomId>>,
) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let button = text_query.get_mut(children[0]).unwrap();
        let target = &button.sections[0].value[..];

        match *interaction {
            Interaction::Clicked => {
                *material = top_bar_button_materials.pressed.clone();
                match target {
                    text if text == MENU_TEXT => app_state.set(AppState::GameMenu).unwrap(),
                    text if text == NEXT_TURN => {
                        turn.next_turn();
                        for mut time_label in time_label_query.iter_mut() {
                            time_label.sections[0].value = turn.label.clone();
                        }
                    }
                    _ => (),
                }
            }
            Interaction::Hovered => {
                *material = top_bar_button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = top_bar_button_materials.normal.clone();
            }
        }
    }
}
