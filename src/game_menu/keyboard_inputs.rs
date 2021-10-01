use crate::consts::AppState;
use bevy::prelude::*;

pub fn keyboard_inputs(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        keyboard_input.reset(KeyCode::Escape);
        app_state.set(AppState::InGame).unwrap()
    }
}
