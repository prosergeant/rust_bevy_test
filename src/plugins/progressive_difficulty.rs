/// Плагин для прогрессивной сложности
///
/// Этот модуль управляет динамическим изменением сложности
/// в режиме ProgressiveDifficulty, где параметры игры
/// постепенно усложняются каждые 10 очков.
use crate::{
    core::difficulty_types::DifficultySettings, core::resources::GameScore,
    states::game_state::GameState,
};
use bevy::prelude::*;

/// Плагин для прогрессивной сложности
pub struct ProgressiveDifficultyPlugin;

impl Plugin for ProgressiveDifficultyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_progressive_difficulty.run_if(in_state(GameState::Playing)),
        );
    }
}

/// Система обновления прогрессивной сложности
fn update_progressive_difficulty(
    mut difficulty_settings: ResMut<DifficultySettings>,
    game_score: Res<GameScore>,
) {
    // Обновляем только если выбрана прогрессивная сложность
    if difficulty_settings.current_level
        == crate::core::difficulty_types::DifficultyLevel::Progressive
    {
        difficulty_settings.update_progressive_params(game_score.0);
    }
}
