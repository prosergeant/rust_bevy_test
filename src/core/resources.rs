use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    pub bird_texture: Handle<Image>,
    pub pipe_texture: Handle<Image>,
    pub font: Handle<Font>,
}

#[derive(Resource, Default)]
pub struct GameScore(pub u32);
