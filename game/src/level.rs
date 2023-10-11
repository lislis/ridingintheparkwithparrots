use crate::*;

pub struct LevelPlugin;

#[derive(Component, Debug, Reflect)]
pub struct Level;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Level>()
        .add_systems(OnEnter(GameState::Gameplay), spawn_basic_scene)
        .add_systems(OnExit(GameState::Gameplay), rm_basic_scene);
    }
}

fn rm_basic_scene(
    mut commands: Commands,
    level_q: Query<Entity, With<Level>>,
) {
    for entity in level_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    _game_assets: Res<GameAssets>,
    mut rng_q: Query<&mut EntropyComponent<ChaCha8Rng>>,
) {
    let floor = (PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(30.0))),
        material: materials.add(Color::YELLOW_GREEN.into()),
        ..default()
    }, Level, Name::new("Floor"));

    let dir_light = (DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 5000.0,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(5.0, 4.0, 0.5),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }.into(),
        ..default()
    }, Level, Name::new("DirectionalLight"));

    
    let mut create_cube = |size: f32, color: Color, xyz: (f32, f32, f32), name: String | -> (PbrBundle, Level, Name) {
        (PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(size))),
            material: materials.add(color.into()),
            transform: Transform::from_xyz(xyz.0, xyz.1, xyz.2),
            ..default()
        },
        Level,
        Name::new(name))
    };
    commands.spawn(create_cube(0.7, Color::DARK_GREEN, (5.0, 0.35, 4.4), "Cube 1".to_string()));
    commands.spawn(create_cube(0.6, Color::GREEN, (2.0, 0.3, 7.4), "Cube 2".to_string()));
    commands.spawn(create_cube(0.8, Color::DARK_GREEN, (-3.0, 0.4, 0.5), "Cube 3".to_string()));
    commands.spawn(create_cube(0.3, Color::BLUE, (-8.0, 0.15, 7.5), "Cube 4".to_string()));
    commands.spawn(create_cube(0.5, Color::DARK_GREEN, (4.2, 0.25, -6.5), "Cube 5".to_string()));
    //commands.spawn(create_cube(0.5, Color::GREEN, (0.0, 0.25, 0.0), "Cube 6".to_string()));

    commands.spawn(dir_light);
    commands.spawn(floor);
}
