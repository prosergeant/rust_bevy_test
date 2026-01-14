//! Модуль параллакс фона для создания глубины и анимации
//!
//! Использует несколько слоев с разной скоростью прокрутки для создания эффекта глубины.
//! Каждый слой имеет два спрайта для бесшовной прокрутки.

use crate::core::resources::GameAssets;
use crate::states::app_state::AppState;
// use crate::states::game_state::GameState;
use bevy::prelude::*;
/// Компонент для слоя параллакс фона
#[derive(Component)]
pub struct BackgroundLayer {
    pub scroll_speed: f32,
}

/// Константы для настройки параллакс фона
pub const BACKGROUND_LAYER_WIDTH: f32 = 1600.0;
pub const BACKGROUND_LAYER_HEIGHT: f32 = 600.0;
pub const LAYER_SPEEDS: [f32; 3] = [20.0, 50.0, 80.0];
pub const LAYER_Z_POSITIONS: [f32; 3] = [-300.0, -200.0, -100.0];

// Плагин для управления фоном
pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loaded), spawn_background_layers)
            .add_systems(
                Update,
                parallax_scroll.run_if(
                    in_state(AppState::Loaded)
                        // .and(in_state(GameState::Playing).or(in_state(GameState::MainMenu))),
                ),
            );
    }
}

/// Создает слои параллакс фона с двумя спрайтами на каждый слой для бесшовной прокрутки
pub fn spawn_background_layers(mut commands: Commands, assets: Res<GameAssets>) {
    // Создаем родительскую сущность для всех слоев фона

    // Создаем по 2 спрайта на каждый слой для бесшовной прокрутки
    for (i, texture) in assets.background_layers.iter().enumerate() {
        // Первый спрайт (основной)
        commands.spawn((
            Sprite {
                image: texture.clone(),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, LAYER_Z_POSITIONS[i])).with_scale(
                Vec3::new(
                    BACKGROUND_LAYER_WIDTH / 800.0 * 2.0,
                    BACKGROUND_LAYER_HEIGHT / 600.0 * 2.0,
                    1.0,
                ),
            ),
            Visibility::default(),
            BackgroundLayer {
                scroll_speed: LAYER_SPEEDS[i],
            },
        ));

        // Второй спрайт (для бесшовности)
        commands.spawn((
            Sprite {
                image: texture.clone(),
                ..default()
            },
            Transform::from_translation(Vec3::new(
                BACKGROUND_LAYER_WIDTH,
                0.0,
                LAYER_Z_POSITIONS[i],
            ))
            .with_scale(Vec3::new(
                BACKGROUND_LAYER_WIDTH / 800.0 * 2.0,
                BACKGROUND_LAYER_HEIGHT / 600.0 * 2.0,
                1.0,
            )),
            Visibility::default(),
            BackgroundLayer {
                scroll_speed: LAYER_SPEEDS[i],
            },
        ));
    }
}

/// Анимирует прокрутку слоев параллакс фона с разной скоростью для каждого слоя
pub fn parallax_scroll(time: Res<Time>, mut query: Query<(&mut Transform, &BackgroundLayer)>) {
    for (mut transform, layer) in &mut query {
        // Двигаем слой влево с его скоростью
        transform.translation.x -= layer.scroll_speed * time.delta_secs();

        // Бесшовная прокрутка - когда спрайт уходит за левый край,
        // переносим его за правый край
        if transform.translation.x <= -BACKGROUND_LAYER_WIDTH {
            transform.translation.x += BACKGROUND_LAYER_WIDTH * 2.0;
        }
    }
}
