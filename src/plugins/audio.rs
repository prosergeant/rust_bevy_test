use crate::core::resources::GameAssets;
use crate::states::app_state::AppState;
use bevy::prelude::*;

/// Плагин для управления звуковыми эффектами
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<JumpEvent>()
            .add_event::<ScoreEvent>()
            .add_event::<CollisionEvent>()
            .add_event::<GameOverEvent>()
            .add_systems(
                Update,
                (
                    play_jump_sounds,
                    play_score_sounds,
                    play_collision_sounds,
                    play_game_over_sounds,
                )
                    .run_if(in_state(AppState::Loaded)),
            );
    }
}

/// События для звуковых эффектов
#[derive(Event)]
pub struct JumpEvent;

#[derive(Event)]
pub struct ScoreEvent;

#[derive(Event)]
pub struct CollisionEvent;

#[derive(Event)]
pub struct GameOverEvent;

/// Настройка аудио системы
#[allow(dead_code)]
pub fn setup_audio() {
    println!("🔊 Аудио система инициализирована (заглушка)");
    // TODO: Добавить реальную аудио систему когда будем добавлять звуки
}

/// Воспроизведение звука прыжка
pub fn play_jump_sounds(mut jump_events: EventReader<JumpEvent>, _assets: Res<GameAssets>) {
    for _event in jump_events.read() {
        // TODO: Добавить реальное воспроизведение звука
        // bevy_audio пока не поддерживается в этом проекте, но структура готова
    }
}

/// Воспроизведение звука получения очков
pub fn play_score_sounds(mut score_events: EventReader<ScoreEvent>, _assets: Res<GameAssets>) {
    for _event in score_events.read() {
        println!("🔊 Воспроизводится звук получения очка (пока заглушка)");
    }
}

/// Воспроизведение звука столкновения
pub fn play_collision_sounds(
    mut collision_events: EventReader<CollisionEvent>,
    _assets: Res<GameAssets>,
) {
    for _event in collision_events.read() {
        println!("🔊 Воспроизводится звук столкновения (пока заглушка)");
    }
}

/// Воспроизведение звука окончания игры
pub fn play_game_over_sounds(
    mut game_over_events: EventReader<GameOverEvent>,
    _assets: Res<GameAssets>,
) {
    for _event in game_over_events.read() {
        println!("🔊 Воспроизводится звук окончания игры (пока заглушка)");
    }
}

/// Вспомогательные функции для отправки событий
#[allow(dead_code)]
pub fn send_jump_event(mut writer: EventWriter<JumpEvent>) {
    writer.send(JumpEvent);
}

#[allow(dead_code)]
pub fn send_score_event(mut writer: EventWriter<ScoreEvent>) {
    writer.send(ScoreEvent);
}

#[allow(dead_code)]
pub fn send_collision_event(mut writer: EventWriter<CollisionEvent>) {
    writer.send(CollisionEvent);
}

#[allow(dead_code)]
pub fn send_game_over_event(mut writer: EventWriter<GameOverEvent>) {
    writer.send(GameOverEvent);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_events_creation() {
        // Базовый тест для проверки создания событий
        let jump_event = JumpEvent;
        let score_event = ScoreEvent;
        let collision_event = CollisionEvent;
        let game_over_event = GameOverEvent;

        // Просто проверяем, что события создаются без паники
        assert!(true); // Placeholder тест
    }
}
