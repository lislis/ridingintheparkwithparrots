use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_mod_picking::prelude::*;

use crate::*;

#[derive(Component, Reflect, Default)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_offset: Vec3,
}

#[derive(InspectorOptions, Component, Clone, Copy, Debug)]
pub enum TowerType {
    Tomato,
    Potato,
    Cabbage
}

#[derive(Component)]
pub struct TowerUIRoot;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>()
        .add_systems(Update, tower_shooting.run_if(in_state(GameState::Gameplay)))
        //.add_systems(Update, build_tower)
        .add_systems(Update, create_ui_on_selection.run_if(in_state(GameState::Gameplay)))
        .add_systems(Update, tower_button_clicked.run_if(in_state(GameState::Gameplay)));
    }
}

fn tower_shooting(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut towers: Query<(Entity, &mut Tower, &TowerType, &GlobalTransform)>,
    targets: Query<&GlobalTransform, With<Target>>,
    time: Res<Time>
) {
    for (tower_ent, mut tower, tower_type, transform) in &mut towers {
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
                let (model, bullet) = tower_type.get_bullet(direction, &game_assets);
                commands.entity(tower_ent).with_children(|commands| {
                    let bullet = (SceneBundle {
                        scene: model,
                        transform: Transform::from_translation(tower.bullet_offset),
                        ..default()
                    }, 
                    Lifetime { timer: Timer::from_seconds(10.0, TimerMode::Once) },
                    bullet,
                    Name::new("Bullet"));
                    commands.spawn(bullet);
                });
            } 
            
        }
    }
}

// fn build_tower(
//     mut commands: Commands,
//     selection: Query<(Entity, &PickSelection, &Transform)>,
//     keyboard: Res<Input<KeyCode>>,
//     assets: Res<GameAssets>
// ) {
//     if keyboard.just_pressed(KeyCode::Space) {
//         for (entity, selection, transform) in &selection {
//             if selection.is_selected {
//                 commands.entity(entity).despawn_recursive();
//                 spawn_tower(&mut commands, &assets, transform.translation);
//             }
//         }
//     }
// }

impl TowerType {
    fn get_tower(&self, assets: &GameAssets) -> (Handle<Scene>, Tower) {
        match self {
            TowerType::Tomato => (
                assets.tomato_tower_scene.clone(),
                Tower {
                    shooting_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                    bullet_offset: Vec3::new(0.0, 0.6, 0.0),
                }
            ),
            TowerType::Potato => (
                assets.potato_tower_scene.clone(),
                Tower {
                    shooting_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                    bullet_offset: Vec3::new(0.0, 0.6, 0.0),
                }
            ),
            TowerType::Cabbage => (
                assets.cabbage_tower_scene.clone(),
                Tower {
                    shooting_timer: Timer::from_seconds(0.8, TimerMode::Repeating),
                    bullet_offset: Vec3::new(0.0, 0.6, 0.0),
                }
            ),
        }
    }
}

impl TowerType {
    fn get_bullet(&self, direction: Vec3, assets: &GameAssets) -> (Handle<Scene>, Bullet) {
        match self {
            TowerType::Tomato => (
                assets.tomato_scene.clone(),
                Bullet {
                    direction,
                    speed: 3.5,
                }
            ),
            TowerType::Potato => (
                assets.potato_scene.clone(),
                Bullet {
                    direction,
                    speed: 6.5,
                }
            ),
            TowerType::Cabbage => (
                assets.cabbage_scene.clone(),
                Bullet {
                    direction,
                    speed: 1.5,
                }
            ),
        }
    }
}

fn spawn_tower(
    commands: &mut Commands,
    assets: &GameAssets,
    position: Vec3,
    tower_type: TowerType
) -> Entity {
    let (tower_scene, tower) = tower_type.get_tower(assets);
    commands.spawn((
        SpatialBundle::from_transform(Transform::from_translation(position)),
        tower_type,
        tower,
        Name::new("Tower"),
    )).with_children(|commands| {
        commands.spawn((SceneBundle {
            scene: tower_scene,
            transform: Transform::from_xyz(0.0, -0.8, 0.0),
            ..default()
        }, PickableBundle::default()));
    })
    .id()
}

fn tower_button_clicked(
    interaction: Query<(&Interaction, &TowerType), Changed<Interaction>>,
    mut commands: Commands,
    selection: Query<(Entity, &PickSelection, &Transform)>,
    assets: Res<GameAssets>,
) {
    for (interaction, tower_type) in &interaction {
        if matches!(interaction, Interaction::Pressed) {
            for (entity, selection, transform) in &selection {
                if selection.is_selected {
                    //Remove the base model/hitbox
                    commands.entity(entity).despawn_recursive();

                    spawn_tower(&mut commands, &assets, transform.translation, *tower_type);
                }
            }
        }
    }
}


fn create_ui_on_selection(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    selections: Query<&PickSelection>,
    root: Query<Entity, With<TowerUIRoot>>,
) {
    let at_least_one_selected = selections.iter().any(|selection| selection.is_selected );
    match root.get_single() {
        Ok(root) => {
            if !at_least_one_selected {
                commands.entity(root).despawn_recursive();
            }
        },
        Err(bevy::ecs::query::QuerySingleError::NoEntities(..)) => {
            if at_least_one_selected {
                create_ui(&mut commands, &asset_server);
            }
        },
        _ => unreachable!("Too many ui Tower roots!"),
    }
}

fn create_ui(
    commands: &mut Commands,
    asset_server: &AssetServer
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
                        align_self: AlignSelf::FlexEnd,
                        margin: UiRect::all(Val::Percent(2.0)),
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