use crate::{
    core::{
        components::{Collider, Scrollable},
        difficulty_types::{DifficultyParams, DifficultySettings},
        resources::{ActivePowerUps, GameMode, GameModeSettings, *},
        utils::despawn_entities,
    },
    plugins::audio::{CollisionEvent, GameOverEvent, ScoreEvent},
    plugins::bird::Bird,
    states::game_state::GameState,
};
use bevy::prelude::*;

const PIPE_WIDTH: f32 = 80.0;
const OFFSCREEN_THRESHOLD: f32 = -400.0; // Порог удаления труб

#[derive(Component)]
pub struct Pipe;

#[derive(Resource)]
pub struct PipeSpawner {
    pub timer: Timer,
    pub last_pipe_x: f32,
}

impl Default for PipeSpawner {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            last_pipe_x: 400.0,
        }
    }
}

pub struct PipesPlugin;

impl Plugin for PipesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PipeSpawner>()
            .add_systems(OnEnter(GameState::PreGame), reset_and_spawn_pipes)
            .add_systems(
                Update,
                (
                    move_pipes,
                    check_collisions,
                    score_system,
                    spawn_pipes_continuously,
                    cleanup_offscreen_pipes,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::GameOver), despawn_entities::<Pipe>)
            .add_systems(OnExit(GameState::Playing), reset_pipe_spawner);
    }
}

fn reset_and_spawn_pipes(
    mut commands: Commands,
    assets: Res<GameAssets>,
    windows: Query<&Window>,
    mut spawner: ResMut<PipeSpawner>,
    difficulty: Res<DifficultySettings>,
) {
    // Сбрасываем состояние спавнера
    spawner.last_pipe_x = 400.0;
    spawner.timer.reset();

    let window = windows.single();
    let window_height = window.height();

    for i in 0..3 {
        let pipe_x = 400.0 + i as f32 * difficulty.current_params.pipe_distance;
        spawn_pipe_pair(
            &mut commands,
            &assets,
            window_height,
            pipe_x,
            &difficulty.current_params,
        );
        spawner.last_pipe_x = pipe_x;
    }
}

fn reset_pipe_spawner(mut spawner: ResMut<PipeSpawner>) {
    spawner.last_pipe_x = 400.0;
    spawner.timer.reset();
}

fn spawn_pipe_pair(
    commands: &mut Commands,
    assets: &GameAssets,
    window_height: f32,
    pipe_x: f32,
    params: &DifficultyParams,
) {
    let gap_y = rand::random::<f32>() * 200.0 - 100.0;

    // Верхняя труба
    commands.spawn((
        Sprite {
            image: assets.pipe_texture.clone(),
            custom_size: Some(Vec2::new(PIPE_WIDTH, window_height)),
            ..default()
        },
        Transform {
            translation: Vec3::new(
                pipe_x,
                gap_y + params.pipe_gap / 2.0 + window_height / 2.0,
                0.0,
            ),
            scale: Vec3::new(1.0, -1.0, 1.0), // Переворачиваем
            ..default()
        },
        Pipe,
        Collider {
            size: Vec2::new(PIPE_WIDTH, window_height),
        },
    ));

    // Нижняя труба
    commands.spawn((
        Sprite {
            image: assets.pipe_texture.clone(),
            custom_size: Some(Vec2::new(PIPE_WIDTH, window_height)),
            ..default()
        },
        Transform {
            translation: Vec3::new(
                pipe_x,
                gap_y - params.pipe_gap / 2.0 - window_height / 2.0,
                0.0,
            ),
            ..default()
        },
        Pipe,
        Collider {
            size: Vec2::new(PIPE_WIDTH, window_height),
        },
        Scrollable,
    ));
}

fn spawn_pipes_continuously(
    mut commands: Commands,
    assets: Res<GameAssets>,
    windows: Query<&Window>,
    time: Res<Time>,
    mut spawner: ResMut<PipeSpawner>,
    difficulty: Res<DifficultySettings>,
    active_effects: Res<ActivePowerUps>,
    mode_settings: Res<GameModeSettings>,
) {
    let time_multiplier = if active_effects.slow_motion_active {
        0.3 // Замедление времени влияет на спавн
    } else {
        1.0
    };

    // Применяем множитель сложности от игрового режима
    let adjusted_interval =
        difficulty.current_params.spawn_interval / mode_settings.difficulty_multiplier;

    // Устанавливаем корректный интервал таймера с учетом множителя
    if spawner.timer.duration().as_secs_f32() != adjusted_interval {
        spawner
            .timer
            .set_duration(std::time::Duration::from_secs_f32(
                adjusted_interval.clamp(0.3, 10.0),
            ));
    }

    let adjusted_delta = time.delta().mul_f32(time_multiplier);
    spawner.timer.tick(adjusted_delta);

    if spawner.timer.just_finished() {
        let window = windows.single();
        let window_height = window.height();

        let new_pipe_x = spawner.last_pipe_x + difficulty.current_params.pipe_distance;
        spawn_pipe_pair(
            &mut commands,
            &assets,
            window_height,
            new_pipe_x,
            &difficulty.current_params,
        );
        spawner.last_pipe_x = new_pipe_x;
    }
}

