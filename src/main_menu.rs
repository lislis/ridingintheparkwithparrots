use bevy::{prelude::*, app::AppExit};

use crate::*;

#[derive(Component)]
pub struct MenuUIRoot;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct QuitButton;

#[derive(Component)]
pub struct MenuCamera;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_menu_camera)
            .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_menu_camera)
            .add_systems(Update, start_button_clicked.run_if(in_state(GameState::MainMenu)))
            .add_systems(Update, quit_button_clicked.run_if(in_state(GameState::MainMenu)));
    }
}

pub fn spawn_menu_camera(mut commands: Commands) {
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MenuCamera,
        //RaycastPickCamera::default()
    );
    commands.spawn(camera);
}

pub fn despawn_menu_camera(
    mut commands: Commands,
    camera_q: Query<(Entity, &MenuCamera)>
) {
    let (entity, _cam) = camera_q.single();
    commands.entity(entity).despawn_recursive();
}

fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let start_button = spawn_button(&mut commands, &asset_server, "Start riding", Color::LIME_GREEN);
    commands.entity(start_button).insert(StartButton);

    let quit_button = spawn_button(&mut commands, &asset_server, "Exit game", Color::NONE);
    commands.entity(quit_button).insert(QuitButton);

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::DARK_GRAY.into(),
            ..default()
        },
        MenuUIRoot,
        Name::new("Main_Menu"),
    )).with_children(|commands| {
        commands.spawn((
            TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Percent(3.0)),
                    max_width: Val::Percent(70.0),                   
                    ..default()
                },
                text: Text::from_section("Riding in the park with parrots", TextStyle {
                    font: asset_server.load("fonts/Gorditas-Bold.ttf"),
                    font_size: 96.0,
                    color: Color::BLACK,
                    
                }),
                ..default()
            },
        ));
    })
    .add_child(start_button)
    .add_child(quit_button);
}

pub fn spawn_button(
    commands: &mut Commands,
    asset_server: &AssetServer,
    text: &str,
    color: Color,
) -> Entity {
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Percent(45.0), 
                height: Val::Percent(15.0),
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::all(Val::Percent(2.0)),
                ..default()
            },
            background_color: color.into(),
            ..default()
        })
        .with_children(|commands| {
            commands.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Percent(3.0)),
                    ..default()
                },
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: asset_server.load("fonts/Gorditas-Bold.ttf"),
                        font_size: 52.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });
        })
        .id()
}

fn start_button_clicked(
  mut commands: Commands,
  interactions: Query<&Interaction, (With<StartButton>, Changed<Interaction>)>,
  menu_root: Query<Entity, With<MenuUIRoot>>,
  mut game_state: ResMut<NextState<GameState>>,
  mut mouse_input: ResMut<Input<MouseButton>>,
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Pressed) {
            let root_entity = menu_root.single();
            commands.entity(root_entity).despawn_recursive();
            game_state.set(GameState::Gameplay);
            mouse_input.clear();
        }
    }
}

fn quit_button_clicked(
    _commands: Commands,
    interactions: Query<&Interaction, (With<QuitButton>, Changed<Interaction>)>,
    mut exit: EventWriter<AppExit>,
  ) {
      for interaction in &interactions {
          if matches!(interaction, Interaction::Pressed) {
              exit.send(AppExit);
          }
      }
  }