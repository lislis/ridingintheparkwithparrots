use bevy::prelude::*;
//use bevy_third_person_camera::*;
use bevy::math::Vec3Swizzles;

use crate::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Parrot {
    pub health: usize
}

pub struct ParrotPlugin;

impl Plugin for ParrotPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Parrot>();
        //.add_systems(OnEnter(GameState::Gameplay), spawn_parrots);
    }
}
