use crate::consts::*;
use crate::design_consts::*;
use bevy::prelude::*;

pub struct InGamePlugin;
impl Plugin for InGamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(ClearColor(BACKGROUND_DEFAULT))
            .add_system_set(
                SystemSet::on_enter(AppState::InGame).with_system(setup_in_game.system()),
            );
    }
}

fn setup_in_game(mut commands: Commands, mut color_materials: ResMut<Assets<ColorMaterial>>) {
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
            material: color_materials.add(BACKGROUND_PAPER.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            // top bar
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(40.0)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                material: color_materials.add(BACKGROUND_DEFAULT.into()),
                ..Default::default()
            });
        });
}
