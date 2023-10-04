use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy::math::Vec3Swizzles;
use rand::prelude::Rng;

use crate::*;


#[derive(InspectorOptions, Component, Clone, Copy, Debug)]
pub enum ParrotType {
    Blue,
    Red
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Parrot {
    pub health: usize,
    pub distress_timer: Timer,
    pub is_distressed: bool,
}


#[derive(Event)]
pub struct DistressedParrotEvent;


#[derive(Event)]
pub struct RelaxedParrotEvent;

#[derive(Component)]
pub struct Bang;


pub struct ParrotPlugin;

impl Plugin for ParrotPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Parrot>()
        .add_event::<DistressedParrotEvent>()
        .add_event::<RelaxedParrotEvent>()
        .add_systems(Update, distress_parrots.run_if(in_state(GameState::Gameplay)))
        //.add_systems(Update, spawn_bangs.run_if(in_state(GameState::Gameplay)))
        .add_systems(Update, update_parrot_sprites.run_if(in_state(GameState::Gameplay)))
        .add_systems(Update, despawn_bangs.run_if(in_state(GameState::Gameplay)))
        .add_systems(Update, relax_parrots.run_if(in_state(GameState::Gameplay)))
        .add_systems(Update, check_parrot_health.run_if(in_state(GameState::Gameplay)))
        .add_systems(Update, check_parrots_left.run_if(in_state(GameState::Gameplay)));
    }
}

fn update_parrot_sprites(
    mut _commands: Commands,
    mut parrots_q: Query<(&Parrot, &mut AtlasSprite3dComponent)>
) {
    for (parrot, mut sprite) in parrots_q.iter_mut() {
        match parrot.health {
            4 => sprite.index = 0,
            3 => sprite.index = 1,
            2 => sprite.index = 2,
            _ => sprite.index = 3,
        }
    }
}

fn spawn_bang(
    commands: &mut Commands,
    game_assets: &GameAssets,
    sprite_params: &mut Sprite3dParams,
    parent_entity: Entity
) {
    let bang_id = commands.spawn((
        Sprite3d {
            image: game_assets.bang_image.clone(),
            pixels_per_metre: 500.,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            transform: Transform::from_xyz(0., 0.7, -0.1)
                .with_scale(Vec3::new(1., 1., 1.)),
            ..default()
            }.bundle(sprite_params),
        Bang,
        Name::new("Bang")
    )).id();
    commands.entity(parent_entity).push_children(&[bang_id]);
}

fn despawn_bangs(
    mut commands: Commands,
    parrots_q: Query<&Parrot>,
    bangs_q: Query<(Entity, &Bang, &Parent)>,
) {
    for (entity, _bang, parent) in bangs_q.iter() {
        let parent_parrot = parrots_q.get(parent.get());
        if let Ok(parent_parrot) = parent_parrot {
            if !parent_parrot.is_distressed {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn distress_parrots(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut parrots_q: Query<(Entity, &mut Parrot)>,
    mut distress_events: EventReader<DistressedParrotEvent>,
    mut rng_q: Query<&mut EntropyComponent<ChaCha8Rng>>,
    mut sprite_params : Sprite3dParams,
) {
    if !parrots_q.is_empty() {
        let max_parrots = parrots_q.iter().len();
        let mut who = 0;

        if max_parrots != 1 {
            let mut rng = rng_q.single_mut();
            who = rng.gen_range(0..max_parrots) as usize;    
        }
        
        for _event in distress_events.iter() {
            for (i, (entity, mut parrot)) in &mut parrots_q.iter_mut().enumerate() {
                if who == i {
                    parrot.is_distressed = true;
                    spawn_bang(&mut commands, &game_assets, &mut sprite_params, entity);
                }
            }
        }
    } 
}

fn relax_parrots(
    mut parrots_q: Query<&mut Parrot>,
    mut relaxed_events: EventReader<RelaxedParrotEvent>,
) {
    for _event in relaxed_events.iter() {
        for (_i, mut parrot) in &mut parrots_q.iter_mut().enumerate() {
            parrot.is_distressed = false;
            parrot.distress_timer.reset();
        }
    }
}

fn check_parrot_health(
    mut commands: Commands,
    mut parrots: Query<(Entity, &mut Parrot)>,
    time: Res<Time>,
) {
    for (entity, mut parrot) in &mut parrots.iter_mut() {
        if parrot.is_distressed {
            parrot.distress_timer.tick(time.delta());

            if parrot.distress_timer.just_finished() {
                parrot.health -= 1;
            }

            if parrot.health == 0 {
                commands.entity(entity).despawn_recursive();
            }
        } 
    }
}

fn check_parrots_left(
    mut _commands: Commands,
    parrots_q: Query<&Parrot>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if parrots_q.is_empty() {
        game_state.set(GameState::GameOver);
    }
}

impl ParrotType {
    fn get_parrot(&self, assets: &GameAssets) -> (Handle<TextureAtlas>, Parrot) {
        match self {
            ParrotType::Blue => (
                assets.parrot_blue_atlas.clone(),
                Parrot {
                    health: 4,
                    distress_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
                    is_distressed: false,
                }
            ),
            ParrotType::Red => (
                assets.parrot_red_atlas.clone(),
                Parrot {
                    health: 4,
                    distress_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
                    is_distressed: false,
                }
            ),
        }
    }
}


pub fn spawn_parrot(
    commands: &mut ChildBuilder,
    assets: &GameAssets,
    sprite_params: &mut Sprite3dParams,
    xyz: Vec3,
    parrot_type: ParrotType,
) -> Entity {
    let (atlas, parrot) = parrot_type.get_parrot(assets);

    commands.spawn((
        AtlasSprite3d {
            atlas,
            index: 0,
            pixels_per_metre: 400.,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            transform: Transform::from_xyz(xyz.x, xyz.y, xyz.z)
                .with_scale(Vec3::new(0.6, 0.6, 0.6)),
            ..default()
            }.bundle(sprite_params),
        parrot,
        parrot_type,
        Name::new(format!("Parrot_{:?}", parrot_type))
    )).id()
}