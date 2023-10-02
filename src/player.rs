use bevy::prelude::*;
use bevy::math::Vec3Swizzles;

use crate::*;

#[derive(Resource)]
pub struct PlayerPath {
    waypoints: Vec<Vec2>
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player {
    pub balance: f32,
    pub speed: f32,
    pub path_index: usize
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Player>()
        //.register_type::<PlayerInfo>()
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
        .add_systems(Update, camera_controls.run_if(in_state(GameState::Gameplay)));
    }
}

fn disrupt_player(
    mut commands: Commands,
    mut player_q: Query<&mut Player>,
) {
    let mut player = player_q.single();
    // @todo random value to throw off balance
    // some kind of debounce or timer
    info!("Balance is {:?}, ", player.balance);
}

fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut parent_q: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    let (mut transform, player) = parent_q.single_mut();
    
    let mut forward = transform.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut left = transform.left();
    left.y = 0.0;
    left = left.normalize();

    let speed = player.speed;
    let rotate_speed = 0.7;
    
    if keyboard.pressed(KeyCode::W) {
        transform.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        transform.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::A) {
        transform.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::D) {
        transform.translation -= left * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::Q) {
        transform.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds())
    }
    if keyboard.pressed(KeyCode::E) {
        transform.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds())
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    _game_assets: Res<GameAssets>,
) {
    let player = (
        SpatialBundle::from_transform(Transform::from_xyz(1.0, 1.0, 1.0)),
        Player {
            balance: 90.0,
            path_index: 0,
            speed: 1.0,
        },
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
                .with_scale(Vec3::new(0.1, 0.8, 0.1)),
            ..default()
        }, 
        NotShadowCaster,
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
                        .with_scale(Vec3::new(0.2, 0.2, 0.2)),
                    ..default()
                },
                Parrot {
                    health: 3,
                },
                Name::new(name)
            )
        };

        for i in 0..=3 {
            let x = -0.5 + (i as f32 * 0.3);
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
        info!("END OF GAME");
        game_state.set(GameState::MainMenu);
    }
}