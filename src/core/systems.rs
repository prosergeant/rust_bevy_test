use crate::core::components::{ExitButton, MainMenuButton, MenuButton, RestartButton, StartButton};
use crate::states::game_state::GameState;
use bevy::prelude::*;

pub fn transition_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match current_state.get() {
            GameState::GameOver => next_state.set(GameState::MainMenu),
            _ => {}
        }
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
            _ => {}
        }
    }
}

pub fn handle_menu_button_clicks(
    start_button_query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    exit_button_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
    restart_button_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    main_menu_button_query: Query<&Interaction, (Changed<Interaction>, With<MainMenuButton>)>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    for interaction in &start_button_query {
        if *interaction == Interaction::Pressed {
            match current_state.get() {
                GameState::MainMenu => next_state.set(GameState::PreGame),
                _ => {}
            }
        }
    }

    for interaction in &exit_button_query {
        if *interaction == Interaction::Pressed {
            exit.send(AppExit::Success);
        }
    }

    for interaction in &restart_button_query {
        if *interaction == Interaction::Pressed {
            match current_state.get() {
                GameState::GameOver => next_state.set(GameState::PreGame),
                _ => {}
            }
        }
    }

    for interaction in &main_menu_button_query {
        if *interaction == Interaction::Pressed {
            match current_state.get() {
                GameState::GameOver => next_state.set(GameState::MainMenu),
                _ => {}
            }
        }
    }
}

pub fn menu_button_hover_effect(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), With<MenuButton>>,
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
}
