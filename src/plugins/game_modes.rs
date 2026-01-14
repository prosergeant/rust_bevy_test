use bevy::prelude::*;
use bevy::text::{TextColor, TextFont};
use bevy::ui::{
    AlignItems, BorderRadius, FlexDirection, JustifyContent, Node, PositionType, UiRect, Val,
};

use crate::core::components::{
    ClassicModeButton, GameModeButton, GameModeInfoText, GameModeTimerText, MenuButton,
    OnGameModeSelectionScreen, OnGameModeUI, SurvivalModeButton, TimeAttackModeButton,
    ZenModeButton,
};
use crate::core::resources::{
    GameAssets, GameMode, GameModeSettings, GameScore, GameTimer, PipeSpawner, SurvivalLives,
};
use crate::core::utils::despawn_entities;
use crate::plugins::audio::CollisionEvent;
use crate::states::app_state::AppState;
use crate::states::game_state::GameState;

pub struct GameModesPlugin;

impl Plugin for GameModesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameModeSettings>()
            .init_resource::<GameTimer>()
            .init_resource::<SurvivalLives>()
            .add_systems(
                OnEnter(GameState::GameModeSelection),
                spawn_game_mode_selection.run_if(in_state(AppState::Loaded)),
            )
            .add_systems(
                OnExit(GameState::GameModeSelection),
                despawn_entities::<OnGameModeSelectionScreen>,
            )
            .add_systems(
                Update,
                handle_game_mode_selection
                    .run_if(in_state(GameState::GameModeSelection).and(in_state(AppState::Loaded))),
            )
            .add_systems(
                Update,
                (
                    check_time_attack_victory,
                    check_zen_mode_scoring,
                    check_survival_lives,
                    apply_mode_difficulty,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnEnter(GameState::PreGame), setup_game_mode)
            .add_systems(OnEnter(GameState::PreGame), spawn_game_mode_ui)
            .add_systems(
                Update,
                update_game_mode_ui.run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                OnExit(GameState::Playing),
                (reset_game_mode_state, despawn_game_mode_ui),
            );
    }
}

fn spawn_game_mode_selection(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnGameModeSelectionScreen,
        ))
        .with_children(|parent| {
            // Заголовок
            parent.spawn((
                Text::new("Выберите режим игры"),
                TextFont {
                    font: assets.font.clone(),
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
            ));

            // Кнопка классического режима
            spawn_mode_button(
                parent,
                "Классика",
                "Классический режим Flappy Bird",
                &assets,
                ClassicModeButton,
            );

            // Кнопка режима на время
            spawn_mode_button(
                parent,
                "Гонка на время",
                "Наберите максимум очков за 60 секунд",
                &assets,
                TimeAttackModeButton,
            );

            // Кнопка дзен режима
            spawn_mode_button(
                parent,
                "Дзен",
                "Бесконечная игра без столкновений",
                &assets,
                ZenModeButton,
            );

            // Кнопка режима выживания
            spawn_mode_button(
                parent,
                "Выживание",
                "Одна жизнь и растущая сложность",
                &assets,
                SurvivalModeButton,
            );

            // Кнопка назад
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
                    BackgroundColor(Color::srgb(0.5, 0.2, 0.2)),
                    MenuButton,
                    crate::core::components::MainMenuButton,
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

fn spawn_mode_button(
    parent: &mut ChildBuilder,
    title: &str,
    description: &str,
    assets: &GameAssets,
    button_component: impl Component,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(400.0),
                height: Val::Px(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
            BorderRadius::all(Val::Px(12.0)),
            BackgroundColor(Color::srgb(0.2, 0.3, 0.4)),
            GameModeButton,
            button_component,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(title),
                TextFont {
                    font: assets.font.clone(),
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
            ));

            parent.spawn((
                Text::new(description),
                TextFont {
                    font: assets.font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));
        });
}

fn handle_game_mode_selection(
    _commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<GameModeButton>),
    >,
    classic_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<ClassicModeButton>),
    >,
    time_attack_query: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<Button>,
            With<TimeAttackModeButton>,
        ),
    >,
    zen_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<ZenModeButton>)>,
    survival_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<SurvivalModeButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut mode_settings: ResMut<GameModeSettings>,
) {
    // Обрабатываем нажатия для каждого типа кнопок отдельно
    for interaction in &classic_query {
        if *interaction == Interaction::Pressed {
            mode_settings.current_mode = GameMode::Classic;
            setup_classic_mode(&mut mode_settings);
            next_state.set(GameState::PreGame);
            return;
        }
    }

    for interaction in &time_attack_query {
        if *interaction == Interaction::Pressed {
            mode_settings.current_mode = GameMode::TimeAttack;
            setup_time_attack_mode(&mut mode_settings);
            next_state.set(GameState::PreGame);
            return;
        }
    }

    for interaction in &zen_query {
        if *interaction == Interaction::Pressed {
            mode_settings.current_mode = GameMode::Zen;
            setup_zen_mode(&mut mode_settings);
            next_state.set(GameState::PreGame);
            return;
        }
    }

    for interaction in &survival_query {
        if *interaction == Interaction::Pressed {
            mode_settings.current_mode = GameMode::Survival;
            setup_survival_mode(&mut mode_settings);
            next_state.set(GameState::PreGame);
            return;
        }
    }

    // Обработка hover-эффектов
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = Color::srgb(0.3, 0.4, 0.5).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.2, 0.3, 0.4).into();
            }
            _ => {}
        }
    }
}

