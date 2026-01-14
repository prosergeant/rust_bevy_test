use crate::core::components::{Collider, PowerUp, PowerUpIndicator, PowerUpType};
use crate::core::resources::{ActivePowerUps, GameAssets, PowerUpSpawner};
use crate::core::utils::despawn_entities;
use crate::states::{app_state::AppState, game_state::GameState};
use bevy::prelude::*;
use rand::Rng;

/// Событие сбора Power-up для статистики
#[derive(Event)]
pub struct PowerUpCollectedEvent {
    pub power_type: PowerUpType,
}

/// Константы для Power-ups
pub const POWERUP_SIZE: f32 = 40.0;
// pub const POWERUP_EFFECT_DURATION: f32 = 5.0; // Не используется, закомментировано
pub const POWERUP_SCROLL_SPEED: f32 = -150.0;
pub const SHIELD_DURATION: f32 = 8.0;
pub const DOUBLE_SCORE_DURATION: f32 = 10.0;
pub const SLOW_MOTION_DURATION: f32 = 6.0;

/// Плагин для управления Power-ups
pub struct PowerUpsPlugin;

impl Plugin for PowerUpsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PowerUpSpawner>()
            .init_resource::<ActivePowerUps>()
            .add_event::<PowerUpCollectedEvent>()
            .add_systems(
                Update,
                (
                    spawn_powerups.run_if(in_state(GameState::Playing)),
                    update_powerups,
                    check_powerup_collection.run_if(in_state(GameState::Playing)),
                    update_power_up_timers.run_if(in_state(GameState::Playing)),
                    update_active_effects_state
                        .run_if(in_state(GameState::Playing).or(in_state(GameState::Paused))),
                    update_power_up_ui
                        .run_if(in_state(GameState::Playing).or(in_state(GameState::Paused))),
                    despawn_offscreen_powerups,
                )
                    .run_if(in_state(AppState::Loaded)),
            )
            .add_systems(OnExit(GameState::Playing), (despawn_entities::<PowerUp>,))
            .add_systems(
                OnEnter(GameState::GameOver),
                (
                    despawn_entities::<PowerUp>,
                    despawn_entities::<PowerUpIndicator>,
                    reset_active_powerups,
                ),
            );
    }
}

/// Спавн Power-ups через равные промежутки времени
fn spawn_powerups(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<GameAssets>,
    mut spawner: ResMut<PowerUpSpawner>,
) {
    spawner.timer.tick(time.delta());

    if spawner.timer.finished() {
        let mut rng = rand::rng();

        // Выбираем случайный тип PowerUp
        let power_type = match rng.random_range(0..3) {
            0 => PowerUpType::Shield,
            1 => PowerUpType::DoubleScore,
            _ => PowerUpType::SlowMotion,
        };

        spawn_powerup(&mut commands, &assets, power_type);

        spawner.timer.reset();
    }
}

/// Создание одного Power-up
fn spawn_powerup(commands: &mut Commands, _assets: &GameAssets, power_type: PowerUpType) {
    let (color, effect_duration) = match power_type {
        PowerUpType::Shield => (Color::srgb(0.0, 0.8, 1.0), SHIELD_DURATION),
        PowerUpType::DoubleScore => (Color::srgb(1.0, 0.8, 0.0), DOUBLE_SCORE_DURATION),
        PowerUpType::SlowMotion => (Color::srgb(0.8, 0.4, 1.0), SLOW_MOTION_DURATION),
    };

    commands.spawn((
        PowerUp {
            power_type,
            effect_duration,
        },
        Sprite {
            color,
            custom_size: Some(Vec2::splat(POWERUP_SIZE)),
            ..default()
        },
        Transform::from_translation(Vec3::new(
            450.0, // Начальная позиция за правым краем экрана
            rand::rng().random_range(-200.0..200.0),
            5.0, // Z-позиция между трубами и UI
        )),
        Collider {
            size: Vec2::splat(POWERUP_SIZE),
        },
    ));
}

/// Обновление движения Power-ups
fn update_powerups(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &PowerUp)>,
    active_effects: Res<ActivePowerUps>,
) {
    let speed_multiplier = if active_effects.slow_motion_active {
        0.3 // Замедление в 3 раза
    } else {
        1.0
    };

    for (mut transform, _) in &mut query {
        transform.translation.x += POWERUP_SCROLL_SPEED * speed_multiplier * time.delta_secs();
    }
}

