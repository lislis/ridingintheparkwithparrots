use bevy::prelude::*;
//use bevy_third_person_camera::*;
use bevy::math::Vec3Swizzles;
use rand::prelude::Rng;

use crate::*;

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

pub struct ParrotPlugin;

impl Plugin for ParrotPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Parrot>()
        .add_event::<DistressedParrotEvent>()
        .add_event::<RelaxedParrotEvent>()
        .add_systems(Update, distress_parrots.run_if(in_state(GameState::Gameplay)))
        .add_systems(Update, relax_parrots.run_if(in_state(GameState::Gameplay)))
        .add_systems(Update, check_parrot_health.run_if(in_state(GameState::Gameplay)))
        .add_systems(Update, check_parrots_left.run_if(in_state(GameState::Gameplay)));
    }
}

fn distress_parrots(
    mut parrots_q: Query<&mut Parrot>,
    mut distress_events: EventReader<DistressedParrotEvent>,
    mut rng_q: Query<&mut EntropyComponent<ChaCha8Rng>>,
) {
    if !parrots_q.is_empty() {
        let max_parrots = parrots_q.iter().len();
        let mut who = 0;

        if max_parrots != 1 {
            let mut rng = rng_q.single_mut();
            who = rng.gen_range(0..max_parrots) as usize;    
        }
        
        for _event in distress_events.iter() {
            for (i, mut parrot) in &mut parrots_q.iter_mut().enumerate() {
                if who == i {
                    parrot.is_distressed = true;
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
        for (i, mut parrot) in &mut parrots_q.iter_mut().enumerate() {
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
    mut parrots_q: Query<&mut Parrot>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if parrots_q.is_empty() {
        game_state.set(GameState::GameOver);
    }
}