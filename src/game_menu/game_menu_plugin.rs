use super::*;
use crate::consts::*;
use crate::ui::*;
use bevy::prelude::*;

pub struct GameMenuPlugin;
impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MenuButtonMaterials>()
            .add_system_set(
                SystemSet::on_enter(AppState::GameMenu).with_system(setup_game_menu.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::GameMenu).with_system(game_menu_buttons.system()),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::GameMenu).with_system(despawn_game_menu.system()),
            );
    }
}

fn despawn_game_menu(mut commands: Commands, query: Query<(Entity, &GameMenu)>) {
    for (e, _) in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

struct GameMenu;

pub fn setup_game_menu(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<MenuButtonMaterials>,
) {
    let background_color = color_materials.add(BACKGROUND_DEFAULT.into());
    let primary_lighter = color_materials.add(PRIMARY_LIGHT.into());
    let font = asset_server.load(LATO_REGULAR);

    let layer_node = layer_node(background_color);
    let button_wrapper = paper(200.0, 200.0, primary_lighter);

    let buttons = [
        MenuButtons::Resume,
        MenuButtons::ExitToMainMenu,
        MenuButtons::ExitToDesktop,
    ];

    commands
        .spawn_bundle(layer_node)
        .insert(GameMenu)
        .with_children(|parent| {
            parent.spawn_bundle(button_wrapper).with_children(|parent| {
                for button in buttons {
                    let btn_material = button_materials.normal.clone();
                    parent
                        .spawn_bundle(menu_button(btn_material))
                        .with_children(|parent| {
                            let label = button.name();
                            let btn_label = button_label(label, font.clone());

                            parent.spawn_bundle(btn_label);
                        });
                }
            });
        });
}
