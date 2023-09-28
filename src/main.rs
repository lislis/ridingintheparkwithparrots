use bevy::{prelude::*, reflect, pbr::NotShadowCaster};
// use bevy_third_person_camera::*;
use bevy_inspector_egui::{quick::WorldInspectorPlugin, bevy_inspector::hierarchy::SelectedEntities};
use bevy_mod_picking::prelude::*;

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
        .add_plugins(DefaultPickingPlugins
            .build()
            .disable::<DebugPickingPlugin>())
        .add_systems(PreStartup, asset_loading)
        .add_systems(Startup, (spawn_camera, spawn_basic_scene))
        .add_systems(Update, camera_controls)
        .add_systems(Update, what_is_selected)
        .add_systems(Startup, create_ui)
        .add_plugins((TowerPlugin, TargetPlugin, BulletPlugin))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        RaycastPickCamera::default()
    );
    commands.spawn(camera);
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_assets: Res<GameAssets>,
) {
    let floor = (PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.0))),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    }, Name::new("Floor"));

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
        SceneBundle {
            scene: game_assets.target_scene.clone(),
            transform: Transform::from_xyz(-2.0, 0.2, 1.5),
            ..default()
        },
        Target { speed: 0.3 },
        Health { value: 3 },
        Name::new("Dummy target1") );

    let some_cube2 = (
        SceneBundle {
            scene: game_assets.target_scene.clone(),
            transform: Transform::from_xyz(-4.0, 0.2, 1.5),
            ..default()
        },
        Target { speed: 0.3 },
        Health { value: 3 },
        Name::new("Dummy target2") );

    // TOWER 
    let default_collider_color = materials.add(Color::rgba(0.3, 0.5, 0.3, 0.3).into());
    let selected_collider_color = materials.add(Color::rgba(0.3, 0.9, 0.3, 0.9).into());
    
    commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.8, 0.0)),
        meshes.add(shape::Capsule::default().into()),
        Highlight {
            hovered: Some(HighlightKind::Fixed(selected_collider_color.clone())),
            pressed: Some(HighlightKind::Fixed(selected_collider_color.clone())),
            selected: Some(HighlightKind::Fixed(selected_collider_color))
        },
        default_collider_color,
        NotShadowCaster,
        PickableBundle::default(),
        RaycastPickTarget::default(),
        Name::new("Tower_Base"),
    )).with_children(|commands| {
        commands.spawn((SceneBundle {
            scene: game_assets.tower_base_scene.clone(),
            transform: Transform::from_xyz(0.0, -0.8, 0.0),
            ..default()
        }, PickableBundle::default()));
    });

    commands.spawn(pointlight);
    commands.spawn(floor);
    commands.spawn(some_cube1);
    commands.spawn(some_cube2);
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        tower_base_scene: assets.load("TowerBase.glb#Scene0"),
        tomato_tower_scene: assets.load("TomatoTower.glb#Scene0"),
        tomato_scene: assets.load("Tomato.glb#Scene0"),
        target_scene: assets.load("Target.glb#Scene0"),
    });
}

#[derive(Resource)]
pub struct GameAssets {
    tower_base_scene: Handle<Scene>,
    tomato_tower_scene: Handle<Scene>,
    tomato_scene: Handle<Scene>,
    target_scene: Handle<Scene>
}

fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_q: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_q.single_mut();
    let mut forward = camera.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut left = camera.left();
    left.y = 0.0;
    left = left.normalize();

    let speed = 3.0;
    let rotate_speed = 0.3;
    
    if keyboard.pressed(KeyCode::W) {
        camera.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::A) {
        camera.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::D) {
        camera.translation -= left * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds())
    }
    if keyboard.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds())
    }
}

fn what_is_selected(selection: Query<(&Name, &PickSelection)>) {
    for (name, selection) in &selection {
        //info!("{:?} is selected: {:?}", name, selection);
        if selection.is_selected {
            info!("HEllo {:?} is selected: {:?}", name, selection);
        }
    }
}

fn create_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let button_icons = [
      asset_server.load("tomato_tower.png"),
      asset_server.load("potato_tower.png"),
      asset_server.load("cabbage_tower.png"),
    ];
    let towers = [TowerType::Tomato, TowerType::Potato, TowerType::Cabbage];
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        }, 
        TowerUIRoot)
    ).with_children(|commands| {
        for i in 0..3 {
            commands.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(15.0 * 9.0 / 16.0),
                        height: Val::Percent(15.0),
                        align_self: AlignSelf::FlexStart,
                        margin: UiRect::all(Val::Percent((2.0))),
                        ..default()
                    },
                    image: button_icons[i].clone().into(),
                    ..default()
                }, 
                towers[i]
                )
            );
        }
    });
}

#[derive(Component)]
pub struct TowerUIRoot;

#[derive(Component, Clone, Copy, Debug)]
pub enum TowerType {
    Tomato,
    Potato,
    Cabbage
}