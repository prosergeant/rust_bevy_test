use crate::core::{resources::ActivePowerUps, utils::despawn_entities};
use crate::plugins::audio::{CollisionEvent, ScoreEvent};
use crate::states::{
    app_state::AppState,
    game_state::{EffectsSet, GameState},
};
use bevy::prelude::*;
use rand::Rng;

/// Компонент частицы с физикой
#[derive(Component)]
pub struct Particle {
    pub velocity: Vec2,
    pub lifetime: Timer,
}

/// Ресурс для управления тряской камеры
#[derive(Resource)]
pub struct ScreenShake {
    pub duration: Timer,
    pub intensity: f32,
    pub active: bool,
}

impl Default for ScreenShake {
    fn default() -> Self {
        Self {
            duration: Timer::from_seconds(0.5, TimerMode::Once),
            intensity: 10.0,
            active: false,
        }
    }
}

/// Компонент для маркировки основной камеры
#[derive(Component)]
pub struct MainCamera;

/// Компонент для всплывающего текста
#[derive(Component)]
pub struct FloatingText {
    pub velocity: Vec2,
    pub lifetime: Timer,
    pub start_scale: f32,
}

/// Константы для визуальных эффектов
pub const PARTICLE_LIFETIME: f32 = 2.0;
pub const FLOATING_TEXT_LIFETIME: f32 = 2.0;
pub const SCREEN_SHAKE_COLLISION_INTENSITY: f32 = 15.0;

/// Плагин для управления визуальными эффектами
pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScreenShake>()
            .add_systems(
                Startup,
                setup_camera
                    .in_set(EffectsSet::UpdateCam)
                    .after(EffectsSet::SpawnCam),
            )
            .add_systems(
                Update,
                (
                    spawn_collision_particles,
                    spawn_score_particles,
                    spawn_score_floating_text,
                    update_particles,
                    update_floating_text,
                    cleanup_particles,
                    trigger_screen_shake_system,
                    apply_screen_shake,
                )
                    .run_if(in_state(AppState::Loaded)),
            )
            .add_systems(
                OnEnter(GameState::GameOver),
                despawn_entities::<FloatingText>,
            );
    }
}

/// Настройка основной камеры с компонентом MainCamera
fn setup_camera(mut commands: Commands, query: Query<Entity, With<Camera2d>>) {
    if let Ok(camera_entity) = query.get_single() {
        commands.entity(camera_entity).insert(MainCamera);
    }
}

/// Создание частиц при столкновении
fn spawn_collision_particles(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for _event in collision_events.read() {
        spawn_particle_explosion(
            &mut commands,
            Vec3::new(0.0, 0.0, 1.0),   // Центр экрана
            Color::srgb(1.0, 0.5, 0.0), // Оранжевый цвет для столкновения
            200,                        // Количество частиц
            5.0,                        // Базовый размер
            200.0,                      // Базовая скорость
        );
    }
}

/// Создание частиц при получении очков
fn spawn_score_particles(mut commands: Commands, mut score_events: EventReader<ScoreEvent>) {
    for _event in score_events.read() {
        spawn_particle_explosion(
            &mut commands,
            Vec3::new(0.0, 100.0, 1.0), // Центр экрана
            Color::srgb(0.0, 1.0, 0.5), // Зелёный цвет для очков
            250,
            3.0,   // Меньший размер
            150.0, // Меньшая скорость
        );
    }
}

/// Создание всплывающего текста при получении очков
fn spawn_score_floating_text(
    mut commands: Commands,
    mut score_events: EventReader<ScoreEvent>,
    assets: Res<crate::core::resources::GameAssets>,
    active_effects: Res<ActivePowerUps>,
) {
    let score_text = if active_effects.double_score_active {
        "+2".to_string()
    } else {
        "+1".to_string()
    };

    for _event in score_events.read() {
        commands.spawn((
            Text2d::new(&score_text),
            TextFont {
                font: assets.font.clone(),
                font_size: 40.0,
                ..default()
            },
            TextColor(Color::srgb(0.0, 1.0, 0.5)),
            Transform::from_translation(Vec3::new(0.0, 100.0, 10.0)),
            FloatingText {
                velocity: Vec2::new(0.0, 100.0), // Движение вверх
                lifetime: Timer::from_seconds(FLOATING_TEXT_LIFETIME, TimerMode::Once),
                start_scale: 1.0,
            },
        ));
    }
}

