use super::*;
use crate::consts::*;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
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
    button_materials: Res<ButtonMaterials>,
) {
    let title_font = asset_server.load(MEDIEVAL_REGULAR);
    let buttons = [MenuButtons::NewGame, MenuButtons::ExitToDesktop];

    commands.spawn_bundle(UiCameraBundle::default());

    commands
        // root node
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                display: Display::Flex,
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MainMenu)
        .with_children(|parent| {
            // title
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: color_materials.add(BACKGROUND_DEFAULT.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            ..Default::default()
                        },
                        text: Text::with_section(
                            "Test Example",
                            TextStyle {
                                font: title_font,
                                font_size: H1_SIZE,
                                color: PRIMARY_LIGHTER,
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
            // menu
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(400.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: color_materials.add(BACKGROUND_DEFAULT.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    for button in buttons {
                        // menu button
                        parent
                            .spawn_bundle(ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(160.0), Val::Px(40.0)),
                                    margin: Rect::all(Val::Px(8.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..Default::default()
                                },
                                material: button_materials.normal.clone(),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent.spawn_bundle(TextBundle {
                                    text: Text::with_section(
                                        button.name(),
                                        TextStyle {
                                            font: asset_server.load(LATO_REGULAR),
                                            font_size: MENU_BUTTON_TEXT,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                        Default::default(),
                                    ),
                                    ..Default::default()
                                });
                            });
                    }
                });
        });
}
