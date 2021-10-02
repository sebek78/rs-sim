use super::*;
use crate::consts::*;
use crate::ui::*;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MenuButtonMaterials>()
            .insert_resource(ClearColor(BACKGROUND_DEFAULT))
            .add_system_set(
                SystemSet::on_enter(AppState::MainMenu).with_system(setup_main_menu.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::MainMenu).with_system(main_menu_buttons.system()),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::MainMenu).with_system(despawn_main_menu.system()),
            );
    }
}

fn despawn_main_menu(mut commands: Commands, query: Query<(Entity, &MainMenu)>) {
    for (e, _) in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

struct MainMenu;

fn setup_main_menu(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<MenuButtonMaterials>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    let title_font = asset_server.load(MEDIEVAL_REGULAR);
    let font = asset_server.load(LATO_REGULAR);
    let background_color = color_materials.add(BACKGROUND_DEFAULT.into());

    let root_node = root_node(background_color.clone(), JustifyContent::FlexStart);
    let title_area = container(100.0, background_color.clone());
    let title = title(title_font);

    let button_area = container(400.0, background_color);
    let buttons = [MenuButtons::NewGame, MenuButtons::ExitToDesktop];

    commands
        .spawn_bundle(root_node)
        .insert(MainMenu)
        .with_children(|parent| {
            parent.spawn_bundle(title_area).with_children(|parent| {
                parent.spawn_bundle(title);
            });
            parent.spawn_bundle(button_area).with_children(|parent| {
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
