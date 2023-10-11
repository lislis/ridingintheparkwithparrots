use crate::*;

pub struct ScorePlugin;

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Score {
    pub history: Vec<usize>
}

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Score>()
        .add_systems(Update, add_score)
        .add_systems(OnEnter(GameState::MainMenu), setup_score);
    }
}

fn add_score(
    mut commands: Commands,
    mut game_over_event_reader: EventReader<GameOverEvent>,
    mut score_q: Query<&mut Score>
) {
    for event in game_over_event_reader.iter() {
        if let Ok(mut score) = score_q.get_single_mut() {
            score.history.push(event.0);
        }
    }
}

fn setup_score(
    mut commands: Commands
) {
    commands.spawn((Score {
        history: vec!(),
    }, Name::new("Score")));
}

