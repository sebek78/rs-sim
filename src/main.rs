mod consts;
mod design_consts;
mod in_game;
mod main_menu;
mod main_menu_buttons;

use crate::consts::*;
use bevy::prelude::*;
use in_game::InGamePlugin;
use main_menu::MainMenuPlugin;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "TEST".to_string(),
            width: 640.0,
            height: 480.0,
            ..Default::default()
        })
        .add_state(AppState::MainMenu)
        .add_plugins(DefaultPlugins)
        .add_plugin(MainMenuPlugin)
        .add_plugin(InGamePlugin)
        .run();
}
