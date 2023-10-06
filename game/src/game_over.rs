use bevy::{prelude::*, app::AppExit};

use crate::*;

#[derive(Component)]
pub struct MainMenuButton;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), spawn_menu_camera)
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over)
            .add_systems(OnExit(GameState::GameOver), despawn_menu_camera)
            .add_systems(Update, main_menu_button_clicked.run_if(in_state(GameState::GameOver)));
    }
}

fn spawn_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    parrots_q: Query<&Parrot>,
) {
    let main_menu_button = spawn_button(&mut commands, &asset_server, "Main menu", Color::LIME_GREEN);
    commands.entity(main_menu_button).insert(MainMenuButton);

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
        Name::new("Game_Over"),
    )).with_children(|commands| {
        commands.spawn((
            TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Percent(3.0)),
                    max_width: Val::Percent(70.0),                   
                    ..default()
                },
                text: Text::from_section(format!("Your ride is over, with {} parrots left.", parrots_q.iter().len()), TextStyle {
                    font: asset_server.load("fonts/Gorditas-Bold.ttf"),
                    font_size: 96.0,
                    color: Color::BLACK,
                    
                }),
                ..default()
            },
        ));
    })
    .add_child(main_menu_button);
}

fn main_menu_button_clicked(
  mut commands: Commands,
  interactions: Query<&Interaction, (With<MainMenuButton>, Changed<Interaction>)>,
  menu_root: Query<Entity, With<MenuUIRoot>>,
  mut game_state: ResMut<NextState<GameState>>,
  mut mouse_input: ResMut<Input<MouseButton>>,
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Pressed) {
            let root_entity = menu_root.single();
            commands.entity(root_entity).despawn_recursive();
            game_state.set(GameState::MainMenu);
            mouse_input.clear();
        }
    }
}