use crate::{
    core::{
        difficulty_types::{
            DifficultyButton, DifficultyLevel, DifficultySettings, OnSettingsScreen,
        },
        resources::GameAssets,
        utils::despawn_entities,
    },
    states::game_state::GameState,
};
use bevy::prelude::*;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, Node, UiRect, Val};

/// Плагин для UI настроек
pub struct SettingsUIPlugin;

impl Plugin for SettingsUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Settings), spawn_settings_ui)
            .add_systems(
                Update,
                (
                    handle_settings_button_clicks,
                    handle_difficulty_selection,
                    update_difficulty_buttons,
                )
                    .run_if(in_state(GameState::Settings)),
            )
            .add_systems(
                OnExit(GameState::Settings),
                despawn_entities::<OnSettingsScreen>,
            );
    }
}

/// Создает интерфейс настроек
fn spawn_settings_ui(
    mut commands: Commands,
    assets: Res<GameAssets>,
    difficulty: Res<DifficultySettings>,
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
            OnSettingsScreen,
        ))
        .with_children(|parent| {
            // Заголовок
            parent.spawn((
                Text::new("⚙️ Настройки"),
                TextFont {
                    font: assets.font.clone(),
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            // Раздел выбора сложности
            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Сложность:"),
                        TextFont {
                            font: assets.font.clone(),
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        Node {
                            margin: UiRect::bottom(Val::Px(20.0)),
                            ..default()
                        },
                    ));

                    // Кнопки выбора сложности
                    let difficulties = [
                        DifficultyLevel::Easy,
                        DifficultyLevel::Normal,
                        DifficultyLevel::Hard,
                        DifficultyLevel::Progressive,
                    ];

                    for level in difficulties {
                        spawn_difficulty_button(
                            parent,
                            &assets,
                            level,
                            difficulty.current_level == level,
                        );
                    }
                });

            // Кнопка возврата
            spawn_menu_button(parent, &assets, "🔙 Назад в меню", BackButton);
        });
}

/// Создает кнопку выбора сложности
fn spawn_difficulty_button(
    parent: &mut ChildBuilder,
    assets: &GameAssets,
    level: DifficultyLevel,
    is_selected: bool,
) {
    let color = if is_selected {
        Color::srgb(0.2, 0.6, 0.2)
    } else {
        Color::srgb(0.2, 0.2, 0.2)
    };

    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(250.0),
                height: Val::Px(50.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::vertical(Val::Px(5.0)),
                ..default()
            },
            BorderRadius::all(Val::Px(8.0)),
            BackgroundColor(color),
            DifficultyButton { level },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(format!("• {}", level.russian_name())),
                TextFont {
                    font: assets.font.clone(),
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

/// Создает кнопку меню
pub fn spawn_menu_button(
    parent: &mut ChildBuilder,
    assets: &GameAssets,
    text: &str,
    button_component: impl Component,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            },
            BorderRadius::all(Val::Px(8.0)),
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            button_component,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(text),
                TextFont {
                    font: assets.font.clone(),
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

/// Компонент кнопки возврата
#[derive(Component)]
pub struct BackButton;

/// Обрабатывает клики по кнопкам настроек
fn handle_settings_button_clicks(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &BackButton),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, _button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.4, 0.4, 0.4).into();
                next_state.set(GameState::MainMenu);
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.3, 0.3, 0.3).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.2, 0.2, 0.2).into();
            }
        }
    }
}

/// Обрабатывает выбор сложности
fn handle_difficulty_selection(
    mut difficulty_settings: ResMut<DifficultySettings>,
    mut interaction_query: Query<
        (&Interaction, &DifficultyButton),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            difficulty_settings.set_level(button.level);
        }
    }
}

/// Обновляет цвет кнопок сложности в зависимости от текущего выбора
fn update_difficulty_buttons(
    difficulty: Res<DifficultySettings>,
    mut button_query: Query<(&DifficultyButton, &mut BackgroundColor, &Interaction), With<Button>>,
) {
    for (button, mut color, interaction) in &mut button_query {
        let is_selected = difficulty.current_level == button.level;
        let base_color = if is_selected {
            Color::srgb(0.2, 0.6, 0.2)
        } else {
            Color::srgb(0.2, 0.2, 0.2)
        };

        match *interaction {
            Interaction::Pressed => {
                if is_selected {
                    *color = BackgroundColor(Color::srgb(0.0, 0.4, 0.0));
                } else {
                    *color = BackgroundColor(Color::srgb(0.0, 0.0, 0.0));
                }
            }
            Interaction::Hovered => {
                if is_selected {
                    *color = BackgroundColor(Color::srgb(0.1, 0.5, 0.1));
                } else {
                    *color = BackgroundColor(Color::srgb(0.1, 0.1, 0.1));
                }
            }
            Interaction::None => {
                *color = BackgroundColor(base_color);
            }
        }
    }
}
