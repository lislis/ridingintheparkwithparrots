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
        .register_type::<Target>()
        .register_type::<Health>()
        .register_type::<Bullet>()
        .add_systems(PreStartup, asset_loading)
        .add_systems(Startup, (spawn_camera, spawn_basic_scene))
        .add_systems(Update, (tower_shooting, bullet_despawn, move_target, move_bullets))
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

#[derive(Component, Reflect, Default)]
pub struct Tower {
    shooting_timer: Timer,
    bullet_offset: Vec3,
}

fn tower_shooting(
    mut commands: Commands,
    bullet_assets: Res<GameAssets>,
    mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
    targets: Query<&GlobalTransform, With<Target>>,
    time: Res<Time>
) {
    for (tower_ent, mut tower, transform) in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let bullet_spawn: Vec3 = transform.translation() + tower.bullet_offset;

            let direction = targets
                .iter()
                .min_by_key(|target_transform| {
                    bevy::utils::FloatOrd(Vec3::distance(target_transform.translation(), bullet_spawn))
                })
                .map(|closes_target| closes_target.translation() - bullet_spawn);

            if let Some(direction) = direction {
                commands.entity(tower_ent).with_children(|commands| {
                    let bullet = (SceneBundle {
                        scene: bullet_assets.bullet_scene.clone(),
                        //mesh: meshes.add(Mesh::from(shape::Cube::new(0.1))),
                        //material: materials.add(Color::rgb(0.87, 0.44, 0.42).into()),
                        transform: Transform::from_translation(tower.bullet_offset),
                        ..default()
                    }, 
                    Lifetime { timer: Timer::from_seconds(1000.5, TimerMode::Once) },
                    Bullet {
                        direction,
                        speed: 2.5
                    },
                    Name::new("Bullet"));
                    commands.spawn(bullet);
                });
            } 
            
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


#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Target {
    speed: f32
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Health {
    value: i32
}

fn move_target(
    mut targets: Query<(&Target, &mut Transform)>,
    time: Res<Time>
) {
    for (target, mut transform) in &mut targets {
        transform.translation.x += target.speed * time.delta_seconds();
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Bullet {
    direction: Vec3,
    speed: f32
}

fn move_bullets(
    mut bullets: Query<(&Bullet, &mut Transform)>,
    time: Res<Time>,
) {
    for (bullet, mut transform) in &mut bullets {
        transform.translation += bullet.direction.normalize() * bullet.speed * time.delta_seconds();
    }
}