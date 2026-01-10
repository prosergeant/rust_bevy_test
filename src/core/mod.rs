pub mod components;
pub mod resources;
pub mod systems;

use self::resources::{GameAssets, GameScore};
use self::systems::{despawn_entities, transition_to_game_state};
use crate::plugins::{asset_loader::AssetLoaderPlugin, bird::BirdPlugin, pipes::PipesPlugin};
use crate::states::app_state::AppState; // Added AppState import
use crate::states::game_state::GameState;
use bevy::prelude::*;
use bevy::text::{TextFont, TextColor};
use bevy::ui::{Node, PositionType, Val};

pub struct GamePlugin;

#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
pub struct OnGameOverScreen;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameScore>()
            .init_state::<GameState>() // Initialize GameState here
            .add_plugins((AssetLoaderPlugin, BirdPlugin, PipesPlugin))
            .add_systems(Startup, setup)
            .add_systems(OnEnter(AppState::Loaded), init_game_state) // Initialize GameState when AppState is Loaded
            .add_systems(
                Update,
                (
                    transition_to_game_state,
                    pregame_to_playing.run_if(in_state(GameState::PreGame)),
                )
                    .run_if(in_state(AppState::Loaded)), // Only run game logic when AppState is Loaded
            )
            .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu.run_if(in_state(AppState::Loaded)))
            .add_systems(OnExit(GameState::MainMenu), despawn_entities::<OnMainMenuScreen>.run_if(in_state(AppState::Loaded)))
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over_screen.run_if(in_state(AppState::Loaded)))
            .add_systems(OnExit(GameState::GameOver), despawn_entities::<OnGameOverScreen>.run_if(in_state(AppState::Loaded)));
    }
}

fn init_game_state(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::MainMenu);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn pregame_to_playing(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Playing);
}

fn spawn_main_menu(mut commands: Commands, asset: Res<GameAssets>) {
    commands.spawn((
        // Сам текст
        Text::new("Flappy Bird"),
        // Настройки шрифта
        TextFont {
            font: asset.font.clone(),
            font_size: 80.0,
            ..default()
        },
        // Цвет текста
        TextColor(Color::WHITE),
        // Позиционирование в UI (например, в центре)
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(30.0),
            left: Val::Percent(50.0),
            ..default()
        },
        OnMainMenuScreen,
    ));
    commands.spawn((
        // Сам текст
        Text::new("Press Space to Start"),
        // Настройки шрифта
        TextFont {
            font: asset.font.clone(),
            font_size: 40.0,
            ..default()
        },
        // Цвет текста
        TextColor(Color::WHITE),
        // Позиционирование в UI (например, в центре)
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(60.0),
            left: Val::Percent(50.0),
            ..default()
        },
        OnMainMenuScreen,
    ));
}

fn spawn_game_over_screen(mut commands: Commands, score: Res<GameScore>, asset: Res<GameAssets>) {
    commands.spawn((
        // Сам текст
        Text::new("Game Over"),
        // Настройки шрифта
        TextFont {
            font: asset.font.clone(),
            font_size: 80.0,
            ..default()
        },
        // Цвет текста
        TextColor(Color::WHITE),
        // Позиционирование в UI (например, в центре)
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(30.0),
            left: Val::Percent(50.0),
            ..default()
        },
        OnGameOverScreen,
    ));
    commands.spawn((
        // Сам текст
        Text::new(format!("Score: {}", score.0)),
        // Настройки шрифта
        TextFont {
            font: asset.font.clone(),
            font_size: 40.0,
            ..default()
        },
        // Цвет текста
        TextColor(Color::WHITE),
        // Позиционирование в UI (например, в центре)
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(50.0),
            left: Val::Percent(50.0),
            ..default()
        },
        OnGameOverScreen,
    ));
    commands.spawn((
        // Сам текст
        Text::new("Press Space to Restart"),
        // Настройки шрифта
        TextFont {
            font: asset.font.clone(),
            font_size: 40.0,
            ..default()
        },
        // Цвет текста
        TextColor(Color::WHITE),
        // Позиционирование в UI (например, в центре)
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(70.0),
            left: Val::Percent(50.0),
            ..default()
        },
        OnGameOverScreen,
    ));
}
