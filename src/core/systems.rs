use crate::core::components::{
    ExitButton, GameModeSelectionButton, MainMenuButton, MenuButton, RestartButton, SettingsButton,
    StartButton, StatisticsButton,
};
use crate::states::game_state::GameState;
use bevy::prelude::*;

pub fn transition_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) && current_state.get() == &GameState::GameOver {
        next_state.set(GameState::MainMenu);
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Playing),
            GameState::GameOver => {
                exit.send(AppExit::Success);
            }
            GameState::MainMenu => {
                exit.send(AppExit::Success);
            }
            GameState::Settings => next_state.set(GameState::MainMenu),
            GameState::GameModeSelection => next_state.set(GameState::MainMenu),
            GameState::Statistics => next_state.set(GameState::MainMenu),
            _ => {}
        }
    }
}

pub fn handle_menu_button_clicks(
    start_button_query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    settings_button_query: Query<&Interaction, (Changed<Interaction>, With<SettingsButton>)>,
    statistics_button_query: Query<&Interaction, (Changed<Interaction>, With<StatisticsButton>)>,
    game_mode_button_query: Query<
        &Interaction,
        (Changed<Interaction>, With<GameModeSelectionButton>),
    >,
    exit_button_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
    restart_button_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    main_menu_button_query: Query<&Interaction, (Changed<Interaction>, With<MainMenuButton>)>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    for interaction in &start_button_query {
        if *interaction == Interaction::Pressed && current_state.get() == &GameState::MainMenu {
            next_state.set(GameState::PreGame);
        }
    }

    for interaction in &settings_button_query {
        if *interaction == Interaction::Pressed && current_state.get() == &GameState::MainMenu {
            next_state.set(GameState::Settings);
        }
    }

    for interaction in &statistics_button_query {
        if *interaction == Interaction::Pressed && current_state.get() == &GameState::MainMenu {
            next_state.set(GameState::Statistics);
        }
    }

    for interaction in &game_mode_button_query {
        if *interaction == Interaction::Pressed && current_state.get() == &GameState::MainMenu {
            next_state.set(GameState::GameModeSelection);
        }
    }

    for interaction in &exit_button_query {
        if *interaction == Interaction::Pressed {
            exit.send(AppExit::Success);
        }
    }

    for interaction in &restart_button_query {
        if *interaction == Interaction::Pressed && current_state.get() == &GameState::GameOver {
            next_state.set(GameState::PreGame);
        }
    }

    for interaction in &main_menu_button_query {
        if *interaction == Interaction::Pressed && current_state.get() == &GameState::GameOver {
            next_state.set(GameState::MainMenu);
        }
    }

    for interaction in &main_menu_button_query {
        if *interaction == Interaction::Pressed && current_state.get() == &GameState::Statistics {
            next_state.set(GameState::MainMenu);
        }
    }
}

pub fn menu_button_hover_effect(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<MenuButton>, Without<GameModeSelectionButton>),
    >,
    mut game_mode_button_query: Query<
        (&Interaction, &mut BackgroundColor),
        With<GameModeSelectionButton>,
    >,
) {
    for (interaction, mut bg_color) in &mut button_query {
        match *interaction {
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
            }
            _ => {}
        }
    }

    for (interaction, mut bg_color) in &mut game_mode_button_query {
        match *interaction {
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgb(0.3, 0.4, 0.5));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgb(0.2, 0.3, 0.4));
            }
            _ => {}
        }
    }
}
