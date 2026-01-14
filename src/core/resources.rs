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
    pub powerup_collect_sound: Handle<AudioSource>,
    pub powerup_spawn_sound: Handle<AudioSource>,
    // Фоновые текстуры
    pub background_layers: Vec<Handle<Image>>,
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

/// Ресурс для управления Power-ups
#[derive(Resource)]
pub struct PowerUpSpawner {
    pub timer: Timer,
}

impl Default for PowerUpSpawner {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(8.0, TimerMode::Repeating),
        }
    }
}

/// Ресурс для отслеживания активных эффектов
#[derive(Resource, Default)]
pub struct ActivePowerUps {
    pub shield_active: bool,
    pub double_score_active: bool,
    pub slow_motion_active: bool,
}

/// Игровые режимы
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Classic,
    TimeAttack,
    Zen,
    Survival,
}

impl Default for GameMode {
    fn default() -> Self {
        GameMode::Classic
    }
}

/// Настройки игровых режимов
#[derive(Resource)]
pub struct GameModeSettings {
    pub current_mode: GameMode,
    pub time_limit: Option<f32>,
    pub target_score: Option<u32>,
    pub lives: Option<u32>,
    pub difficulty_multiplier: f32,
}

impl Default for GameModeSettings {
    fn default() -> Self {
        Self {
            current_mode: GameMode::Classic,
            time_limit: None,
            target_score: None,
            lives: None,
            difficulty_multiplier: 1.0,
        }
    }
}

/// Ресурс для отслеживания времени в режимах с ограничением по времени
#[derive(Resource, Default)]
pub struct GameTimer {
    pub remaining_time: f32,
    pub is_active: bool,
}

/// Ресурс для отслеживания жизней в режиме выживания
#[derive(Resource, Default)]
pub struct SurvivalLives {
    pub current_lives: u32,
    pub max_lives: u32,
}

// Реэкспорт PipeSpawner из плагина pipes
pub use crate::plugins::pipes::PipeSpawner;
