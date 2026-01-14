use crate::core::{
    components::*,
    resources::{GameMode, GameScore},
};
use crate::plugins::audio::{JumpEvent, ScoreEvent};
use crate::plugins::powerups::PowerUpCollectedEvent;
use crate::states::{app_state::AppState, game_state::GameState};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


/// Плагин статистики
pub struct StatisticsPlugin;

impl Plugin for StatisticsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameStatistics>()
            .add_systems(Startup, load_statistics)
            .add_systems(
                Update,
                (
                    track_game_events,
                    track_bird_jumps,
                    track_powerup_collection,
                    track_pipe_passing,
                )
                    .run_if(in_state(AppState::Loaded)),
            )
            .add_systems(
                OnEnter(GameState::Statistics),
                spawn_statistics_screen.run_if(in_state(AppState::Loaded)),
            )
            .add_systems(OnExit(GameState::Statistics), despawn_statistics_screen)
            .add_systems(OnExit(AppState::Loaded), save_statistics);
    }
}

/// Комплексный ресурс игровой статистики
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GameStatistics {
    // Основные метрики
    pub total_games: u32,
    pub total_score: u64,
    pub best_score: u32,
    pub average_score: f32,
    pub total_play_time: f32,

    // Метрики действий
    pub jumps_made: u64,
    pub pipes_passed: u64,
    pub powerups_collected: u64,

    // Статистика по режимам игры
    pub games_by_mode: HashMap<String, u32>,
    pub best_score_by_mode: HashMap<String, u32>,
    pub total_time_by_mode: HashMap<String, f32>,

    // Статистика сессий
    pub current_session_start: Option<f32>,
    pub total_sessions: u32,
    pub longest_session: f32,
    pub current_session_jumps: u32,

    // Дополнительная статистика
    pub total_deaths: u64,
    pub shields_used: u64,
    pub double_score_used: u64,
    pub slow_motion_used: u64,
    pub perfect_runs: u32, // игры без столкновений
    pub games_over_10: u32,
    pub games_over_25: u32,
    pub games_over_50: u32,
}

impl Default for GameStatistics {
    fn default() -> Self {
        Self {
            total_games: 0,
            total_score: 0,
            best_score: 0,
            average_score: 0.0,
            total_play_time: 0.0,
            jumps_made: 0,
            pipes_passed: 0,
            powerups_collected: 0,
            games_by_mode: HashMap::new(),
            best_score_by_mode: HashMap::new(),
            total_time_by_mode: HashMap::new(),
            current_session_start: None,
            total_sessions: 0,
            longest_session: 0.0,
            current_session_jumps: 0,
            total_deaths: 0,
            shields_used: 0,
            double_score_used: 0,
            slow_motion_used: 0,
            perfect_runs: 0,
            games_over_10: 0,
            games_over_25: 0,
            games_over_50: 0,
        }
    }
}

impl GameStatistics {
    /// Обновляет средний счёт
    pub fn update_average_score(&mut self) {
        if self.total_games > 0 {
            self.average_score = self.total_score as f32 / self.total_games as f32;
        }
    }

    /// Начинает новую игровую сессию
    pub fn start_session(&mut self, current_time: f32) {
        self.current_session_start = Some(current_time);
        self.total_sessions += 1;
        self.current_session_jumps = 0;
    }

    /// Завершает текущую сессию
    pub fn end_session(&mut self, current_time: f32) {
        if let Some(start_time) = self.current_session_start {
            let session_duration = current_time - start_time;
            self.total_play_time += session_duration;
            if session_duration > self.longest_session {
                self.longest_session = session_duration;
            }
            self.current_session_start = None;
        }
    }

    /// Регистрирует начало игры
    pub fn start_game(&mut self, mode: &GameMode) {
        self.total_games += 1;
        let mode_str = format!("{:?}", mode);
        *self.games_by_mode.entry(mode_str.clone()).or_insert(0) += 1;
    }

    /// Регистрирует окончание игры
    pub fn end_game(&mut self, score: u32, mode: &GameMode, was_perfect: bool) {
        self.total_score += score as u64;

        if score > self.best_score {
            self.best_score = score;
        }

        let mode_str = format!("{:?}", mode);
        let best_mode_score = self.best_score_by_mode.entry(mode_str.clone()).or_insert(0);
        if score > *best_mode_score {
            *best_mode_score = score;
        }

        // Обновляем пороговые достижения
        if score >= 50 {
            self.games_over_50 += 1;
        }
        if score >= 25 {
            self.games_over_25 += 1;
        }
        if score >= 10 {
            self.games_over_10 += 1;
        }

        if was_perfect {
            self.perfect_runs += 1;
        }

        self.update_average_score();
    }

