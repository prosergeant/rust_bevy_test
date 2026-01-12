use crate::{
    core::{resources::GameAssets, utils::despawn_entities},
    plugins::audio::{CollisionEvent, GameOverEvent, JumpEvent},
    states::game_state::GameState,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Bird {
    pub velocity: f32,
}

const VELOCITY_TO_ROTATION_RATIO: f32 = 7.5;
const BIRD_SIZE: f32 = 50.0;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::PreGame), spawn_bird)
            .add_systems(
                Update,
                (bird_movement, bird_jump, check_bird_bounds).run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::GameOver), despawn_entities::<Bird>);
    }
}

fn spawn_bird(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        Sprite {
            image: assets.bird_texture.clone(),
            custom_size: Some(Vec2::new(BIRD_SIZE, BIRD_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Bird { velocity: 0.0 },
        Name::new("Bird"),
    ));
}

fn bird_jump(
    mut query: Query<&mut Bird>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut jump_events: EventWriter<super::audio::JumpEvent>,
) {
    if keys.just_pressed(KeyCode::Space) || mouse_buttons.just_pressed(MouseButton::Left) {
        for mut bird in &mut query {
            bird.velocity = 500.0;
        }
        // Отправляем событие прыжка для воспроизведения звука
        jump_events.send(super::audio::JumpEvent);
    }
}

fn bird_movement(time: Res<Time>, mut query: Query<(&mut Bird, &mut Transform)>) {
    for (mut bird, mut transform) in &mut query {
        bird.velocity -= 2000.0 * time.delta_secs();
        transform.translation.y += bird.velocity * time.delta_secs(); // * 100.0;

        transform.rotation = Quat::from_axis_angle(
            Vec3::Z,
            f32::clamp(bird.velocity / VELOCITY_TO_ROTATION_RATIO, -90., 90.).to_radians(),
        );
    }
}

fn check_bird_bounds(
    query: Query<&Transform, With<Bird>>,
    windows: Query<&Window>,
    mut next_state: ResMut<NextState<GameState>>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut game_over_events: EventWriter<GameOverEvent>,
) {
    if let Ok(window) = windows.get_single() {
        let window_height = window.height();
        let bird_height = BIRD_SIZE;

        // Границы с учетом размера птицы
        let top_bound = window_height / 2.0 - bird_height / 2.0;
        let bottom_bound = -window_height / 2.0 + bird_height / 2.0;

        if let Ok(bird_transform) = query.get_single() {
            let bird_y = bird_transform.translation.y;

            if bird_y > top_bound || bird_y < bottom_bound {
                // Отправляем звуковые события
                collision_events.send(CollisionEvent);
                game_over_events.send(GameOverEvent);

                next_state.set(GameState::GameOver);
            }
        }
    }
}
