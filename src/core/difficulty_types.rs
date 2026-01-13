use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Уровни сложности игры
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DifficultyLevel {
    Easy,
    Normal,
    #[default]
    Hard,
    Progressive,
}

impl DifficultyLevel {
    /// Возвращает русское название сложности
    pub fn russian_name(self) -> &'static str {
        match self {
            DifficultyLevel::Easy => "Легко",
            DifficultyLevel::Normal => "Нормально",
            DifficultyLevel::Hard => "Сложно",
            DifficultyLevel::Progressive => "Прогрессивная",
        }
    }

    /// Возвращает параметры сложности
    pub fn get_params(self) -> DifficultyParams {
        match self {
            DifficultyLevel::Easy => DifficultyParams {
                pipe_gap: 400.0,
                pipe_speed: 80.0,
                spawn_interval: 2.5,
                pipe_distance: 450.0,
            },
            DifficultyLevel::Normal => DifficultyParams {
                pipe_gap: 300.0,
                pipe_speed: 100.0,
                spawn_interval: 2.0,
                pipe_distance: 400.0,
            },
            DifficultyLevel::Hard => DifficultyParams {
                pipe_gap: 200.0,
                pipe_speed: 300.0,
                spawn_interval: 0.4,
                pipe_distance: 350.0,
            },
            DifficultyLevel::Progressive => DifficultyParams {
                pipe_gap: 350.0,
                pipe_speed: 80.0,
                spawn_interval: 2.5,
                pipe_distance: 400.0,
            },
        }
    }
}

/// Параметры сложности для настроек игрового процесса
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyParams {
    /// Расстояние между трубами по вертикали
    pub pipe_gap: f32,
    /// Скорость движения труб
    pub pipe_speed: f32,
    /// Интервал spawn труб
    pub spawn_interval: f32,
    /// Расстояние между парами труб
    pub pipe_distance: f32,
}

/// Текущие настройки игры
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct DifficultySettings {
    pub current_level: DifficultyLevel,
    pub current_params: DifficultyParams,
}

impl Default for DifficultySettings {
    fn default() -> Self {
        let level = DifficultyLevel::default();
        Self {
            current_level: level,
            current_params: level.get_params(),
        }
    }
}

impl DifficultySettings {
    /// Устанавливает новый уровень сложности
    pub fn set_level(&mut self, level: DifficultyLevel) {
        self.current_level = level;
        self.current_params = level.get_params();
    }

    /// Обновляет параметры для прогрессивной сложности
    pub fn update_progressive_params(&mut self, score: u32) {
        if self.current_level == DifficultyLevel::Progressive {
            let base_params = DifficultyLevel::Easy.get_params();
            let hard_params = DifficultyLevel::Hard.get_params();

            // Прогрессия каждый 1 очок от Easy до Hard
            let progress = score as f32 / 10.0;

            self.current_params.pipe_gap =
                base_params.pipe_gap + (hard_params.pipe_gap - base_params.pipe_gap) * progress;
            self.current_params.pipe_speed = base_params.pipe_speed
                + (hard_params.pipe_speed - base_params.pipe_speed) * progress;
            self.current_params.spawn_interval = base_params.spawn_interval
                + (hard_params.spawn_interval - base_params.spawn_interval) * progress;
            self.current_params.pipe_distance = base_params.pipe_distance
                + (hard_params.pipe_distance - base_params.pipe_distance) * progress;
        }
    }
}

/// Компонент для кнопки выбора сложности
#[derive(Component)]
pub struct DifficultyButton {
    pub level: DifficultyLevel,
}

/// Маркер-компонент для экрана настроек
#[derive(Component)]
pub struct OnSettingsScreen;
