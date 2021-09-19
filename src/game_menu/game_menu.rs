use crate::consts::*;
use crate::ui::*;
use bevy::prelude::*;

pub struct GameMenuPlugin;
impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MenuButtonMaterials>().add_system_set(
            SystemSet::on_enter(AppState::GameMenu).with_system(setup_game_menu.system()),
        );
    }
}

fn setup_game_menu(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    println!("game menu");
}
