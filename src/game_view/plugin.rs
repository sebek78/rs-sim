use super::*;
use crate::consts::*;
use crate::resources::Turn;
use crate::ui::*;
use bevy::prelude::*;

pub struct InGamePlugin;
impl Plugin for InGamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(ClearColor(BACKGROUND_DEFAULT))
            .init_resource::<TopBarButtonMaterials>()
            .insert_resource(Turn::init_turn())
            .add_system_set(
                SystemSet::on_enter(AppState::InGame).with_system(setup_game_view.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame).with_system(game_view_buttons.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame).with_system(keyboard_inputs.system()),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGame).with_system(despawn_game_view.system()),
            );
    }
}

fn despawn_game_view(mut commands: Commands, query: Query<(Entity, &GameView)>) {
    for (e, _) in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub struct GameView;

fn setup_game_view(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    game_time: Res<Turn>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    let font = asset_server.load(LATO_REGULAR);
    let background_color = color_materials.add(PRIMARY_DARK.into());
    let transparent_color = color_materials.add(TRANSPARENT_BACKGROUND.into());
    let color = PRIMARY_LIGHTER;

    let root_node = root_node(transparent_color, JustifyContent::SpaceBetween);
    let top_bar = create_bar(background_color.clone());
    let bottom_bar = create_bar(background_color.clone());
    let menu_button = bar_menu_button(background_color);

    commands
        .spawn_bundle(root_node)
        .insert(GameView)
        .with_children(|parent| {
            parent.spawn_bundle(top_bar).with_children(|parent| {
                parent
                    .spawn_bundle(menu_button.clone())
                    .with_children(|parent| {
                        let label = MENU_TEXT.to_string();
                        let btn_label = bar_button_label(label, font.clone(), color);
                        parent.spawn_bundle(btn_label);
                    });

                let time_label = time_label(game_time.label.clone(), font.clone(), color);
                parent.spawn_bundle(time_label).insert(CustomId {
                    id: NEXT_TURN.to_string(),
                });
            });
            parent.spawn_bundle(bottom_bar).with_children(|parent| {
                parent.spawn_bundle(menu_button).with_children(|parent| {
                    let label = NEXT_TURN.to_string();
                    let btn_label = bar_button_label(label, font.clone(), color);
                    parent.spawn_bundle(btn_label);
                });
            });
        });
}
