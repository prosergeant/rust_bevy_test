use bevy::prelude::*;

// Bird компонент определён в bird.rs, но для использования в других модулях
// реэкспортируем его здесь
pub use crate::plugins::bird::Bird;

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

#[derive(Component)]
pub struct Scrollable;

#[derive(Component)]
pub struct MenuButton;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct ExitButton;

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct MainMenuButton;

#[derive(Component)]
pub struct SettingsButton;

#[derive(Component)]
pub struct GameModeSelectionButton;

// Power-ups компоненты
#[derive(Component)]
pub struct PowerUp {
    pub power_type: PowerUpType,
    pub effect_duration: f32,
}

#[derive(Component)]
pub struct ActiveShield {
    pub timer: Timer,
}

#[derive(Component)]
pub struct DoubleScore {
    pub timer: Timer,
}

#[derive(Component)]
pub struct SlowMotion {
    pub timer: Timer,
}

#[derive(Component)]
pub struct PowerUpIndicator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerUpType {
    Shield,
    DoubleScore,
    SlowMotion,
}

// Игровые режимы компоненты
#[derive(Component)]
pub struct GameModeButton;

#[derive(Component)]
pub struct ClassicModeButton;

#[derive(Component)]
pub struct TimeAttackModeButton;

#[derive(Component)]
pub struct ZenModeButton;

#[derive(Component)]
pub struct SurvivalModeButton;

#[derive(Component)]
pub struct OnGameModeSelectionScreen;

#[derive(Component)]
pub struct OnGameModeUI;

#[derive(Component)]
pub struct GameModeTimerText;

#[derive(Component)]
pub struct GameModeInfoText;

#[derive(Component)]
pub struct StatisticsButton;

#[derive(Component)]
pub struct OnStatisticsScreen;