fn setup_classic_mode(mode_settings: &mut GameModeSettings) {
    mode_settings.time_limit = None;
    mode_settings.target_score = None;
    mode_settings.lives = None;
    mode_settings.difficulty_multiplier = 1.0;
}

fn setup_time_attack_mode(mode_settings: &mut GameModeSettings) {
    mode_settings.time_limit = Some(60.0); // 60 секунд
    mode_settings.target_score = None;
    mode_settings.lives = None;
    mode_settings.difficulty_multiplier = 1.2;
}

fn setup_zen_mode(mode_settings: &mut GameModeSettings) {
    mode_settings.time_limit = None;
    mode_settings.target_score = None;
    mode_settings.lives = None; // Бесконечные жизни
    mode_settings.difficulty_multiplier = 0.8; // Легче обычного
}

fn setup_survival_mode(mode_settings: &mut GameModeSettings) {
    mode_settings.time_limit = None;
    mode_settings.target_score = None;
    mode_settings.lives = Some(1); // Одна жизнь
    mode_settings.difficulty_multiplier = 1.5; // Сложнее обычного
}

fn setup_game_mode(
    _commands: Commands,
    mode_settings: Res<GameModeSettings>,
    mut game_timer: ResMut<GameTimer>,
    mut survival_lives: ResMut<SurvivalLives>,
) {
    match mode_settings.current_mode {
        GameMode::TimeAttack => {
            if let Some(time_limit) = mode_settings.time_limit {
                game_timer.remaining_time = time_limit;
                game_timer.is_active = true;
            }
        }
        GameMode::Survival => {
            if let Some(lives) = mode_settings.lives {
                survival_lives.current_lives = lives;
                survival_lives.max_lives = lives;
            }
        }
        _ => {
            // Сброс для других режимов
            game_timer.is_active = false;
            survival_lives.current_lives = 0;
        }
    }
}

fn check_time_attack_victory(
    time: Res<Time>,
    mut game_timer: ResMut<GameTimer>,
    mode_settings: Res<GameModeSettings>,
    _score: Res<GameScore>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if mode_settings.current_mode != GameMode::TimeAttack || !game_timer.is_active {
        return;
    }

    game_timer.remaining_time -= time.delta_secs();

    if game_timer.remaining_time <= 0.0 {
        game_timer.is_active = false;
        // Время вышло - переходим к Game Over для отображения результатов
        next_state.set(GameState::GameOver);
    }
}

fn check_zen_mode_scoring(mode_settings: Res<GameModeSettings>, _score: ResMut<GameScore>) {
    // В дзен режиме можно добавить бонусы за долгое выживание
    if mode_settings.current_mode == GameMode::Zen {
        // Здесь можно добавить логику бонусов
    }
}

fn check_survival_lives(
    _commands: Commands,
    mode_settings: Res<GameModeSettings>,
    mut survival_lives: ResMut<SurvivalLives>,
    mut collision_events: EventReader<CollisionEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if mode_settings.current_mode != GameMode::Survival {
        return;
    }

    // Обрабатываем только события столкновений за этот кадр
    if collision_events.read().count() > 0 && survival_lives.current_lives > 0 {
        survival_lives.current_lives -= 1;

        if survival_lives.current_lives == 0 {
            // Все жизни закончились - Game Over
            next_state.set(GameState::GameOver);
        }
    }
}

