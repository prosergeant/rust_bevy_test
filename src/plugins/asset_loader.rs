use crate::core::resources::GameAssets;
use bevy::prelude::*;

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets);
    }
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bird_texture: asset_server.load("textures/bird.png"),
        pipe_texture: asset_server.load("textures/pipe.png"),
        // font: asset_server.load("fonts/fira.ttf"),
    });
}
