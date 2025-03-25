use crate::core::GamePlugin;
use bevy::prelude::*;
use states::game_state::GameState;

mod core;
mod plugins;
mod states;

fn main() {
    App::new()
        .add_plugins(
            (DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Flappy Bird".into(),
                    resolution: (800., 600.).into(),
                    ..default()
                }),
                ..default()
            })),
        )
        .add_plugins(GamePlugin)
        .init_state::<GameState>()
        .run();
}
