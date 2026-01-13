use crate::core::resources::GameAssets;
use crate::core::utils::despawn_entities;
use crate::states::app_state::AppState;
use bevy::prelude::*;

/// Маркер для аудио сущностей
#[derive(Component)]
pub struct AudioEntity;

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
            )
            .add_systems(
                OnExit(crate::states::game_state::GameState::GameOver),
                despawn_entities::<AudioEntity>,
            )
            .add_systems(
                OnEnter(crate::states::game_state::GameState::PreGame),
                despawn_entities::<AudioEntity>,
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

/// Воспроизведение звука прыжка
pub fn play_jump_sounds(
    mut commands: Commands,
    mut jump_events: EventReader<JumpEvent>,
    assets: Res<GameAssets>,
) {
    for _event in jump_events.read() {
        commands.spawn((
            AudioPlayer::new(assets.jump_sound.clone()),
            PlaybackSettings::DESPAWN,
            AudioEntity,
        ));
    }
}

/// Воспроизведение звука получения очков
pub fn play_score_sounds(
    mut commands: Commands,
    mut score_events: EventReader<ScoreEvent>,
    assets: Res<GameAssets>,
) {
    for _event in score_events.read() {
        commands.spawn((
            AudioPlayer::new(assets.score_sound.clone()),
            PlaybackSettings::DESPAWN,
            AudioEntity,
        ));
    }
}

/// Воспроизведение звука столкновения
pub fn play_collision_sounds(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    assets: Res<GameAssets>,
) {
    for _event in collision_events.read() {
        commands.spawn((
            AudioPlayer::new(assets.hit_sound.clone()),
            PlaybackSettings::DESPAWN,
            AudioEntity,
        ));
    }
}

/// Воспроизведение звука окончания игры
pub fn play_game_over_sounds(
    mut commands: Commands,
    mut game_over_events: EventReader<GameOverEvent>,
    assets: Res<GameAssets>,
) {
    for _event in game_over_events.read() {
        commands.spawn((
            AudioPlayer::new(assets.game_over_sound.clone()),
            PlaybackSettings::DESPAWN,
            AudioEntity,
        ));
    }
}