    /// Регистрирует прыжок
    pub fn register_jump(&mut self) {
        self.jumps_made += 1;
        self.current_session_jumps += 1;
    }

    /// Регистрирует прохождение трубы
    pub fn register_pipe_passed(&mut self) {
        self.pipes_passed += 1;
    }

    /// Регистрирует сбор power-up
    pub fn register_powerup_collected(&mut self, power_type: PowerUpType) {
        self.powerups_collected += 1;

        match power_type {
            PowerUpType::Shield => self.shields_used += 1,
            PowerUpType::DoubleScore => self.double_score_used += 1,
            PowerUpType::SlowMotion => self.slow_motion_used += 1,
        }
    }

    /// Регистрирует смерть
    pub fn register_death(&mut self) {
        self.total_deaths += 1;
    }
}

/// Система отслеживания игровых событий
pub fn track_game_events(
    time: Res<Time>,
    mut statistics: ResMut<GameStatistics>,
    game_state: Res<State<GameState>>,
    score: Res<GameScore>,
    mode_settings: Res<crate::core::resources::GameModeSettings>,
    bird_query: Query<Entity, With<crate::core::components::Bird>>,
    powerup_query: Query<&PowerUp>,
) {
    let current_time = time.elapsed_secs();

    // Отслеживание сессий
    match game_state.get() {
        GameState::MainMenu => {
            if statistics.current_session_start.is_some() {
                statistics.end_session(current_time);
            }
        }
        GameState::Playing => {
            if statistics.current_session_start.is_none() {
                statistics.start_session(current_time);
                statistics.start_game(&mode_settings.current_mode);
            }
        }
        GameState::GameOver => {
            if statistics.current_session_start.is_some() {
                statistics.end_game(
                    score.0,
                    &mode_settings.current_mode,
                    powerup_query.is_empty(),
                );
                statistics.end_session(current_time);
            }
        }
        _ => {}
    }

    // Регистрируем смерть
    // когда мы заходим в GameOver то птица все равно остается на экране, надо по другому признаку смотреть
    if *game_state.get() == GameState::GameOver && bird_query.is_empty() {
        statistics.register_death();
    }
}

/// Система отслеживания прыжков птицы
pub fn track_bird_jumps(
    mut statistics: ResMut<GameStatistics>,
    mut jump_events: EventReader<JumpEvent>,
) {
    for _event in jump_events.read() {
        statistics.register_jump();
    }
}

/// Система отслеживания сбора power-ups
pub fn track_powerup_collection(
    mut statistics: ResMut<GameStatistics>,
    mut powerup_events: EventReader<PowerUpCollectedEvent>,
) {
    for event in powerup_events.read() {
        statistics.register_powerup_collected(event.power_type);
    }
}

/// Система отслеживания прохождения труб
pub fn track_pipe_passing(
    mut statistics: ResMut<GameStatistics>,
    mut score_events: EventReader<ScoreEvent>,
) {
    for _event in score_events.read() {
        statistics.register_pipe_passed();
    }
}

