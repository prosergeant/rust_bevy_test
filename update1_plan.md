# План обновления Flappy Bird v1.0

## Обзор проекта

### Анализ текущего состояния
Игра Flappy Bird построена с использованием Bevy ECS с модульной архитектурой. Текущая реализация включает:
- Базовые игровые механики (движение птицы, генерация труб, обнаружение столкновений)
- Простую систему подсчёта очков
- Управление состояниями игры (MainMenu, PreGame, Playing, Paused, GameOver)
- Систему загрузки ассетов
- Базовый UI с кнопками меню

### Приоритеты реализации
Основываясь на опыте игрока и технической сложности, обновления приоритезируются как:
1. **Быстрые победы** - Высокое влияние, низкая сложность функций
2. **Заметные улучшения** - Средняя сложность, значительное расширение
3. **Расширение контента** - Сложные функции, требующие существенной разработки

---

## Приоритет 1 - Быстрые победы (1-2 недели)

### 1. Система рекордов

**Технические шаги реализации:**
1. Создать компонент хранения рекордов
2. Реализовать сохранение счёта с использованием системы ресурсов Bevy
3. Обновить UI для отображения рекордов
4. Добавить логику сравнения счёта

**Необходимые файлы:**
- `src/core/resources.rs` (добавить ресурс HighScore)
- `src/core/systems.rs` (добавить системы UI рекордов)
- `src/plugins/high_score.rs` (новый плагин)

**Новые компоненты/системы:**
```rust
#[derive(Resource, Default)]
pub struct HighScore {
    pub current: u32,
    pub best: u32,
}

#[derive(Component)]
pub struct HighScoreText;

// Новые системы
fn update_high_score(score: Res<GameScore>, mut high_score: ResMut<HighScore>)
fn display_high_score(mut commands: Commands, assets: Res<GameAssets>, high_score: Res<HighScore>)
```

### 2. Базовые звуковые эффекты

**Технические шаги реализации:**
1. Добавить аудио зависимости в Cargo.toml
2. Создать систему загрузки звуковых ассетов
3. Реализовать звуки прыжка, счёта и столкновения
4. Добавить системы воспроизведения звуков

**Необходимые файлы:**
- `Cargo.toml` (добавить `bevy_kira_audio` или `bevy_audio`)
- `src/core/resources.rs` (добавить ресурс GameSounds)
- `src/plugins/audio.rs` (новый плагин)
- `assets/sounds/` (директория для аудио файлов)

**Новые компоненты/системы:**
```rust
#[derive(Resource)]
pub struct GameSounds {
    pub jump_sound: Handle<AudioSource>,
    pub score_sound: Handle<AudioSource>,
    pub collision_sound: Handle<AudioSource>,
}

#[derive(Component)]
pub struct SoundEmitter;

// Новые системы
fn play_jump_sound(keys: Res<ButtonInput<KeyCode>>, sounds: Res<GameSounds>)
fn play_score_sound(score: Res<GameScore>, sounds: Res<GameSounds>, mut last_score: Local<u32>)
fn play_collision_sound(mut collision_events: EventReader<CollisionEvent>, sounds: Res<GameSounds>)
```

### 3. Анимация птицы

**Технические шаги реализации:**
1. Создать спрайт-лист для анимации птицы
2. Реализовать компонент таймера анимации
3. Добавить систему состояния анимации
4. Обновить рендеринг птицы для использования анимации

**Необходимые файлы:**
- `src/core/components.rs` (добавить компонент BirdAnimation)
- `src/plugins/bird.rs` (добавить системы анимации)
- `assets/textures/bird_animation.png` (спрайт-лист)

**Новые компоненты/системы:**
```rust
#[derive(Component)]
pub struct BirdAnimation {
    pub timer: Timer,
    pub current_frame: usize,
    pub frames: usize,
}

#[derive(Component)]
pub struct AnimatedBird;

// Новые системы
fn animate_bird(time: Res<Time>, mut query: Query<(&mut BirdAnimation, &mut TextureAtlas)>)
```

### 4. Улучшенный Game Over UI

**Технические шаги реализации:**
1. Создать детальный экран окончания игры
2. Добавить отображение счёта и кнопку перезапуска
3. Реализовать навигацию по меню из game over
4. Добавить визуальную обратную связь для окончания игры

**Необходимые файлы:**
- `src/core/systems.rs` (улучшить системы UI game over)
- `src/core/components.rs` (добавить компоненты GameOverUI)

**Новые компоненты/системы:**
```rust
#[derive(Component)]
pub struct GameOverScreen;

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct MainMenuButton;

// Улучшенные системы
fn spawn_game_over_ui(mut commands: Commands, assets: Res<GameAssets>, score: Res<GameScore>, high_score: Res<HighScore>)
fn handle_game_over_buttons(interaction_query: Query<(&Interaction, &RestartButton, &MainMenuButton)>)
```

