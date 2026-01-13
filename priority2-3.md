# План обновления Flappy Bird - Приоритет 2 (остаток) и Приоритет 3

## Обзор текущего состояния

**Приоритет 2 - Реализовано:**
- ✅ Анимированный фон (параллакс)
- ✅ Базовые звуковые эффекты  
- ✅ Уровни сложности
- ✅ Система настроек

**Приоритет 2 - Осталось:**
- ⏳ Визуальные эффекты

---

## Приоритет 2 (остаток) - Визуальные эффекты (1 неделя)

### 1. Система частиц

**Цель:** Создать визуальные эффекты для улучшения игрового опыта

**Технические шаги реализации:**
1. Создать компонент частицы с физикой
2. Реализовать систему спавна частиц для событий
3. Добавить системы обновления и очистки частиц
4. Интегрировать с существующими игровыми событиями

**Необходимые файлы:**
- `src/plugins/effects.rs` (новый плагин)
- `src/core/components.rs` (добавить компоненты частиц)

**Новые компоненты/системы:**
```rust
#[derive(Component)]
pub struct Particle {
    pub velocity: Vec2,
    pub lifetime: Timer,
    pub size: f32,
    pub color: Color,
}

#[derive(Component)]
pub struct ParticleEmitter {
    pub spawn_rate: f32,
    pub particle_count: u32,
}

// Системы
fn spawn_collision_particles(mut commands: Commands, collision_events: EventReader<CollisionEvent>)
fn update_particles(time: Res<Time>, mut query: Query<(&mut Particle, &mut Transform)>)
fn cleanup_particles(mut commands: Commands, query: Query<Entity, With<Particle>>)
```

### 2. Эффект тряски камеры

**Технические шаги реализации:**
1. Создать ресурс управления тряской камеры
2. Реализовать систему применения тряски к камере
3. Добавить триггеры для различных игровых событий
4. Настроить интенсивность и продолжительность

**Новые компоненты/системы:**
```rust
#[derive(Resource)]
pub struct ScreenShake {
    pub duration: Timer,
    pub intensity: f32,
    pub active: bool,
}

#[derive(Component)]
pub struct MainCamera;

// Системы
fn trigger_screen_shake(mut shake: ResMut<ScreenShake>, event: CollisionEvent)
fn apply_screen_shake(time: Res<Time>, mut shake: ResMut<ScreenShake>, mut camera: Query<&mut Transform, With<MainCamera>>)
```

### 3. Всплывающие текстовые эффекты

**Технические шаги реализации:**
1. Создать компонент анимированного текста
2. Реализовать систему спавна плавающих текстов
3. Добавить анимацию движения и исчезновения
4. Интегрировать с системой подсчёта очков

**Новые компоненты/системы:**
```rust
#[derive(Component)]
pub struct FloatingText {
    pub velocity: Vec2,
    pub lifetime: Timer,
    pub start_scale: f32,
}

// Системы
fn spawn_score_floating_text(mut commands: Commands, score_events: EventReader<ScoreEvent>, assets: Res<GameAssets>)
fn update_floating_text(time: Res<Time>, mut query: Query<(&mut FloatingText, &mut Transform, &mut Text)>)
```

---

## Приоритет 3 - Расширение контента (3-4 недели)

### 1. Power-ups и бонусы (1 неделя)

**Цель:** Добавить игровые улучшения для разнообразия геймплея

**Технические шаги реализации:**
1. Создать enum типов power-ups
2. Реализовать компоненты для различных эффектов
3. Добавить систему спавна power-ups
4. Создать визуальные индикаторы активных power-ups

**Необходимые файлы:**
- `src/plugins/powerups.rs` (новый плагин)
- `src/core/components.rs` (добавить компоненты power-ups)

**Новые компоненты/системы:**
```rust
#[derive(Component)]
pub struct PowerUp {
    pub power_type: PowerUpType,
    pub effect_duration: f32,
}

#[derive(Component)]
pub struct ActiveShield {
    pub timer: Timer,
}

#[derive(Component)]
pub struct DoubleScore {
    pub timer: Timer,
}

#[derive(Component)]
pub struct SlowMotion {
    pub timer: Timer,
}

// Системы
fn spawn_power_ups(mut commands: Commands, assets: Res<GameAssets>, time: Res<Time>)
fn collect_power_ups(mut commands: Commands, bird_query: Query<&Bird>, power_up_query: Query<&PowerUp>)
fn update_power_up_effects(time: Res<Time>, mut effects: Query<(&mut Timer, &mut PowerUp)>)
```

### 2. Игровые режимы (1 неделя)

**Цель:** Расширить геймплей различными режимами игры

**Технические шаги реализации:**
1. Создать enum игровых режимов
2. Реализовать ресурс настроек режимов
3. Добавить состояния для выбора режима
4. Создать UI для выбора режима

**Необходимые файлы:**
- `src/plugins/game_modes.rs` (новый плагин)
- `src/states/game_state.rs` (добавить состояние GameModeSelection)

