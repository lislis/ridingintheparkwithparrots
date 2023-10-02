use bevy::prelude::*;
use bevy::math::Vec3Swizzles;

use crate::GameState;
use crate::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Target {
    pub speed: f32,
    pub path_index: usize
}

#[derive(Resource)]
pub struct TargetPath {
    waypoints: Vec<Vec2>
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Health {
    pub value: i32
}

#[derive(Component)]
pub struct DamageSound;


#[derive(Event)]
pub struct TargetDeathEvent;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Target>()
        .register_type::<Health>()
        .add_event::<TargetDeathEvent>()
        .insert_resource(TargetPath {
            waypoints: vec![
                Vec2::new(3.0, 2.0),
                Vec2::new(3.0, 3.0),
                Vec2::new(-2.0, -2.0),
            ]
        })
        .add_systems(Update, move_target.run_if(in_state(GameState::Gameplay)))
        .add_systems(Update, target_death.run_if(in_state(GameState::Gameplay)))
        //.add_systems(OnEnter(GameState::Gameplay), play_damage_sound)
        .add_systems(Update, hurt_player.run_if(in_state(GameState::Gameplay)));
    }
}

fn move_target(
    mut targets: Query<(&mut Target, &mut Transform)>,
    path: Res<TargetPath>,
    time: Res<Time>
) {
    for (mut target, mut transform) in &mut targets {
        let delta = target.speed * time.delta_seconds();
        if path.waypoints.len() > target.path_index {
            let delta_target = path.waypoints[target.path_index] - transform.translation.xz();

            // this step will get us closer to the goal
            if delta_target.length() > delta {
                let movement = delta_target.normalize() * delta;
                transform.translation += movement.extend(0.0).xzy();
                // copy for ownership reasons
                let y = transform.translation.y;
                transform.look_at(path.waypoints[target.path_index].extend(y).xzy(), Vec3::Y);
            } else {
                target.path_index += 1;
            }
            //transform.translation.x += target.speed * time.delta_seconds();
        }
        
    }
}

fn target_death(
    mut commands: Commands,
    targets: Query<(Entity, &Health)>,
    mut death_event_writer: EventWriter<TargetDeathEvent>
) {
    for (ent, health) in &targets {
        if health.value <= 0 {
            commands.entity(ent).despawn_recursive();
            death_event_writer.send(TargetDeathEvent);
        }
    }
}

fn hurt_player(
    mut commands: Commands,
    targets: Query<(Entity, &Target)>,
    path: Res<TargetPath>,
    mut player: Query<&mut Player>,
    //damage_sound: Query<&AudioSink, With<DamageSound>>,
    assets: Res<GameAssets>
) {
    for (entity, target) in &targets {
        if target.path_index >= path.waypoints.len() {
            commands.entity(entity).despawn_recursive();

            let mut player = player.single_mut();
            if player.health > 0 {
                player.health -= 1;

                play_damage_sound(&mut commands, &assets);
            }
            if player.health == 0 {
                info!("GAME OVER");
            }
        }
    }
}

fn play_damage_sound(
    commands: &mut Commands,
    assets: &GameAssets,
) {
    commands.spawn((AudioBundle {
        source: assets.damage_sound.clone(),
        settings: PlaybackSettings::ONCE,
    }, DamageSound));
}