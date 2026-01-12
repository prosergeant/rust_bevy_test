use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource)]
pub struct GameAssets {
    pub bird_textures: Vec<Handle<Image>>,
    pub pipe_texture: Handle<Image>,
    pub font: Handle<Font>,
    // Аудио ассеты
    pub jump_sound: Handle<AudioSource>,
    pub score_sound: Handle<AudioSource>,
    pub hit_sound: Handle<AudioSource>,
    pub game_over_sound: Handle<AudioSource>,
}

#[derive(Resource, Default)]
pub struct GameScore(pub u32);

/// Рекорды игры с историей
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct HighScores {
    pub scores: Vec<HighScoreEntry>,
    pub max_entries: usize,
}

impl Default for HighScores {
    fn default() -> Self {
        Self {
            scores: Vec::new(),
            max_entries: 10,
        }
    }
}

/// Одна запись в таблице рекордов
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighScoreEntry {
    pub score: u32,
    pub date: String,
    pub difficulty: String,
}

impl HighScoreEntry {
    pub fn new(score: u32, difficulty: String) -> Self {
        Self {
            score,
            date: chrono::Utc::now().format("%Y-%m-%d %H:%M").to_string(),
            difficulty,
        }
    }
}

/// Ресурс для управления состоянием Game Over UI
#[derive(Resource, Default)]
pub struct GameOverUIState {
    pub timer: f32,
    pub is_visible: bool,
}
