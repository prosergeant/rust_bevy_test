pub mod components;
pub mod resources;
pub mod systems;
pub mod utils;

use self::components::{ExitButton, MenuButton, StartButton};
use self::resources::{GameAssets, GameScore, HighScores};
use self::systems::{
    despawn_entities, handle_menu_button_clicks, menu_button_hover_effect, transition_to_game_state,
};
use self::utils::despawn_entities;
use crate::plugins::{
    asset_loader::AssetLoaderPlugin, audio::AudioPlugin, bird::BirdPlugin,
    high_score::HighScorePlugin, pipes::PipesPlugin,
};
use crate::states::app_state::AppState; // Added AppState import
use crate::states::game_state::GameState;
use bevy::prelude::*;
use bevy::text::{TextColor, TextFont};
use bevy::ui::{AlignItems, BorderRadius, FlexDirection, JustifyContent, Node, Overflow, Val};

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
            .init_state::<GameState>()
            .add_plugins((
                AssetLoaderPlugin,
                AudioPlugin,
                BirdPlugin,
                PipesPlugin,
                HighScorePlugin,
            ))
            .add_systems(Startup, (setup, spawn_state_ui))
            .add_systems(OnEnter(AppState::Loaded), init_game_state)
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
                spawn_game_over_screen.run_if(in_state(AppState::Loaded)),
            )
            .add_systems(
                OnExit(GameState::GameOver),
                despawn_entities::<OnGameOverScreen>,
            );
    }
}

fn init_game_state(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::MainMenu);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
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
    asset: Res<GameAssets>,
    high_scores: Res<HighScores>,
) {
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

            // Отображаем лучший рекорд
            if let Some(best_score) = high_scores.scores.first() {
                let is_new_record = score.0 >= best_score.score;
                let color = if is_new_record {
                    Color::srgb(1.0, 0.84, 0.0)
                } else {
                    Color::srgb(0.8, 0.8, 0.8)
                };
                let text = if is_new_record {
                    format!("🏆 НОВЫЙ РЕКОРД: {}!", best_score.score)
                } else {
                    format!("Лучший рекорд: {}", best_score.score)
                };

                parent.spawn((
                    Text::new(text),
                    TextFont {
                        font: asset.font.clone(),
                        font_size: 32.0,
                        ..default()
                    },
                    TextColor(color),
                    Node {
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                ));
            }

            // Отображаем таблицу рекордов
            if !high_scores.scores.is_empty() {
                parent
                    .spawn((Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("📊 Топ рекордов"),
                            TextFont {
                                font: asset.font.clone(),
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.7, 0.7, 0.7)),
                            Node {
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                        ));

                        // Показываем топ 5 рекордов
                        for (index, entry) in high_scores.scores.iter().take(5).enumerate() {
                            let medal = match index {
                                0 => "🥇",
                                1 => "🥈",
                                2 => "🥉",
                                _ => "  ",
                            };

                            parent.spawn((
                                Text::new(format!(
                                    "{} {}. {} - {} очков",
                                    medal,
                                    index + 1,
                                    entry.date.split(' ').next().unwrap_or(&entry.date),
                                    entry.score
                                )),
                                TextFont {
                                    font: asset.font.clone(),
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.6, 0.6, 0.6)),
                            ));
                        }
                    });
            }

            parent.spawn((
                Text::new("Нажмите Пробел для перезапуска"),
                TextFont {
                    font: asset.font.clone(),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}
