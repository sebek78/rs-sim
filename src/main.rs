mod consts;
mod game_view;
mod main_menu;
mod ui;

use bevy::prelude::*;
use consts::AppState;
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
        .add_plugin(game_view::InGamePlugin)
        .run();
}
