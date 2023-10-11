use bevy::{prelude::*, pbr::{CascadeShadowConfigBuilder}};
use bevy_inspector_egui::{quick::WorldInspectorPlugin};
use bevy_sprite3d::*;
use bevy_rand::prelude::*;
use bevy_prng::ChaCha8Rng;
use rand::prelude::Rng;
use bevy_asset_loader::prelude::*;
use bevy_mod_reqwest::*;
use bevy_serial::{SerialPlugin, SerialReadEvent};

use std::f32::consts::PI;

mod main_menu;
mod game_over;
mod player;
mod parrot;
mod controller;
mod level;
mod score;

pub use player::*;
pub use parrot::*;
pub use controller::*;
pub use level::*;
pub use main_menu::*;
pub use game_over::*;
pub use score::*;

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Riding in the park with parrots".into(),
                    resolution: (WIDTH, HEIGHT).into(),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            })
        )
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::MainMenu)
        )
        .add_collection_to_loading_state::<_, GameAssets>(GameState::Loading)
        .add_plugins(ReqwestPlugin)
        .add_plugins(Sprite3dPlugin)
        .add_plugins(EntropyPlugin::<ChaCha8Rng>::default(),)
        //.add_plugins(WorldInspectorPlugin::new())
        .add_plugins(MainMenuPlugin)
        .add_plugins(GameOverPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ParrotPlugin)
        .add_plugins(ControllerPlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(ScorePlugin)
        .run();
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "handlebar.png")]
    handlebar_image: Handle<Image>,
    #[asset(path = "bang.png")]
    bang_image: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 512., tile_size_y = 512.))]
    #[asset(texture_atlas(columns = 4, rows = 1))]
    #[asset(path = "parrot_blue_atlas.png")]
    parrot_blue_atlas: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 512., tile_size_y = 512.))]
    #[asset(texture_atlas(columns = 4, rows = 1))]
    #[asset(path = "parrot_red_atlas.png")]
    parrot_red_atlas: Handle<TextureAtlas>,
    #[asset(path = "rotation_indicator.png")]
    rotation_indicator: Handle<Image>,
    #[asset(path = "handle_indicator.png")]
    handle_indicator: Handle<Image>,
}


#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    Gameplay,
    GameOver,
}