/// Система отображения экрана статистики
pub fn spawn_statistics_screen(
    mut commands: Commands,
    assets: Res<crate::core::resources::GameAssets>,
    statistics: Res<GameStatistics>,
) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            OnStatisticsScreen,
        ))
        .with_children(|parent| {
            // Заголовок
            parent.spawn((
                Text::new("Статистика"),
                TextFont {
                    font: assets.font.clone(),
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ));

            // Скроллируемый контейнер для статистики
            parent
                .spawn((Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(500.0), // Фиксированная высота для скролла
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Center,
                    overflow: Overflow::scroll_y(), // Вертикальный скролл
                    ..default()
                },))
                .with_children(|scroll_parent| {
                    // Левый столбец
                    scroll_parent
                        .spawn((Node {
                            width: Val::Px(350.0),
                            height: Val::Auto,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            margin: UiRect::right(Val::Px(20.0)),
                            ..default()
                        },))
                        .with_children(|left_column| {
                            // Основная статистика
                            spawn_statistics_section(
                                left_column,
                                &assets,
                                "Основная",
                                &[
                                    &format!("Всего игр: {}", statistics.total_games),
                                    &format!("Лучший счёт: {}", statistics.best_score),
                                    &format!("Средний счёт: {:.1}", statistics.average_score),
                                    &format!("Всего очков: {}", statistics.total_score),
                                    &format!(
                                        "Общее время: {:.1} мин",
                                        statistics.total_play_time / 60.0
                                    ),
                                ],
                            );

                            // Достижения
                            spawn_statistics_section(
                                left_column,
                                &assets,
                                "Достижения",
                                &[
                                    &format!("Идеальные игры: {}", statistics.perfect_runs),
                                    &format!("Игр >10 очков: {}", statistics.games_over_10),
                                    &format!("Игр >25 очков: {}", statistics.games_over_25),
                                    &format!("Игр >50 очков: {}", statistics.games_over_50),
                                ],
                            );
                        });

                    // Правый столбец
                    scroll_parent
                        .spawn((Node {
                            width: Val::Px(350.0),
                            height: Val::Auto,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            margin: UiRect::left(Val::Px(20.0)),
                            ..default()
                        },))
                        .with_children(|right_column| {
                            // Действия
                            spawn_statistics_section(
                                right_column,
                                &assets,
                                "Действия",
                                &[
                                    &format!("Прыжков сделано: {}", statistics.jumps_made),
                                    &format!("Труб пройдено: {}", statistics.pipes_passed),
                                    &format!(
                                        "Power-ups собрано: {}",
                                        statistics.powerups_collected
                                    ),
                                    &format!("Смертей: {}", statistics.total_deaths),
                                ],
                            );

                            // Power-ups статистика
                            spawn_statistics_section(
                                right_column,
                                &assets,
                                "Power-ups",
                                &[
                                    &format!("Щитов использовано: {}", statistics.shields_used),
                                    &format!("Двойных очков: {}", statistics.double_score_used),
                                    &format!("Замедления времени: {}", statistics.slow_motion_used),
                                ],
                            );
                        });
                });

            // Кнопка возврата
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::top(Val::Px(30.0)),
                        ..default()
                    },
                    BorderRadius::all(Val::Px(8.0)),
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    MenuButton,
                    MainMenuButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Назад"),
                        TextFont {
                            font: assets.font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

/// Вспомогательная функция для создания секции статистики
fn spawn_statistics_section(
    parent: &mut ChildBuilder,
    assets: &crate::core::resources::GameAssets,
    title: &str,
    stats: &[&str],
) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                margin: UiRect::vertical(Val::Px(10.0)),
                padding: UiRect::all(Val::Px(15.0)),
                ..default()
            },
            BorderRadius::all(Val::Px(10.0)),
            BackgroundColor(Color::srgb(0.1, 0.1, 0.2)),
        ))
        .with_children(|parent| {
            // Заголовок секции
            parent.spawn((
                Text::new(title),
                TextFont {
                    font: assets.font.clone(),
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.8, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                },
            ));

            // Статистика
            for stat in stats {
                parent.spawn((
                    Text::new(stat.to_string()),
                    TextFont {
                        font: assets.font.clone(),
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Node {
                        margin: UiRect::bottom(Val::Px(4.0)),
                        ..default()
                    },
                ));
            }
        });
}

/// Система сохранения статистики
pub fn save_statistics(statistics: Res<GameStatistics>) {
    // Сохраняем статистику в JSON файл
    if let Ok(json) = serde_json::to_string_pretty(&*statistics) {
        if let Err(e) = std::fs::write("flappy_bird_statistics.json", json) {
            eprintln!("Ошибка сохранения статистики: {}", e);
        } else {
            println!("Статистика успешно сохранена в flappy_bird_statistics.json");
        }
    }
}

/// Система загрузки статистики
pub fn load_statistics(mut commands: Commands) {
    if let Ok(json) = std::fs::read_to_string("flappy_bird_statistics.json") {
        if let Ok(statistics) = serde_json::from_str::<GameStatistics>(&json) {
            commands.insert_resource(statistics);
            println!("Статистика успешно загружена из файла");
        } else {
            eprintln!("Ошибка парсинга файла статистики, используются значения по умолчанию");
        }
    } else {
        println!("Файл статистики не найден, создается новая статистика");
    }
}

/// Система очистки UI статистики
pub fn despawn_statistics_screen(
    mut commands: Commands,
    query: Query<Entity, With<OnStatisticsScreen>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

