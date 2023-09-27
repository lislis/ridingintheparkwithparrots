use bevy::prelude::*;

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
        .add_systems(Update, tower_shooting);
    }
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
