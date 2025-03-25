pub mod components;
pub mod resources;
pub mod systems;

use self::{resources::*, systems::*};
use crate::{
    plugins::{asset_loader::AssetLoaderPlugin, bird::BirdPlugin, pipes::PipesPlugin},
    states::game_state::GameState,
};
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameScore>()
            .add_plugins((AssetLoaderPlugin, BirdPlugin, PipesPlugin))
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                transition_to_game_state
                    .run_if(in_state(GameState::Menu).or(in_state(GameState::GameOver))),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
