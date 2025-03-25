use crate::{core::resources::GameAssets, states::game_state::GameState};
use bevy::prelude::*;

#[derive(Component)]
pub struct Bird {
    pub velocity: f32,
}

const VELOCITY_TO_ROTATION_RATIO: f32 = 7.5;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_bird)
            .add_systems(
                Update,
                (bird_movement, bird_jump).run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_bird(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        Sprite {
            image: assets.bird_texture.clone(),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Bird { velocity: 0.0 },
        Name::new("Bird"),
    ));
}

fn bird_jump(mut query: Query<&mut Bird>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        for mut bird in &mut query {
            bird.velocity = 5.0;
        }
    }
}

fn bird_movement(time: Res<Time>, mut query: Query<(&mut Bird, &mut Transform)>) {
    for (mut bird, mut transform) in &mut query {
        bird.velocity -= 15.0 * time.delta_secs();
        transform.translation.y += bird.velocity * time.delta_secs() * 100.0;

        transform.rotation = Quat::from_axis_angle(
            Vec3::Z,
            f32::clamp(bird.velocity / VELOCITY_TO_ROTATION_RATIO, -90., 90.).to_radians(),
        );
    }
}
