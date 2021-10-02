mod consts;
mod game_menu;
mod game_view;
mod main_menu;
mod resources;
mod ui;

use bevy::prelude::*;
use consts::AppState;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "TEST".to_string(),
            width: 640.0,
            height: 480.0,
            ..Default::default()
        })
        .add_state(AppState::MainMenu)
        .add_plugins(DefaultPlugins)
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(game_view::InGamePlugin)
        .add_plugin(game_menu::GameMenuPlugin)
        .run();
}