fn apply_mode_difficulty(mode_settings: Res<GameModeSettings>, _pipe_spawner: ResMut<PipeSpawner>) {
    // Применение множителя сложности теперь происходит в spawn_pipes_continuously
    // Эта функция может использоваться для других настроек сложности в будущем
    if !mode_settings.difficulty_multiplier.is_finite()
        || mode_settings.difficulty_multiplier <= 0.0
    {
        bevy::utils::tracing::warn!(
            "Invalid difficulty multiplier: {}",
            mode_settings.difficulty_multiplier
        );
    }
}

fn spawn_game_mode_ui(
    mut commands: Commands,
    mode_settings: Res<GameModeSettings>,
    assets: Res<GameAssets>,
) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(20.0),
                position_type: PositionType::Absolute,
                top: Val::Px(60.0),
                left: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnGameModeUI,
        ))
        .with_children(|parent| {
            // Информация о режиме с цветовой индикацией
            let (mode_text, color) = match mode_settings.current_mode {
                GameMode::Classic => ("Классический режим", Color::WHITE),
                GameMode::TimeAttack => ("⏱ Гонка на время", Color::srgb(1.0, 1.0, 0.0)),
                GameMode::Zen => ("🧘 Дзен режим", Color::srgb(0.0, 1.0, 0.0)),
                GameMode::Survival => ("💀 Режим выживания", Color::srgb(1.0, 0.0, 0.0)),
            };

            parent.spawn((
                Text::new(mode_text),
                TextFont {
                    font: assets.font.clone(),
                    font_size: 26.0,
                    ..default()
                },
                TextColor(color),
                GameModeInfoText,
            ));

            // Дополнительная информация для режимов
            match mode_settings.current_mode {
                GameMode::TimeAttack => {
                    parent.spawn((
                        Text::new("Время: 60.0"),
                        TextFont {
                            font: assets.font.clone(),
                            font_size: 22.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 1.0, 0.0)),
                        GameModeTimerText,
                        Node {
                            margin: UiRect::top(Val::Px(8.0)),
                            ..default()
                        },
                    ));
                }
                GameMode::Survival => {
                    parent.spawn((
                        Text::new("❤️❤️❤️"),
                        TextFont {
                            font: assets.font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.2, 0.2)),
                        GameModeTimerText,
                        Node {
                            margin: UiRect::top(Val::Px(8.0)),
                            ..default()
                        },
                    ));
                }
                GameMode::Zen => {
                    parent.spawn((
                        Text::new("Бесконечный полёт без столкновений"),
                        TextFont {
                            font: assets.font.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 1.0, 0.8)),
                        GameModeTimerText,
                        Node {
                            margin: UiRect::top(Val::Px(5.0)),
                            ..default()
                        },
                    ));
                }
                _ => {} // Классический режим - без дополнительной информации
            }
        });
}

fn update_game_mode_ui(
    mode_settings: Res<GameModeSettings>,
    game_timer: Res<GameTimer>,
    survival_lives: Res<SurvivalLives>,
    mut timer_query: Query<&mut Text, With<GameModeTimerText>>,
) {
    if let Ok(mut text) = timer_query.get_single_mut() {
        match mode_settings.current_mode {
            GameMode::TimeAttack => {
                let time_remaining = game_timer.remaining_time.max(0.0);
                let urgency_color = if time_remaining <= 10.0 {
                    "🔴"
                } else if time_remaining <= 30.0 {
                    "🟡"
                } else {
                    "🟢"
                };
                **text = format!("{} Время: {:.1}с", urgency_color, time_remaining);
            }
            GameMode::Survival => {
                let hearts = match survival_lives.current_lives {
                    3 => "❤️❤️❤️".to_string(),
                    2 => "❤️❤️🖤".to_string(),
                    1 => "❤️🖤🖤".to_string(),
                    0 => "🖤🖤🖤".to_string(),
                    _ => "❤️".repeat(survival_lives.current_lives as usize),
                };
                **text = format!("Жизни: {}", hearts);
            }
            GameMode::Zen => {
                **text = "🌸 Бесконечный полёт 🌸".to_string();
            }
            _ => {
                **text = String::new();
            }
        }
    }
}

fn despawn_game_mode_ui(mut commands: Commands, ui_query: Query<Entity, With<OnGameModeUI>>) {
    for entity in &ui_query {
        commands.entity(entity).despawn_recursive();
    }
}

fn reset_game_mode_state(
    mut game_timer: ResMut<GameTimer>,
    mut survival_lives: ResMut<SurvivalLives>,
) {
    game_timer.remaining_time = 0.0;
    game_timer.is_active = false;
    survival_lives.current_lives = 0;
}