**Новые компоненты/системы:**
```rust
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Classic,
    TimeAttack,
    Zen,
    Survival,
}

#[derive(Resource)]
pub struct GameModeSettings {
    pub current_mode: GameMode,
    pub time_limit: Option<f32>,
    pub target_score: Option<u32>,
}

#[derive(Component)]
pub struct GameModeButton;

// Системы
fn setup_game_mode(mut commands: Commands, mode: Res<GameMode>)
fn check_mode_victory_conditions(time: Res<Time>, score: Res<GameScore>, mode: Res<GameMode>)
fn spawn_game_mode_selection(mut commands: Commands, assets: Res<GameAssets>)
```

### 3. Система достижений (1 неделя)

**Цель:** Мотивировать игроков через систему достижений

**Технические шаги реализации:**
1. Создать структуру достижений
2. Реализовать систему отслеживания прогресса
3. Добавить UI уведомлений о достижениях
4. Создать сохранение прогресса

**Необходимые файлы:**
- `src/plugins/achievements.rs` (новый плагин)
- `src/core/resources.rs` (добавить ресурс достижений)

**Новые компоненты/системы:**
```rust
#[derive(Resource)]
pub struct Achievements {
    pub list: Vec<Achievement>,
    pub unlocked: HashSet<String>,
}

#[derive(Component)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub requirement: AchievementRequirement,
}

#[derive(Component)]
pub struct AchievementNotification;

// Системы
fn check_achievements(score: Res<GameScore>, statistics: Res<GameStatistics>, achievements: ResMut<Achievements>)
fn show_achievement_notification(commands: &mut Commands, achievement: &Achievement, assets: Res<GameAssets>)
```

### 4. Расширенная статистика (1 неделя)

**Цель:** Предоставить детальную статистику игры

**Технические шаги реализации:**
1. Создать ресурс комплексной статистики
2. Реализовать отслеживание всех игровых действий
3. Добавить UI отображения статистики
4. Создать систему анализа и сохранения

**Необходимые файлы:**
- `src/plugins/statistics.rs` (новый плагин)
- `src/core/resources.rs` (добавить ресурс статистики)

**Новые компоненты/системы:**
```rust
#[derive(Resource)]
pub struct GameStatistics {
    pub total_games: u32,
    pub total_score: u64,
    pub best_score: u32,
    pub average_score: f32,
    pub total_play_time: f32,
    pub jumps_made: u32,
    pub pipes_passed: u32,
    pub powerups_collected: u32,
}

#[derive(Component)]
pub struct StatisticsDisplay;

// Системы
fn update_statistics(statistics: ResMut<GameStatistics>, score: Res<GameScore>, time: Res<Time>)
fn display_statistics(commands: &mut Commands, statistics: Res<GameStatistics>, assets: Res<GameAssets>)
```

---

## Обновлённая структура файлов

```
src/
├── core/
│   ├── mod.rs
│   ├── components.rs      # Расширен компонентами частиц и эффектов
│   ├── systems.rs        # Расширен системами UI
│   ├── resources.rs      # Расширен ресурсами статистики и достижений
│   └── utils.rs          # Существующий
├── states/
│   ├── mod.rs
│   ├── app_state.rs       # Существующий
│   └── game_state.rs     # Расширен GameModeSelection
├── plugins/
│   ├── mod.rs
│   ├── asset_loader.rs    # Существующий
│   ├── bird.rs           # Существующий
│   ├── pipes.rs          # Существующий
│   ├── audio.rs          # Существующий
│   ├── high_score.rs     # Существующий
│   ├── difficulty.rs     # Существующий
│   ├── background.rs     # Существующий
│   ├── settings_ui.rs    # Существующий
│   ├── effects.rs        # НОВЫЙ - визуальные эффекты
│   ├── powerups.rs       # НОВЫЙ - power-ups
│   ├── game_modes.rs     # НОВЫЙ - игровые режимы
│   ├── achievements.rs   # НОВЫЙ - достижения
│   └── statistics.rs     # НОВЫЙ - статистика
└── main.rs               # Обновлён регистрация плагинов
```

## Порядок регистрации плагинов в main.rs

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Базовые системы
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(GamePlugin)
        // Priority 1 (уже реализовано)
        .add_plugins(AudioPlugin)
        .add_plugins(HighScorePlugin)
        // Priority 2
        .add_plugins(DifficultyPlugin)
        .add_plugins(BackgroundPlugin)
        .add_plugins(SettingsUIPlugin)
        .add_plugins(EffectsPlugin) // НОВЫЙ
        // Priority 3
        .add_plugins(PowerUpsPlugin)    // НОВЫЙ
        .add_plugins(GameModesPlugin)   // НОВЫЙ
        .add_plugins(AchievementsPlugin)// НОВЫЙ
        .add_plugins(StatisticsPlugin)  // НОВЫЙ
        .run();
}
```

## Метрики качества

### Технические метрики
- Поддерживать 60 FPS с включёнными эффектами
- < 150MB использование памяти с новыми функциями
- < 200ms время загрузки всех ассетов
- Покрытие кода > 75% для новых систем

### Метрики игрового опыта
- Увеличить среднюю продолжительность сессии на 40%
- Достичь 90% положительных отзывов о новых функциях
- Поддерживать отзывчивость UI (< 100ms)
- Снизить отток игроков через систему достижений

Этот план завершает Priority 2 визуальными эффектами и систематизирует реализацию Priority 3 функций для создания полноценной игровой экосистемы Flappy Bird.