/// Вспомогательная функция для создания взрыва частиц
fn spawn_particle_explosion(
    commands: &mut Commands,
    position: Vec3,
    color: Color,
    count: u32,
    base_size: f32,
    base_speed: f32,
) {
    for _ in 0..count {
        let mut rng = rand::rng();

        // Случайное направление и скорость
        let angle: f32 = rng.random_range(0.0..2.0 * std::f32::consts::PI);
        let speed = rng.random_range(base_speed * 0.5..base_speed * 1.5);
        let velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);

        // Случайный размер
        let size = rng.random_range(base_size * 0.5..base_size * 1.5);

        commands.spawn((
            Particle {
                velocity,
                lifetime: Timer::from_seconds(PARTICLE_LIFETIME, TimerMode::Once),
            },
            Sprite {
                color,
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            Transform::from_translation(position),
        ));
    }
}

/// Обновление частиц: движение и время жизни
fn update_particles(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Particle, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut particle, mut transform, mut sprite) in &mut query {
        // Обновляем время жизни
        particle.lifetime.tick(time.delta());

        // Движение
        transform.translation.x += particle.velocity.x * time.delta_secs();
        transform.translation.y += particle.velocity.y * time.delta_secs();

        // Гравитация
        particle.velocity.y -= 500.0 * time.delta_secs();

        // Затухание цвета по времени жизни
        let alpha = particle.lifetime.fraction();
        sprite.color.set_alpha(alpha);

        // Уменьшение размера
        transform.scale = Vec3::splat(alpha);

        // Удаляем частицу когда время жизни истекло
        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// Обновление всплывающего текста
fn update_floating_text(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut FloatingText, &mut Transform, &mut Text2d)>,
) {
    for (entity, mut text, mut transform, _) in &mut query {
        // Обновляем время жизни
        text.lifetime.tick(time.delta());

        // Движение
        transform.translation.x += text.velocity.x * time.delta_secs();
        transform.translation.y += text.velocity.y * time.delta_secs();

        // Затухание движения
        text.velocity.x *= 0.98;
        text.velocity.y *= 0.98;

        // Изменение масштаба по времени жизни
        let scale = text.start_scale * (1.0 + (1.0 - text.lifetime.fraction()) * 0.5);
        transform.scale = Vec3::splat(scale);

        // Удаляем текст когда время жизни истекло
        if text.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// Очистка старых частиц (для оптимизации)
fn cleanup_particles(mut commands: Commands, query: Query<Entity, With<Particle>>) {
    // Ограничиваем количество частиц для производительности
    let particle_count = query.iter().count();
    if particle_count > 500 {
        // Удаляем самые старые частицы
        for entity in query.iter().take(particle_count - 500) {
            commands.entity(entity).despawn();
        }
    }
}

/// Применение тряски к камере
fn apply_screen_shake(
    time: Res<Time>,
    mut shake: ResMut<ScreenShake>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        if shake.active {
            shake.duration.tick(time.delta());

            if shake.duration.finished() {
                shake.active = false;
                // Возвращаем камеру в исходное положение
                camera_transform.translation.x = 0.0;
                camera_transform.translation.y = 0.0;
            } else {
                // Применяем случайное смещение
                let mut rng = rand::rng();
                let shake_x = rng.random_range(-shake.intensity..=shake.intensity);
                let shake_y = rng.random_range(-shake.intensity..=shake.intensity);

                camera_transform.translation.x = shake_x;
                camera_transform.translation.y = shake_y;
            }
        }
    }
}

/// Публичная функция для активации тряски из других систем
impl ScreenShake {
    pub fn trigger(&mut self, intensity: f32, duration: f32) {
        self.intensity = intensity;
        self.duration = Timer::from_seconds(duration, TimerMode::Once);
        self.active = true;
    }
}

/// Система для активации тряски камеры при событиях
fn trigger_screen_shake_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut shake: ResMut<ScreenShake>,
) {
    // Тряска при столкновении
    if collision_events.read().next().is_some() {
        shake.trigger(SCREEN_SHAKE_COLLISION_INTENSITY, 0.5);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_creation() {
        // Базовый тест для проверки создания компонента частицы
        let particle = Particle {
            velocity: Vec2::new(100.0, 200.0),
            lifetime: Timer::from_seconds(1.0, TimerMode::Once),
        };

        assert_eq!(particle.velocity.x, 100.0);
    }

    #[test]
    fn test_screen_shake_default() {
        let shake = ScreenShake::default();

        assert_eq!(shake.intensity, 10.0);
        assert!(!shake.active);
        assert_eq!(
            shake.duration.duration(),
            std::time::Duration::from_secs_f32(0.5)
        );
    }

    #[test]
    fn test_screen_shake_trigger() {
        let mut shake = ScreenShake::default();

        shake.trigger(20.0, 1.0);

        assert_eq!(shake.intensity, 20.0);
        assert!(shake.active);
        assert_eq!(
            shake.duration.duration(),
            std::time::Duration::from_secs_f32(1.0)
        );
    }
}
