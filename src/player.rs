use bevy::prelude::*;
//use bevy_third_person_camera::*;

use crate::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct PlayerInfo {
    pub balance: f32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Player>()
        .register_type::<PlayerInfo>()
        .add_systems(OnEnter(GameState::Gameplay), spawn_player)
        .add_systems(Update, log_player_balance.run_if(in_state(GameState::Gameplay)))
        .add_systems(Update, camera_controls.run_if(in_state(GameState::Gameplay)));
    }
}

fn log_player_balance(
    mut _commands: Commands,
    player_q: Query<&PlayerInfo>
) {
    let player = player_q.single();
    info!("Balance is {:?}, ", player.balance);
}

fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut parent_q: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player = parent_q.single_mut();
    
    let mut forward = player.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut left = player.left();
    left.y = 0.0;
    left = left.normalize();

    let speed = 5.0;
    let rotate_speed = 0.7;
    
    if keyboard.pressed(KeyCode::W) {
        player.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        player.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::A) {
        player.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::D) {
        player.translation -= left * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::Q) {
        player.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds())
    }
    if keyboard.pressed(KeyCode::E) {
        player.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds())
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    _game_assets: Res<GameAssets>,
) {
    let player_wrapper = (
        SpatialBundle::from_transform(Transform::from_xyz(1.0, 1.0, 1.0)),
        Player,
        Name::new("Player")
    );

    let player_info = (
        PlayerInfo {
            balance: 90.0
        },
        Name::new("PlayerInfo")
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

    commands.spawn(player_wrapper).with_children(|commands| {
        commands.spawn(camera_player);
        commands.spawn(player_info);
        commands.spawn(handlebar);
    });
}