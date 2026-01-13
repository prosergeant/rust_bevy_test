use crate::core::difficulty_types::DifficultySettings;
use bevy::prelude::*;

/// Плагин для управления системой сложности
pub struct DifficultyPlugin;

impl Plugin for DifficultyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DifficultySettings>();
    }
}