fn cleanup_offscreen_pipes(
    mut commands: Commands,
    pipe_query: Query<(Entity, &Transform), With<Pipe>>,
) {
    for (entity, transform) in &pipe_query {
        if transform.translation.x < OFFSCREEN_THRESHOLD {
            commands.entity(entity).despawn();
        }
    }
}

fn move_pipes(
    mut query: Query<&mut Transform, With<Pipe>>,
    time: Res<Time>,
    difficulty: Res<DifficultySettings>,
    active_effects: Res<ActivePowerUps>,
) {
    let speed_multiplier = if active_effects.slow_motion_active {
        0.3 // Замедление в 3 раза
    } else {
        1.0
    };

    for mut transform in &mut query {
        transform.translation.x -=
            difficulty.current_params.pipe_speed * speed_multiplier * time.delta_secs();
    }
}

fn check_collisions(
    bird_query: Query<&Transform, With<Bird>>,
    pipe_query: Query<(&Transform, Entity), With<Pipe>>,
    collider_query: Query<&Collider>,
    active_effects: Res<ActivePowerUps>,
    mode_settings: Res<GameModeSettings>,
    mut next_state: ResMut<NextState<GameState>>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut game_over_events: EventWriter<GameOverEvent>,
) {
    if let Ok(bird_transform) = bird_query.get_single() {
        let bird_collider = Collider {
            size: Vec2::new(40.0, 40.0),
        };

        // Проверяем столкновения только если не в дзен режиме
        if mode_settings.current_mode != GameMode::Zen {
            for (pipe_transform, _pipe_entity) in &pipe_query {
                // Получаем коллайдер для трубы
                if let Ok(pipe_collider) = collider_query.get(_pipe_entity) {
                    if collide(
                        bird_transform.translation,
                        bird_collider.size,
                        pipe_transform.translation,
                        pipe_collider.size,
                    ) {
                        // Проверяем наличие щита
                        if active_effects.shield_active {
                            // Щит поглощает столкновение, но создаём эффект частиц
                            collision_events.send(CollisionEvent);
                            // Не отправляем GameOverEvent и не меняем состояние
                        } else {
                            // Отправляем звуковые события
                            collision_events.send(CollisionEvent);
                            game_over_events.send(GameOverEvent);
                            next_state.set(GameState::GameOver);
                        }
                        return;
                    }
                }
            }
        } else {
            // В дзен режиме просто проверяем proximity для эффектов, но без GameOver
            for (pipe_transform, _pipe_entity) in &pipe_query {
                if let Ok(_pipe_collider) = collider_query.get(_pipe_entity) {
                    let proximity_threshold = 50.0; // Расстояние для эффектов в дзен режиме
                    let distance =
                        (bird_transform.translation - pipe_transform.translation).length();
                    if distance < proximity_threshold {
                        collision_events.send(CollisionEvent); // только для эффектов
                        break;
                    }
                }
            }
        }
    }
}

fn score_system(
    mut score: ResMut<GameScore>,
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Scrollable>>,
    bird_query: Query<&Transform, With<Bird>>,
    mut score_events: EventWriter<ScoreEvent>,
    active_effects: Res<ActivePowerUps>,
) {
    if let Ok(bird_transform) = bird_query.get_single() {
        for (entity, transform) in &query {
            if transform.translation.x < bird_transform.translation.x - 50.0 {
                // Удваиваем очки если активен DoubleScore
                let points = if active_effects.double_score_active {
                    2
                } else {
                    1
                };
                score.0 += points;
                commands.entity(entity).remove::<Scrollable>();
                score_events.send(ScoreEvent);

                return; // Выходим чтобы избежать множественной обработки в одном кадре
            }
        }
    }
}

fn collide(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> bool {
    let a_min = a_pos.truncate() - a_size / 2.0;
    let a_max = a_pos.truncate() + a_size / 2.0;
    let b_min = b_pos.truncate() - b_size / 2.0;
    let b_max = b_pos.truncate() + b_size / 2.0;

    a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y
}
