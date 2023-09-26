use bevy::{prelude::*, reflect};
// use bevy_third_person_camera::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

// mod player;
// mod camera;
// mod world;

// use player::PlayerPlugin;
// use camera::CameraPlugin;
// use world::WorldPlugin;

pub const WIDTH: f32 = 720.0;
pub const HEIGHT: f32 = 1280.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "I am a window!".into(),
                    resolution: (1280.0, 720.).into(),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            //DefaultPlugins,  
        //    PlayerPlugin, 
        //    CameraPlugin, 
        //    WorldPlugin,
        //    ThirdPersonCameraPlugin,
            WorldInspectorPlugin::new()
        ))
        .register_type::<Tower>()
        .register_type::<Lifetime>()
        .add_systems(PreStartup, asset_loading)
        .add_systems(Startup, (spawn_camera, spawn_basic_scene))
        .add_systems(Update, (tower_shooting, bullet_despawn))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };
    commands.spawn(camera);
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let floor = (PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(5.0))),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    }, Name::new("Floor"));

    let cube = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        }, 
        Name::new("Cube"), 
        Tower {
            shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        });

    let pointlight = (PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    }, Name::new("PointLight") );

    commands.spawn(pointlight);
    commands.spawn(floor);
    commands.spawn(cube);
}

#[derive(Component, Reflect, Default)]
pub struct Tower {
    shooting_timer: Timer,
}

fn tower_shooting(
    mut commands: Commands,
    bullet_assets: Res<GameAssets>,
    mut towers: Query<&mut Tower>,
    time: Res<Time>
) {
    for mut tower in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let spawn_transform = Transform::from_xyz(0.0, 0.7, 0.6)
            .with_rotation(Quat::from_rotation_y(-std::f32::consts::PI / 2.0));

            let bullet = (SceneBundle {
                scene: bullet_assets.bullet_scene.clone(),
                //mesh: meshes.add(Mesh::from(shape::Cube::new(0.1))),
                //material: materials.add(Color::rgb(0.87, 0.44, 0.42).into()),
                transform: spawn_transform,
                ..default()
            }, 
            Lifetime { timer: Timer::from_seconds(0.5, TimerMode::Once) },
            Name::new("Bullet"));
            commands.spawn(bullet);
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Lifetime {
    timer: Timer
}

fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut bullets {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bullet_scene: assets.load("Tomato.glb#Scene0"),
    });
}

#[derive(Resource)]
pub struct GameAssets {
    bullet_scene: Handle<Scene>,
}