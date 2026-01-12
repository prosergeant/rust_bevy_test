use crate::{
    core::{resources::GameAssets, utils::despawn_entities},
    plugins::audio::{CollisionEvent, GameOverEvent, JumpEvent},
    states::game_state::GameState,
};
use bevy::prelude::*;
use bevy::time::Timer;

#[derive(Component)]
pub struct Bird {
    pub velocity: f32,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum BirdAnimationState {
    #[default]
    Idle,
    Flapping,
    Falling,
}

#[derive(Component)]
pub struct BirdAnimation {
    pub timer: Timer,
    pub current_frame: usize,
    pub state: BirdAnimationState,
}

const VELOCITY_TO_ROTATION_RATIO: f32 = 7.5;
const BIRD_SIZE: f32 = 50.0;

// Скорости анимации для разных состояний
const FLAPPING_ANIMATION_SPEED: f32 = 0.1;
const IDLE_ANIMATION_SPEED: f32 = 0.2;
const FALLING_ANIMATION_SPEED: f32 = 0.3;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::PreGame), spawn_bird)
            .add_systems(
                Update,
                (bird_movement, bird_jump, animate_bird, check_bird_bounds)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::GameOver), despawn_entities::<Bird>);
    }
}

fn spawn_bird(mut commands: Commands, assets: Res<GameAssets>) {
    let initial_texture = assets.bird_textures.first().cloned().unwrap_or_default();

    commands.spawn((
        Sprite {
            image: initial_texture,
            custom_size: Some(Vec2::new(BIRD_SIZE, BIRD_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Bird { velocity: 0.0 },
        BirdAnimation {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            current_frame: 0,
            state: BirdAnimationState::Idle,
        },
        Name::new("Bird"),
    ));
}

fn bird_jump(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut jump_events: EventWriter<JumpEvent>,
) {
    if keys.just_pressed(KeyCode::Space) || mouse_buttons.just_pressed(MouseButton::Left) {
        // Отправляем событие прыжка для воспроизведения звука
        jump_events.send(JumpEvent);
    }
}

fn bird_movement(
    time: Res<Time>,
    mut reader: EventReader<JumpEvent>,
    mut query: Query<(&mut Bird, &mut Transform)>,
) {
    let jumped = reader.read().next().is_some();

    if let Ok((mut bird, mut transform)) = query.get_single_mut() {
        if jumped {
            bird.velocity = 500.0;
        }

        bird.velocity -= 2000.0 * time.delta_secs();
        transform.translation.y += bird.velocity * time.delta_secs();

        transform.rotation = Quat::from_axis_angle(
            Vec3::Z,
            f32::clamp(bird.velocity / VELOCITY_TO_ROTATION_RATIO, -90., 90.).to_radians(),
        );
    }
}

fn animate_bird(
    time: Res<Time>,
    mut query: Query<(&Bird, &mut BirdAnimation, &mut Sprite)>,
    assets: Res<GameAssets>,
) {
    for (bird, mut animation, mut sprite) in &mut query {
        // Обновляем таймер анимации
        animation.timer.tick(time.delta());

        // Определяем состояние анимации на основе скорости
        let new_state = if bird.velocity > 100.0 {
            BirdAnimationState::Flapping
        } else if bird.velocity < -100.0 {
            BirdAnimationState::Falling
        } else {
            BirdAnimationState::Idle
        };

        // Если состояние изменилось, обновляем таймер и сбрасываем кадр
        if new_state != animation.state {
            animation.state = new_state;
            animation.current_frame = 0;

            // Устанавливаем скорость таймера для нового состояния
            match animation.state {
                BirdAnimationState::Flapping => {
                    animation.timer =
                        Timer::from_seconds(FLAPPING_ANIMATION_SPEED, TimerMode::Repeating);
                }
                BirdAnimationState::Idle => {
                    animation.timer =
                        Timer::from_seconds(IDLE_ANIMATION_SPEED, TimerMode::Repeating);
                }
                BirdAnimationState::Falling => {
                    animation.timer =
                        Timer::from_seconds(FALLING_ANIMATION_SPEED, TimerMode::Repeating);
                }
            }
        }

        // Переключаем кадры на основе таймера
        if animation.timer.just_finished() {
            animation.current_frame = (animation.current_frame + 1) % assets.bird_textures.len();

            // Меняем текстуру спрайта на соответствующий кадр
            if let Some(new_texture) = assets.bird_textures.get(animation.current_frame) {
                sprite.image = new_texture.clone();
            }
        }
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
