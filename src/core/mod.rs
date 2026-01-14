pub mod components;
pub mod difficulty_types;
pub mod resources;
pub mod systems;
pub mod utils;

use self::components::{
    ExitButton, MainMenuButton, MenuButton, OnStatisticsScreen, RestartButton, SettingsButton,
    StartButton, StatisticsButton,
};
use self::resources::{
    GameAssets, GameMode, GameModeSettings, GameOverUIState, GameScore, HighScores,
};
use self::systems::{
    handle_menu_button_clicks, menu_button_hover_effect, transition_to_game_state,
};
use self::utils::despawn_entities;
use crate::plugins::{
    asset_loader::AssetLoaderPlugin,
    audio::AudioPlugin,
    background::BackgroundPlugin,
    bird::BirdPlugin,
    difficulty::DifficultyPlugin,
    effects::EffectsPlugin,
    game_modes::GameModesPlugin,
    high_score::{spawn_game_over_high_scores, HighScorePlugin},
    pipes::PipesPlugin,
    powerups::PowerUpsPlugin,
    progressive_difficulty::ProgressiveDifficultyPlugin,
    settings_ui::SettingsUIPlugin,
    statistics::StatisticsPlugin,
};
use crate::states::app_state::AppState;
use crate::states::game_state::{EffectsSet, GameOverSet, GameState};
use bevy::prelude::*;
use bevy::text::{TextColor, TextFont};
use bevy::ui::{
    AlignItems, BorderRadius, Display, FlexDirection, JustifyContent, Node, Overflow, Val,
};

pub struct GamePlugin;

#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
pub struct OnGameOverScreen;

#[derive(Component)]
pub struct StateDisplayText;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameScore>()
            .init_resource::<GameOverUIState>()
            .init_state::<GameState>()
            .add_plugins((
                AssetLoaderPlugin,
                AudioPlugin,
                BackgroundPlugin,
                BirdPlugin,
                PipesPlugin,
                DifficultyPlugin,
                ProgressiveDifficultyPlugin,
                SettingsUIPlugin,
                EffectsPlugin,
                HighScorePlugin,
                PowerUpsPlugin,
                GameModesPlugin,
                StatisticsPlugin,
            ))
            .add_systems(
                Startup,
                (setup.in_set(EffectsSet::SpawnCam), spawn_state_ui),
            )
            .add_systems(
                OnEnter(AppState::Loaded),
                (init_game_state, init_default_game_mode),
            )
            .add_systems(OnEnter(GameState::PreGame), reset_score)
            .add_systems(
                Update,
                (
                    transition_to_game_state,
                    pregame_to_playing.run_if(in_state(GameState::PreGame)),
                    handle_menu_button_clicks
                        .run_if(in_state(GameState::MainMenu).or(in_state(GameState::GameOver))),
                    menu_button_hover_effect
                        .run_if(in_state(GameState::MainMenu).or(in_state(GameState::GameOver))),
                )
                    .run_if(in_state(AppState::Loaded)),
            )
            .add_systems(
                Update,
                update_state_display
                    .run_if(state_changed::<AppState>.or(state_changed::<GameState>)),
            )
            .add_systems(
                OnEnter(GameState::MainMenu),
                spawn_main_menu.run_if(in_state(AppState::Loaded)),
            )
            .add_systems(
                OnEnter(AppState::Loaded),
                spawn_main_menu.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(
                OnExit(GameState::MainMenu),
                despawn_entities::<OnMainMenuScreen>,
            )
            .add_systems(
                OnEnter(GameState::GameOver),
                spawn_game_over_screen
                    .in_set(GameOverSet::SpawnUi)
                    .after(GameOverSet::UpdateScores)
                    .run_if(in_state(AppState::Loaded)),
            )
            .add_systems(
                Update,
                (
                    show_game_over_ui.run_if(in_state(GameState::GameOver)),
                    menu_button_hover_effect.run_if(in_state(GameState::Statistics)),
                )
                    .run_if(in_state(AppState::Loaded)),
            )
            .add_systems(
                OnExit(GameState::GameOver),
                (
                    despawn_entities::<OnGameOverScreen>,
                    reset_game_over_ui_timer,
                ),
            )
            .add_systems(
                OnExit(GameState::Statistics),
                despawn_entities::<OnStatisticsScreen>,
            );
    }
}

fn init_game_state(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::MainMenu);
}