/// Проверка сбора Power-ups
fn check_powerup_collection(
    mut commands: Commands,
    bird_query: Query<(Entity, &Transform), With<crate::core::components::Bird>>,
    powerup_query: Query<(Entity, &Transform, &PowerUp), With<PowerUp>>,
    assets: Res<GameAssets>,
    active_effects: Res<ActivePowerUps>,
    mut powerup_events: EventWriter<PowerUpCollectedEvent>,
) {
    if let Ok((bird_entity, bird_transform)) = bird_query.get_single() {
        for (powerup_entity, powerup_transform, powerup) in &powerup_query {
            // Простая AABB коллизия
            let distance = (bird_transform.translation - powerup_transform.translation).length();
            if distance < (POWERUP_SIZE + 30.0) / 2.0 {
                // Проверяем, не активен ли уже такой же эффект
                let can_collect = match powerup.power_type {
                    PowerUpType::Shield => !active_effects.shield_active,
                    PowerUpType::DoubleScore => !active_effects.double_score_active,
                    PowerUpType::SlowMotion => !active_effects.slow_motion_active,
                };

                if can_collect {
                    // Собираем Power-up
                    commands.entity(powerup_entity).despawn();

                    // Отправляем событие для статистики
                    powerup_events.send(PowerUpCollectedEvent {
                        power_type: powerup.power_type,
                    });

                    // Активируем эффект на птице
                    activate_powerup_effect(
                        &mut commands,
                        &assets,
                        powerup.power_type,
                        powerup.effect_duration,
                        bird_entity,
                    );
                }
            }
        }
    }
}

/// Активация визуального эффекта Power-up на птице
fn activate_powerup_effect(
    commands: &mut Commands,
    assets: &GameAssets,
    power_type: PowerUpType,
    duration: f32,
    bird_entity: Entity,
) {
    let (color, text) = match power_type {
        PowerUpType::Shield => (Color::srgb(0.0, 0.8, 1.0), "ЩИТ"),
        PowerUpType::DoubleScore => (Color::srgb(1.0, 0.8, 0.0), "x2"),
        PowerUpType::SlowMotion => (Color::srgb(0.8, 0.4, 1.0), "SLOW"),
    };

    // Создаем визуальный индикатор как дочерний элемент птицы
    commands.entity(bird_entity).with_children(|parent| {
        parent.spawn((
            Text2d::new(text.to_string()),
            TextFont {
                font: assets.font.clone(),
                font_size: 20.0,
                ..default()
            },
            TextColor(color),
            Transform::from_translation(Vec3::new(0.0, 50.0, 15.0)),
            PowerUpIndicator,
        ));
    });

    // Добавляем компонент эффекта непосредственно к птице
    match power_type {
        PowerUpType::Shield => {
            commands
                .entity(bird_entity)
                .insert(crate::core::components::ActiveShield {
                    timer: Timer::from_seconds(duration, TimerMode::Once),
                });
        }
        PowerUpType::DoubleScore => {
            commands
                .entity(bird_entity)
                .insert(crate::core::components::DoubleScore {
                    timer: Timer::from_seconds(duration, TimerMode::Once),
                });
        }
        PowerUpType::SlowMotion => {
            commands
                .entity(bird_entity)
                .insert(crate::core::components::SlowMotion {
                    timer: Timer::from_seconds(duration, TimerMode::Once),
                });
        }
    }
}

/// Обновление таймеров активных эффектов Power-ups (только в Playing)
fn update_power_up_timers(
    time: Res<Time>,
    mut commands: Commands,
    mut shield_query: Query<(Entity, &mut crate::core::components::ActiveShield)>,
    mut double_score_query: Query<(Entity, &mut crate::core::components::DoubleScore)>,
    mut slow_motion_query: Query<(Entity, &mut crate::core::components::SlowMotion)>,
    mut active_effects: ResMut<ActivePowerUps>,
) {
    // Обновление щита
    for (entity, mut shield) in &mut shield_query {
        shield.timer.tick(time.delta());
        if shield.timer.finished() {
            commands
                .entity(entity)
                .remove::<crate::core::components::ActiveShield>();
            active_effects.shield_active = false;
        } else {
            active_effects.shield_active = true;
        }
    }

    // Обновление двойных очков
    for (entity, mut double_score) in &mut double_score_query {
        double_score.timer.tick(time.delta());
        if double_score.timer.finished() {
            commands
                .entity(entity)
                .remove::<crate::core::components::DoubleScore>();
            active_effects.double_score_active = false;
        } else {
            active_effects.double_score_active = true;
        }
    }

    // Обновление замедления времени
    for (entity, mut slow_motion) in &mut slow_motion_query {
        slow_motion.timer.tick(time.delta());
        if slow_motion.timer.finished() {
            commands
                .entity(entity)
                .remove::<crate::core::components::SlowMotion>();
            active_effects.slow_motion_active = false;
        } else {
            active_effects.slow_motion_active = true;
        }
    }
}

/// Обновление состояния активных эффектов и удаление индикаторов (в Playing и Paused)
fn update_active_effects_state(
    mut commands: Commands,
    bird_query: Query<(Entity, &Children), With<crate::core::components::Bird>>,
    shield_query: Query<Entity, With<crate::core::components::ActiveShield>>,
    double_score_query: Query<Entity, With<crate::core::components::DoubleScore>>,
    slow_motion_query: Query<Entity, With<crate::core::components::SlowMotion>>,
    indicator_query: Query<Entity, With<PowerUpIndicator>>,
    mut active_effects: ResMut<ActivePowerUps>,
) {
    // Обновляем флаги на основе наличия компонентов
    active_effects.shield_active = !shield_query.is_empty();
    active_effects.double_score_active = !double_score_query.is_empty();
    active_effects.slow_motion_active = !slow_motion_query.is_empty();

    // Удаляем индикаторы если соответствующий эффект неактивен
    if let Ok((_bird_entity, children)) = bird_query.get_single() {
        if !active_effects.shield_active
            && !active_effects.double_score_active
            && !active_effects.slow_motion_active
        {
            // Удаляем все индикаторы с птицы
            for &child_entity in children.iter() {
                if indicator_query.contains(child_entity) {
                    commands.entity(child_entity).despawn();
                }
            }
        }
    }
}

