use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::*;

#[derive(Component, Reflect, Default)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_offset: Vec3,
}

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>()
        .add_systems(Update, tower_shooting)
        .add_systems(Update, build_tower);
    }
}

fn tower_shooting(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
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
                        scene: game_assets.tomato_scene.clone(),
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

fn build_tower(
    mut commands: Commands,
    selection: Query<(Entity, &PickSelection, &Transform)>,
    keyboard: Res<Input<KeyCode>>,
    assets: Res<GameAssets>
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for (entity, selection, transform) in &selection {
            if selection.is_selected {
                commands.entity(entity).despawn_recursive();
                spawn_tomato_tower(&mut commands, &assets, transform.translation);
            }
        }
    }
}

fn spawn_tomato_tower(
    commands: &mut Commands,
    assets: &GameAssets,
    position: Vec3
) -> Entity {
    commands.spawn((
        SpatialBundle::from_transform(Transform::from_translation(position)),
        Tower {
            shooting_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            bullet_offset: Vec3::new(0.0, 0.6, 0.0)
        },
        Name::new("Tomato_Tower"),
    )).with_children(|commands| {
        commands.spawn((SceneBundle {
            scene: assets.tomato_tower_scene.clone(),
            transform: Transform::from_xyz(0.0, -0.8, 0.0),
            ..default()
        }, PickableBundle::default()));
    })
    .id()
}