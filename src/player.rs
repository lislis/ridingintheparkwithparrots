use bevy::prelude::*;
use bevy::math::Vec3Swizzles;

use bevy_rand::prelude::*;
use bevy_prng::ChaCha8Rng;
use rand::prelude::Rng;

use crate::*;

pub const PLAYER_SPEED: f32 = 1.0;
pub const BALANCE_BASE: f32 = 90.0;
pub const BALANCE_WIGGLE_ROOM: f32 = 3.0;

#[derive(Resource)]
pub struct PlayerPath {
    waypoints: Vec<Vec2>
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player {
    pub balance: f32,
    pub speed: f32,
    pub path_index: usize,
    pub disrupt_timer: Timer
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Handlebar {
    pub prev_rotation: f32,
}


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Player>()
        .register_type::<Handlebar>()
        .insert_resource(PlayerPath {
            waypoints: vec![
                Vec2::new(1.0, 1.0),
                Vec2::new(2.0, 3.0),
                Vec2::new(2.0, -2.0),
                Vec2::new(5.0, -1.0),
                Vec2::new(-2.0, -1.0),
                Vec2::new(2.0, 0.0),
                Vec2::new(6.0, 0.0),
                Vec2::new(4.0, 3.0),
                Vec2::new(0.0, 0.0),
            ]
        })
        .add_systems(OnEnter(GameState::Gameplay), spawn_player)
        .add_systems(OnExit(GameState::Gameplay), despawn_player)
        .add_systems(Update, move_player.run_if(in_state(GameState::Gameplay)))
        .add_systems(Update, disrupt_player.run_if(in_state(GameState::Gameplay)))
        .add_systems(Update, handlebar_controls.run_if(in_state(GameState::Gameplay)));
    }
}

fn disrupt_player(
    mut _commands: Commands,
    mut player_q: Query<&mut Player>,
    mut handle_q: Query<(&mut Transform, &mut Handlebar)>,
    mut rng_q: Query<&mut EntropyComponent<ChaCha8Rng>>,
    time: Res<Time>,
    mut parrot_event_writer: EventWriter<DistressedParrotEvent>
) {
    let mut player = player_q.single_mut();
    let (mut handle_transform, mut handlebar) = handle_q.single_mut();
        
    player.disrupt_timer.tick(time.delta());
    if player.disrupt_timer.just_finished() {
        // all the transformation stuff could be called on a custom event as well
        let mut rng = rng_q.single_mut();
        let rand_range = 0.2f32;
        let rand_val = rng.gen_range(-rand_range..rand_range);

        // first rotate back with old amount
        handle_transform.rotate_local_z(-handlebar.prev_rotation);
        // then roate with new amount
        handle_transform.rotate_local_z(rand_val);
        handlebar.prev_rotation = rand_val;

        let out_min = BALANCE_BASE - (BALANCE_BASE * 0.5);
        let out_max = BALANCE_BASE + (BALANCE_BASE * 0.5);
        player.balance = map_range(rand_val, -rand_range, rand_range, out_min, out_max);
        parrot_event_writer.send(DistressedParrotEvent);
    }
}

fn map_range(num: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    return (num - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
}

fn handlebar_controls(
    keyboard: Res<Input<KeyCode>>,
    mut player_q: Query<&mut Player>,
    mut handle_q: Query<(&mut Transform, &mut Handlebar)>,
    time: Res<Time>,
    mut parrot_event_writer: EventWriter<RelaxedParrotEvent>
) {
    let mut player = player_q.single_mut();
    info!("b {}", player.balance);

    let (mut handle_transform, mut handlebar) = handle_q.single_mut();

    let step = 30.0;
    if keyboard.pressed(KeyCode::A) {
        player.balance += step * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        player.balance -= step * time.delta_seconds();
    }

    let lower_bound = BALANCE_BASE - BALANCE_WIGGLE_ROOM;
    let upper_bound = BALANCE_BASE + BALANCE_WIGGLE_ROOM;
    if player.balance > lower_bound && player.balance < upper_bound {
        handle_transform.rotate_local_z(-handlebar.prev_rotation);
        handle_transform.rotate_local_z(0.0);
        handlebar.prev_rotation = 0.0;
        parrot_event_writer.send(RelaxedParrotEvent);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    _game_assets: Res<GameAssets>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>
) {
    let player = (
        SpatialBundle::from_transform(Transform::from_xyz(1.0, 1.0, 1.0)),
        Player {
            balance: BALANCE_BASE,
            path_index: 0,
            speed: PLAYER_SPEED,
            disrupt_timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        },
        EntropyComponent::from(&mut rng),
        Name::new("Player")
    );

    let camera_player = (
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },
        Name::new("PlayerCam")
    );

    let handlebar = (
        PbrBundle {
            mesh: meshes.add(shape::Cylinder::default().into()),
            material: materials.add(Color::GRAY.into()),
            transform: Transform::from_xyz(0.0,-0.3, -1.0)
                .with_rotation(Quat::from_rotation_z(1.57))
                .with_scale(Vec3::new(0.1, 0.9, 0.1)),
            ..default()
        }, 
        NotShadowCaster,
        Handlebar { prev_rotation: 0.0 },
        Name::new("Handlebar")
    );

    commands.spawn(player).with_children(|commands| {
        commands.spawn(camera_player);
        commands.spawn(handlebar);

        // I'm not super fond of spawning parrots here
        // but I couldn't get push_children to work... so here we are
        let colors = [Color::BLUE, Color::RED, Color::YELLOW_GREEN, Color::ORANGE];

        let mut create_parrot = |color: Color, xyz: (f32, f32, f32), name: String | -> (PbrBundle, Parrot, Name) {
            (PbrBundle {
                    mesh: meshes.add(shape::Capsule::default().into()),
                    material: materials.add(color.into()),
                    transform: Transform::from_xyz(xyz.0, xyz.1, xyz.2)
                        .with_scale(Vec3::new(0.15, 0.15, 0.15)),
                    ..default()
                },
                Parrot {
                    health: 3,
                    distress_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
                    is_distressed: false,
                },
                Name::new(name)
            )
        };

        for i in 0..=3 {
            let x = -0.45 + (i as f32 * 0.3);
            commands.spawn(create_parrot(colors[i], (x, -0.2, -1.1), format!("Parrot_{}", i)));
        }
    });
}

fn despawn_player(
    mut commands: Commands,
    player_q: Query<Entity, With<Player>>
) {
    let entity = player_q.single();
    commands.entity(entity).despawn_recursive();
}

fn move_player(
    mut commands: Commands,
    path: Res<PlayerPath>,
    mut player_q: Query<(&mut Transform, &mut Player)>,
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>
) {
    let (mut transform, mut player) = player_q.single_mut();

    let delta = player.speed * time.delta_seconds();

    if path.waypoints.len() > player.path_index {
        let delta_target = path.waypoints[player.path_index] - transform.translation.xz();

        // this step will get us closer to the goal
        if delta_target.length() > delta {
            let movement = delta_target.normalize() * delta;
            transform.translation += movement.extend(0.0).xzy();
            // copy for ownership reasons
            let y = transform.translation.y;
            transform.look_at(path.waypoints[player.path_index].extend(y).xzy(), Vec3::Y);
        } else {
            player.path_index += 1;
        }
    } else {
        game_state.set(GameState::GameOver);
    }
}