---

## Приоритет 2 - Заметные улучшения (2-4 недели)

### 1. Уровни сложности

**Технические шаги реализации:**
1. Создать ресурс настроек сложности
2. Реализовать переменную скорость труб и размер просвета
3. Добавить меню выбора сложности
4. Обновить игровые системы для использования параметров сложности

**Необходимые файлы:**
- `src/core/resources.rs` (добавить DifficultySettings)
- `src/states/game_state.rs` (добавить состояние DifficultySelection)
- `src/plugins/difficulty.rs` (новый плагин)

**Новые компоненты/системы:**
```rust
#[derive(Resource, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
    Expert,
}

#[derive(Resource)]
pub struct DifficultySettings {
    pub current: Difficulty,
    pub pipe_speed: f32,
    pub pipe_gap: f32,
    pub spawn_interval: f32,
}

#[derive(Component)]
pub struct DifficultyButton;

// Новые системы
fn set_difficulty_settings(mut settings: ResMut<DifficultySettings>, difficulty: Res<Difficulty>)
fn spawn_difficulty_menu(mut commands: Commands, assets: Res<GameAssets>)
fn handle_difficulty_selection(interaction_query: Query<(&Interaction, &DifficultyButton)>)
```

### 2. Анимированный фон

**Технические шаги реализации:**
1. Создать слои параллакс фона
2. Реализовать систему прокрутки фона
3. Добавить эффекты дня/ночи или погоды
4. Оптимизировать производительность рендеринга

**Необходимые файлы:**
- `src/core/components.rs` (добавить компоненты фона)
- `src/plugins/background.rs` (новый плагин)
- `assets/textures/backgrounds/` (ассеты фона)

**Новые компоненты/системы:**
```rust
#[derive(Component)]
pub struct BackgroundLayer {
    pub speed: f32,
    pub z_index: f32,
}

#[derive(Component)]
pub struct ScrollingBackground;

// Новые системы
fn spawn_background(mut commands: Commands, assets: Res<GameAssets>)
fn scroll_background(time: Res<Time>, mut query: Query<(&mut Transform, &BackgroundLayer)>)
fn day_night_cycle(time: Res<Time>, mut background_query: Query<&mut BackgroundColor>)
```

### 3. Система настроек

**Технические шаги реализации:**
1. Создать UI меню настроек
2. Реализовать сохранение настроек
3. Добавить аудио/видео настройки
4. Создать валидацию настроек

**Необходимые файлы:**
- `src/core/resources.rs` (добавить GameSettings)
- `src/states/game_state.rs` (добавить состояние Settings)
- `src/plugins/settings.rs` (новый плагин)

**Новые компоненты/системы:**
```rust
#[derive(Resource)]
pub struct GameSettings {
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
    pub fullscreen: bool,
    pub vsync: bool,
}

#[derive(Component)]
pub struct SettingsButton;

#[derive(Component)]
pub struct VolumeSlider;

// Новые системы
fn spawn_settings_menu(mut commands: Commands, assets: Res<GameAssets>, settings: Res<GameSettings>)
fn handle_settings_interactions(interaction_query: Query<(&Interaction, &SettingsButton)>)
fn apply_settings(settings: Res<GameSettings>, mut audio: ResMut<Audio>)
```

### 4. Визуальные эффекты

**Технические шаги реализации:**
1. Реализовать систему частиц для эффектов
2. Добавить эффекты столкновения/взрыва
3. Создать плавные переходы между состояниями
4. Добавить эффекты тряски камеры

**Необходимые файлы:**
- `src/core/components.rs` (добавить компоненты эффектов)
- `src/plugins/effects.rs` (новый плагин)

**Новые компоненты/системы:**
```rust
#[derive(Component)]
pub struct Particle {
    pub lifetime: Timer,
    pub velocity: Vec2,
    pub size: f32,
}

#[derive(Component)]
pub struct ScreenShake {
    pub duration: Timer,
    pub intensity: f32,
}

// Новые системы
fn spawn_collision_effect(commands: &mut Commands, position: Vec3)
fn update_particles(time: Res<Time>, mut query: Query<(&mut Particle, &mut Transform)>)
fn apply_screen_shake(time: Res<Time>, mut shake: ResMut<ScreenShake>, mut camera: Query<&mut Transform>)
```

---

## Приоритет 3 - Расширение контента (4-8 недель)

### 1. Power-ups и бонусы

**Технические шаги реализации:**
1. Создать систему power-ups с различными типами
2. Реализовать логику спавна power-ups
3. Добавить визуальные индикаторы и эффекты
4. Создать системы длительности и перезарядки power-ups

**Необходимые файлы:**
- `src/core/components.rs` (добавить компоненты power-ups)
- `src/plugins/powerups.rs` (новый плагин)
- `assets/textures/powerups/` (ассеты power-ups)