/// Обновление UI индикаторов Power-ups
fn update_power_up_ui(
    mut queries: ParamSet<(
        Query<(&Transform, &Children), With<crate::core::components::Bird>>,
        Query<&mut Transform, With<PowerUpIndicator>>,
    )>,
) {
    if let Ok((_bird_transform, children)) = queries.p0().get_single() {
        // Собираем дочерние сущности в вектор, чтобы избежать заимствования
        let child_entities: Vec<Entity> = children.iter().copied().collect();

        for &child in &child_entities {
            if let Ok(mut transform) = queries.p1().get_mut(child) {
                // Индикатор следует за птицей (он уже является дочерним элементом)
                transform.translation.x = 0.0; // Относительно птицы
                transform.translation.y = 50.0; // Относительно птицы
            }
        }
    }
}

/// Удаление Power-ups за экраном
fn despawn_offscreen_powerups(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<PowerUp>>,
) {
    for (entity, transform) in &query {
        if transform.translation.x < -500.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// Сброс активных Power-ups
fn reset_active_powerups(
    mut commands: Commands,
    mut active_effects: ResMut<ActivePowerUps>,
    _bird_query: Query<Entity, With<crate::core::components::Bird>>,
    shield_query: Query<Entity, With<crate::core::components::ActiveShield>>,
    double_score_query: Query<Entity, With<crate::core::components::DoubleScore>>,
    slow_motion_query: Query<Entity, With<crate::core::components::SlowMotion>>,
    indicator_query: Query<Entity, With<PowerUpIndicator>>,
) {
    // Удаляем компоненты эффектов с птицы
    for entity in &shield_query {
        commands
            .entity(entity)
            .remove::<crate::core::components::ActiveShield>();
    }
    for entity in &double_score_query {
        commands
            .entity(entity)
            .remove::<crate::core::components::DoubleScore>();
    }
    for entity in &slow_motion_query {
        commands
            .entity(entity)
            .remove::<crate::core::components::SlowMotion>();
    }

    // Удаляем индикаторы
    for entity in &indicator_query {
        commands.entity(entity).despawn();
    }

    // Сбрасываем состояние
    active_effects.shield_active = false;
    active_effects.double_score_active = false;
    active_effects.slow_motion_active = false;
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::time::Timer;

    #[test]
    fn test_powerup_types() {
        let shield = PowerUpType::Shield;
        let double_score = PowerUpType::DoubleScore;
        let slow_motion = PowerUpType::SlowMotion;

        assert_ne!(shield, double_score);
        assert_ne!(double_score, slow_motion);
        assert_ne!(shield, slow_motion);
    }

    #[test]
    fn test_powerup_component() {
        let powerup = PowerUp {
            power_type: PowerUpType::Shield,
            effect_duration: 5.0,
        };

        assert_eq!(powerup.effect_duration, 5.0);
        assert_eq!(powerup.power_type, PowerUpType::Shield);
    }

    #[test]
    fn test_active_powerups_default() {
        let active = ActivePowerUps::default();

        assert!(!active.shield_active);
        assert!(!active.double_score_active);
        assert!(!active.slow_motion_active);
    }

    #[test]
    fn test_effect_durations() {
        assert_eq!(SHIELD_DURATION, 8.0);
        assert_eq!(DOUBLE_SCORE_DURATION, 10.0);
        assert_eq!(SLOW_MOTION_DURATION, 6.0);
    }

    #[test]
    fn test_powerup_colors() {
        let (shield_color, _) = match PowerUpType::Shield {
            PowerUpType::Shield => (Color::srgb(0.0, 0.8, 1.0), "ЩИТ"),
            _ => unreachable!(),
        };

        assert_eq!(shield_color, Color::srgb(0.0, 0.8, 1.0));
    }

    #[test]
    fn test_timer_components() {
        let shield = crate::core::components::ActiveShield {
            timer: Timer::from_seconds(8.0, TimerMode::Once),
        };
        assert_eq!(shield.timer.duration().as_secs_f32(), 8.0);

        let double_score = crate::core::components::DoubleScore {
            timer: Timer::from_seconds(10.0, TimerMode::Once),
        };
        assert_eq!(double_score.timer.duration().as_secs_f32(), 10.0);

        let slow_motion = crate::core::components::SlowMotion {
            timer: Timer::from_seconds(6.0, TimerMode::Once),
        };
        assert_eq!(slow_motion.timer.duration().as_secs_f32(), 6.0);
    }
}
