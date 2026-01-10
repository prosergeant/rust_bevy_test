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
            GameState::MainMenu => next_state.set(GameState::PreGame),
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

pub fn despawn_entities<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
