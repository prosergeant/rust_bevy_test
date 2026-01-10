use crate::core::resources::GameAssets;
use crate::states::app_state::AppState;
use bevy::prelude::*;

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), load_assets)
            .add_systems(
                Update,
                check_assets_loaded.run_if(in_state(AppState::Loading)),
            );
    }
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bird_texture: asset_server.load("textures/bird.png"),
        pipe_texture: asset_server.load("textures/pipe.png"),
        font: asset_server.load("fonts/Roboto-Regular.ttf"),
    });
}

fn check_assets_loaded(
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
    game_assets: Res<GameAssets>,
) {
    let bird_loaded = asset_server
        .load_state(game_assets.bird_texture.id())
        .is_loaded();
    let pipe_loaded = asset_server
        .load_state(game_assets.pipe_texture.id())
        .is_loaded();
    let font_loaded = asset_server.load_state(game_assets.font.id()).is_loaded();

    if bird_loaded && pipe_loaded && font_loaded {
        next_state.set(AppState::Loaded);
    }
}
