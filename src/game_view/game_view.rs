use super::*;
use crate::consts::*;
use crate::ui::*;
use bevy::prelude::*;

pub struct InGamePlugin;
impl Plugin for InGamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(ClearColor(BACKGROUND_DEFAULT))
            .init_resource::<TopBarButtonMaterials>()
            .add_system_set(
                SystemSet::on_enter(AppState::InGame).with_system(setup_game_view.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame).with_system(game_view_buttons.system()),
            );
    }
}

pub const MENU_TEXT: &str = "Menu";

fn setup_game_view(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    let font = asset_server.load(LATO_REGULAR);
    let background_color = color_materials.add(BACKGROUND_DEFAULT.into());
    let color = PRIMARY_LIGHTER;

    let root_node = root_node();
    let top_bar = top_bar(background_color.clone());
    let menu_button = top_bar_menu_button(background_color.clone());

    commands.spawn_bundle(root_node).with_children(|parent| {
        parent.spawn_bundle(top_bar).with_children(|parent| {
            parent.spawn_bundle(menu_button).with_children(|parent| {
                let label = MENU_TEXT.to_string();
                let btn_label = top_bar_button_label(label, font.clone(), color.clone());
                parent.spawn_bundle(btn_label);
            });

            let time_label_text = "TEST".to_string();
            let time_label = time_label(time_label_text, font.clone(), color.clone());
            parent.spawn_bundle(time_label);
        });
    });
}
