use bevy::{prelude::*, app::AppExit};

use crate::*;

#[derive(Component)]
pub struct MenuUIRoot;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct QuitButton;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu);
        app.add_systems(Update, start_button_clicked.run_if(in_state(GameState::MainMenu)));
        app.add_systems(Update, quit_button_clicked.run_if(in_state(GameState::MainMenu)));
    }
}

fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let start_button = spawn_button(&mut commands, &asset_server, "Start Game", Color::RED);
    commands.entity(start_button).insert(StartButton);

    let quit_button = spawn_button(&mut commands, &asset_server, "Quit", Color::BLUE);
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
                    ..default()
                },
                text: Text::from_section("Tower Defense", TextStyle {
                    font: asset_server.load("FiraSans-Bold.ttf"),
                    font_size: 96.0,
                    color: Color::BLACK
                }),
                ..default()
            },
        ));
    })
    .add_child(start_button)
    .add_child(quit_button);
}

fn spawn_button(
    commands: &mut Commands,
    asset_server: &AssetServer,
    text: &str,
    color: Color,
) -> Entity {
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Percent(65.0), 
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
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 64.0,
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
  mut game_state: ResMut<NextState<GameState>>  
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Pressed) {
            let root_entity = menu_root.single();
            commands.entity(root_entity).despawn_recursive();
            game_state.set(GameState::Gameplay);
        }
    }
}

fn quit_button_clicked(
    mut commands: Commands,
    interactions: Query<&Interaction, (With<QuitButton>, Changed<Interaction>)>,
    mut exit: EventWriter<AppExit>,
  ) {
      for interaction in &interactions {
          if matches!(interaction, Interaction::Pressed) {
              exit.send(AppExit);
          }
      }
  }