**Новые компоненты/системы:**
```rust
#[derive(Component)]
pub struct PowerUp {
    pub power_type: PowerUpType,
    pub duration: Timer,
}

#[derive(Component)]
pub struct Invincibility {
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

// Новые системы
fn spawn_power_ups(mut commands: Commands, assets: Res<GameAssets>, time: Res<Time>)
fn apply_power_up_effects(mut bird_query: Query<&mut Bird>, power_ups: Query<&PowerUp>)
fn update_power_up_timers(time: Res<Time>, mut power_ups: Query<&mut PowerUp>)
```

### 2. Игровые режимы

**Технические шаги реализации:**
1. Создать различные enum игровые режимов
2. Реализовать правила и подсчёт очков для каждого режима
3. Добавить меню выбора режима
4. Создать UI элементы для каждого режима

**Необходимые файлы:**
- `src/states/game_state.rs` (добавить состояние GameMode)
- `src/plugins/game_modes.rs` (новый плагин)

**Новые компоненты/системы:**
```rust
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Classic,
    TimeAttack,
    Zen,
    Survival,
    Reverse,
}

#[derive(Resource)]
pub struct ModeSettings {
    pub current_mode: GameMode,
    pub time_limit: Option<f32>,
    pub target_score: Option<u32>,
}

#[derive(Component)]
pub struct ModeButton;

// Новые системы
fn setup_game_mode(mut commands: Commands, mode: Res<GameMode>)
fn check_mode_victory_conditions(time: Res<Time>, score: Res<GameScore>, mode: Res<GameMode>)
```

### 3. Система достижений

**Технические шаги реализации:**
1. Создать структуру данных достижений
2. Реализовать логику отслеживания достижений
3. Добавить UI уведомлений о достижениях
4. Создать сохранение прогресса достижений

**Необходимые файлы:**
- `src/core/resources.rs` (добавить Achievements)
- `src/plugins/achievements.rs` (новый плагин)

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
    pub unlocked: bool,
}

#[derive(Component)]
pub struct AchievementNotification;

// Новые системы
fn check_achievements(score: Res<GameScore>, achievements: ResMut<Achievements>)
fn show_achievement_notification(commands: &mut Commands, achievement: &Achievement)
```

### 4. Расширенная статистика

**Технические шаги реализации:**
1. Создать комплексное отслеживание статистики
2. Реализовать сохранение статистики
3. Добавить UI отображения статистики
4. Создать функции анализа статистики

**Необходимые файлы:**
- `src/core/resources.rs` (добавить GameStatistics)
- `src/plugins/statistics.rs` (новый плагин)

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

// Новые системы
fn update_statistics(statistics: ResMut<GameStatistics>, score: Res<GameScore>, time: Res<Time>)
fn display_statistics(commands: &mut Commands, statistics: Res<GameStatistics>, assets: Res<GameAssets>)
```

---

## Технические требования

### Новые зависимости
```toml
[dependencies]
bevy = "0.15"
rand = "0.9"
# Поддержка аудио
bevy_kira_audio = "0.20"
# Сериализация для настроек/статистики
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
# Файловый I/O
dirs = "5.0"
# Утилиты времени
chrono = { version = "0.4", features = ["serde"] }
```

### Изменения структуры файлов
```
src/
├── core/
│   ├── mod.rs
│   ├── components.rs      # Расширен новыми компонентами
│   ├── systems.rs         # Расширен новыми системами
│   └── resources.rs       # Расширен новыми ресурсами
├── states/
│   ├── mod.rs
│   ├── app_state.rs       # Расширен новыми состояниями
│   └── game_state.rs      # Расширен состоянием GameMode
├── plugins/
│   ├── mod.rs
│   ├── asset_loader.rs    # Расширен аудио ассетами
│   ├── bird.rs            # Расширен анимацией
│   ├── pipes.rs           # Расширен сложностью
│   ├── audio.rs           # НОВЫЙ
│   ├── high_score.rs      # НОВЫЙ
│   ├── difficulty.rs      # НОВЫЙ
│   ├── background.rs      # НОВЫЙ
│   ├── settings.rs        # НОВЫЙ
│   ├── effects.rs         # НОВЫЙ
│   ├── powerups.rs        # НОВЫЙ
│   ├── game_modes.rs      # НОВЫЙ
│   ├── achievements.rs    # НОВЫЙ
│   └── statistics.rs      # НОВЫЙ
└── utils/
    ├── mod.rs             # НОВЫЙ
    ├── serialization.rs   # НОВЫЙ
    └── file_io.rs         # НОВЫЙ
```

