use crate::consts::*;
use bevy::prelude::*;

pub struct MenuButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
}

impl FromWorld for MenuButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        MenuButtonMaterials {
            normal: materials.add(PRIMARY_DARK.into()),
            hovered: materials.add(PRIMARY.into()),
            pressed: materials.add(PRIMARY_LIGHT.into()),
        }
    }
}
