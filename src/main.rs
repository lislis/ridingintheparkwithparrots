use bevy::{prelude::*, reflect};
// use bevy_third_person_camera::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

// mod player;
// mod camera;
// mod world;

// use player::PlayerPlugin;
// use camera::CameraPlugin;
// use world::WorldPlugin;

mod bullet;
mod target;
mod tower;

pub use bullet::*;
pub use target::*;
pub use tower::*;

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
        .add_systems(PreStartup, asset_loading)
        .add_systems(Startup, (spawn_camera, spawn_basic_scene))
        .add_plugins((TowerPlugin, TargetPlugin, BulletPlugin))
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
            bullet_offset: Vec3::new(0.0, 0.6, 0.0),
        });

    let pointlight = (PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    }, Name::new("PointLight"));

    let some_cube1 = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(0.4))),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(-2.0, 0.2, 1.5),
            ..default()
        },
        Target { speed: 0.3 },
        Health { value: 3 }, 
        Name::new("Dummy target1") );

    let some_cube2 = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(0.4))),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(-4.0, 0.2, 1.5),
            ..default()
        },
        Target { speed: 0.3 },
        Health { value: 3 }, 
        Name::new("Dummy target2") );

    commands.spawn(pointlight);
    commands.spawn(floor);
    commands.spawn(cube);
    commands.spawn(some_cube1);
    commands.spawn(some_cube2);
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
