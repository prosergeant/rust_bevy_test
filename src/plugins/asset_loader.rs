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
        bird_textures: vec![
            asset_server.load("textures/bird_up.png"),
            asset_server.load("textures/bird_mid.png"),
            asset_server.load("textures/bird_down.png"),
        ],
        pipe_texture: asset_server.load("textures/pipe.png"),
        font: asset_server.load("fonts/Roboto-Regular.ttf"),
        // Загрузка аудио ассетов
        jump_sound: asset_server.load("sounds/jump.wav"),
        score_sound: asset_server.load("sounds/score.wav"),
        hit_sound: asset_server.load("sounds/hit.wav"),
        game_over_sound: asset_server.load("sounds/game_over.wav"),
        // Загрузка фоновых слоев
        background_layers: vec![
            asset_server.load("backgrounds/bg_0.png"),    // горы
            asset_server.load("backgrounds/layer_1.png"), // облака
            asset_server.load("backgrounds/layer_2.png"), // деревья
        ],
    });
}

fn check_assets_loaded(
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
    game_assets: Res<GameAssets>,
) {
    let bird_textures_loaded = game_assets
        .bird_textures
        .iter()
        .all(|texture| asset_server.load_state(texture.id()).is_loaded());
    let pipe_loaded = asset_server
        .load_state(game_assets.pipe_texture.id())
        .is_loaded();
    let font_loaded = asset_server.load_state(game_assets.font.id()).is_loaded();
    let jump_sound_loaded = asset_server
        .load_state(game_assets.jump_sound.id())
        .is_loaded();
    let score_sound_loaded = asset_server
        .load_state(game_assets.score_sound.id())
        .is_loaded();
    let hit_sound_loaded = asset_server
        .load_state(game_assets.hit_sound.id())
        .is_loaded();
    let game_over_sound_loaded = asset_server
        .load_state(game_assets.game_over_sound.id())
        .is_loaded();
    let background_layers_loaded = game_assets
        .background_layers
        .iter()
        .all(|handle| asset_server.load_state(handle.id()).is_loaded());

    if bird_textures_loaded
        && pipe_loaded
        && font_loaded
        && jump_sound_loaded
        && score_sound_loaded
        && hit_sound_loaded
        && game_over_sound_loaded
        && background_layers_loaded
    {
        next_state.set(AppState::Loaded);
    }
}
