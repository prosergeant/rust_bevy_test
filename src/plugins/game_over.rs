use crate::{
    core::{
        components::{MainMenuButton, MenuButton, RestartButton},
        resources::{GameAssets, GameOverUIState, GameScore, HighScores},
        utils::despawn_entities,
    },
    plugins::high_score::spawn_game_over_high_scores,
    states::{
        app_state::AppState,
        game_state::{GameOverSet, GameState},
    },
};

use bevy::prelude::*;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::GameOver),
            spawn_game_over_screen
                .in_set(GameOverSet::SpawnUi)
                .after(GameOverSet::UpdateScores)
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
            Update,
            (show_game_over_ui.run_if(in_state(GameState::GameOver)))
                .run_if(in_state(AppState::Loaded)),
        );
    }
}

#[derive(Component)]
pub struct OnGameOverScreen;

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