fn init_default_game_mode(mut mode_settings: ResMut<GameModeSettings>) {
    mode_settings.current_mode = GameMode::Classic;
    mode_settings.time_limit = None;
    mode_settings.target_score = None;
    mode_settings.lives = None;
    mode_settings.difficulty_multiplier = 1.0;
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn reset_score(mut score: ResMut<GameScore>) {
    score.0 = 0;
}

fn pregame_to_playing(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Playing);
}

fn spawn_state_ui(mut commands: Commands, asset: Res<GameAssets>) {
    commands
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(20.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                Text::new("app state: Loading"), // начальный текст
                TextFont {
                    font: asset.font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                StateDisplayText, // маркер для поиска
            ));
        });
}

/// Сбрасывает состояние Game Over UI при выходе из состояния
fn reset_game_over_ui_timer(mut ui_state: ResMut<GameOverUIState>) {
    ui_state.timer = 0.0;
    ui_state.is_visible = false;
}

/// Показать Game Over UI после небольшой задержки
fn show_game_over_ui(
    time: Res<Time>,
    mut game_over_ui_query: Query<&mut Node, With<OnGameOverScreen>>,
    mut ui_state: ResMut<GameOverUIState>,
) {
    // Если UI уже отображён, не выполняем систему
    if ui_state.is_visible {
        return;
    }

    ui_state.timer += time.delta_secs();
    if ui_state.timer >= 0.5 && ui_state.timer < 0.6 {
        for mut node in &mut game_over_ui_query {
            node.display = Display::Flex;
            ui_state.is_visible = true;
        }
    }
}
fn update_state_display(
    mut query: Query<&mut Text, With<StateDisplayText>>,
    app_state: Res<State<AppState>>,
    game_state: Res<State<GameState>>,
) {
    for mut text in &mut query {
        **text = format!("app: {:?} | game: {:?}", app_state.get(), game_state.get());
    }
}

fn spawn_main_menu(mut commands: Commands, asset: Res<GameAssets>) {
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
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Flappy Bird"),
                TextFont {
                    font: asset.font.clone(),
                    font_size: 80.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::bottom(Val::Px(20.0)),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BorderRadius::all(Val::Px(8.0)),
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    MenuButton,
                    StartButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Начать"),
                        TextFont {
                            font: asset.font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::bottom(Val::Px(20.0)),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BorderRadius::all(Val::Px(8.0)),
                    BackgroundColor(Color::srgb(0.2, 0.3, 0.4)),
                    MenuButton,
                    crate::core::components::GameModeSelectionButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Режимы"),
                        TextFont {
                            font: asset.font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::bottom(Val::Px(20.0)),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BorderRadius::all(Val::Px(8.0)),
                    BackgroundColor(Color::srgb(0.3, 0.2, 0.4)),
                    MenuButton,
                    StatisticsButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Статистика"),
                        TextFont {
                            font: asset.font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::bottom(Val::Px(20.0)),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BorderRadius::all(Val::Px(8.0)),
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    MenuButton,
                    SettingsButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Настройки"),
                        TextFont {
                            font: asset.font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BorderRadius::all(Val::Px(8.0)),
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    MenuButton,
                    ExitButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Выход"),
                        TextFont {
                            font: asset.font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

fn spawn_game_over_screen(
    mut commands: Commands,
    score: Res<GameScore>,
    high_scores: Res<HighScores>,
    asset: Res<GameAssets>,
    mut ui_state: ResMut<GameOverUIState>,
) {
    // Устанавливаем флаг состояния
    ui_state.timer = 0.0;
    ui_state.is_visible = false;

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                display: Display::None,
                ..default()
            },
            OnGameOverScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Game Over"),
                TextFont {
                    font: asset.font.clone(),
                    font_size: 80.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            // Отображаем текущий счёт
            parent.spawn((
                Text::new(format!("Счёт: {}", score.0)),
                TextFont {
                    font: asset.font.clone(),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            spawn_game_over_high_scores(parent, &score, &high_scores, &asset);

            // Кнопка перезапуска
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BorderRadius::all(Val::Px(8.0)),
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    MenuButton,
                    RestartButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Перезапустить"),
                        TextFont {
                            font: asset.font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Кнопка главного меню
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BorderRadius::all(Val::Px(8.0)),
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    MenuButton,
                    MainMenuButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Главное меню"),
                        TextFont {
                            font: asset.font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}
