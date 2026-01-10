use crate::core::GamePlugin;
use crate::states::app_state::AppState;
use bevy::prelude::*;

mod core;
mod plugins;
mod states;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Flappy Bird".into(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .add_plugins(GamePlugin)
        .run();
}