### Требования к ресурсам
```
assets/
├── textures/
│   ├── bird.png
│   ├── bird_animation.png
│   ├── pipe.png
│   ├── backgrounds/
│   │   ├── day.png
│   │   ├── night.png
│   │   └── sunset.png
│   ├── effects/
│   │   ├── explosion.png
│   │   └── particle.png
│   └── powerups/
│       ├── shield.png
│       ├── double_score.png
│       └── slow_motion.png
├── sounds/
│   ├── jump.wav
│   ├── score.wav
│   ├── collision.wav
│   ├── powerup.wav
│   └── background_music.mp3
└── fonts/
    └── Roboto-Regular.ttf
```

### Модификации архитектуры системы

1. **Расширение системы событий**
```rust
#[derive(Event)]
pub struct CollisionEvent;

#[derive(Event)]
pub struct ScoreEvent(pub u32);

#[derive(Event)]
pub struct PowerUpCollectedEvent(pub PowerUpType);

#[derive(Event)]
pub struct AchievementUnlockedEvent(pub String);
```

2. **Порядок интеграции плагинов**
```rust
// В main.rs
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Сначала основные системы
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(GamePlugin)
        // Функции приоритета 1
        .add_plugins(AudioPlugin)
        .add_plugins(HighScorePlugin)
        .add_plugins(BirdAnimationPlugin)
        // Функции приоритета 2
        .add_plugins(DifficultyPlugin)
        .add_plugins(BackgroundPlugin)
        .add_plugins(SettingsPlugin)
        .add_plugins(EffectsPlugin)
        // Функции приоритета 3
        .add_plugins(PowerUpsPlugin)
        .add_plugins(GameModesPlugin)
        .add_plugins(AchievementsPlugin)
        .add_plugins(StatisticsPlugin)
        .run();
}
```

---

## Стратегия реализации

### Порядок разработки
1. **Неделя 1**: Система рекордов + Базовые звуковые эффекты
2. **Неделя 2**: Анимация птицы + Улучшенный Game Over UI
3. **Неделя 3-4**: Уровни сложности + Система настроек
4. **Неделя 5-6**: Анимированный фон + Визуальные эффекты
5. **Неделя 7-8**: Power-ups + Игровые режимы + Достижения + Статистика

### Стратегия тестирования
1. **Модульное тестирование**: Каждая система тестируется независимо
2. **Интеграционное тестирование**: Тестируются взаимодействия плагинов
3. **Тестирование производительности**: Мониторинг FPS и использования памяти
4. **Пользовательское тестирование**: Валидация игрового опыта

```rust
// Пример структуры тестов
#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn test_high_score_update() {
        let mut app = App::new();
        app.add_systems(Update, update_high_score);
        // Реализация теста
    }

    #[test]
    fn test_difficulty_settings() {
        let mut app = App::new();
        app.add_systems(Update, set_difficulty_settings);
        // Реализация теста
    }
}
```

### Подход интеграции
1. **Флаги функций**: Использовать условную компиляцию Bevy для опциональных функций
2. **Инкрементное развёртывание**: Каждый уровень приоритета может развёртываться независимо
3. **Обратная совместимость**: Поддерживать совместимость сохранённых игр между версиями
4. **Модульный дизайн**: Каждый плагин может быть включён/отключён независимо

### Оценка рисков

**Высокий риск:**
- Интеграция аудио системы с существующей загрузкой ассетов
- Влияние на производительность визуальных эффектов
- Миграция данных сохранённых игр

**Средний риск:**
- Сложность UI макета с множественными экранами
- Управление состоянием системы достижений
- Баланс многорежимной игры

**Низкий риск:**
- Сохранение рекордов
- Тайминг анимации птицы
- Хранение настроек

**Стратегии митигации:**
1. **Поэтапное развёртывание**: Тестировать каждую функцию в изоляции
2. **Мониторинг производительности**: Профилирование после каждого крупного добавления
3. **Система резервного копирования**: Реализовать резервное копирование и восстановление сохранённых игр
4. **Настраиваемые функции**: Позволить отключать ресурсоёмкие функции

---

## Метрики успеха

### Технические метрики
- Поддерживать цель 60 FPS на минимальных спецификациях
- < 100ms время загрузки всех ассетов
- < 50MB использование памяти во время игры
- 0 аварийных сбоев в 1000+ тестовых сессий

### Метрики игрового опыта
- Увеличить среднюю продолжительность сессии на 30%
- Достичь 85% положительных отзывов о новых функциях
- Поддерживать < 5 секунд времени до первого взаимодействия
- Снизить фрустрацию от game over, реализуя постепенную сложность

### Метрики разработки
- Покрытие кода > 80%
- Поддерживать < 100 строк кода в среднем на систему
- Ноль критических изменений в обновлениях минорной версии
- Покрытие документации для всех публичных API

Этот комплексный план предоставляет дорожную карту для трансформации базовой реализации Flappy Bird в функциональную, увлекательную игру, поддерживая качество кода и стандарты производительности.