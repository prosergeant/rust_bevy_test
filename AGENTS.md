# AGENTS.md

Этот файл содержит руководства по разработке и команды сборки для проекта игры Flappy Bird.

## Команды сборки и разработки

### Основные команды
```bash
# Собрать проект
cargo build

# Собрать с оптимизациями
cargo build --release

# Запустить игру
cargo run

# Запустить с оптимизациями
cargo run --release
```

### Тестирование и контроль качества
```bash
# Запустить все тесты
cargo test

# Запустить конкретный тестовый модуль
cargo test --lib module_name

# Запустить тесты с выводом
cargo test -- --nocapture

# Проверить код без сборки
cargo check

# Запустить линтер clippy
cargo clippy

# Исправить форматирование
cargo fmt
```

### Рабочий процесс разработки
```bash
# Следить за изменениями и пересобирать
cargo watch -x run

# Сгенерировать документацию
cargo doc --open

# Проверить неиспользуемые зависимости
cargo machete
```

## Архитектурные принципы проекта

### Двойная система состояний
Проект использует два уровня состояний Bevy:
- **AppState**: управляет загрузкой ресурсов (`Loaded`, `Loading` и др.)
- **GameState**: управляет игровым процессом (`MainMenu`, `PreGame`, `Playing`, `Paused`, `GameOver`)

Правильное использование:
```rust
// В системах всегда проверяйте оба состояния
.run_if(in_state(AppState::Loaded))
.run_if(in_state(GameState::Playing))
```

### Модульная структура
```
src/
├── main.rs              # Точка входа, настройка окна
├── core/                # Основные компоненты игры
│   ├── mod.rs           # GamePlugin и основные системы
│   ├── components.rs    # Collider, MenuButton, Scrollable
│   ├── systems.rs       # Управление UI, очистка сущностей
│   └── resources.rs     # GameAssets, GameScore
├── states/              # Состояния игры
│   ├── mod.rs
│   ├── app_state.rs     # AppState для загрузки
│   └── game_state.rs    # GameState для игрового процесса
└── plugins/             # Игровые механики
    ├── mod.rs
    ├── asset_loader.rs  # Загрузка ассетов
    ├── bird.rs          # Управление птицей
    └── pipes.rs         # Трубы и столкновения
```

## Соглашения по коду

### Организация импортов
```rust
// Стандартная библиотека
use std::time::Duration;

// Внешние крейты (bevy, rand)
use bevy::prelude::*;
use rand::Rng;

// Локальные модули
use crate::core::GamePlugin;
use crate::states::{app_state::AppState, game_state::GameState};
```

### Компоненты и ECS
Используйте `#[derive(Component)]` для всех игровых сущностей:

```rust
#[derive(Component)]
pub struct Bird {
    pub velocity: f32,
}

#[derive(Component)]
pub struct Pipe;

// Маркер-компоненты для очистки
#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
pub struct Scrollable; // Для подсчёта очков
```

### Ресурсы и глобальное состояние
```rust
#[derive(Resource)]
pub struct GameAssets {
    pub bird_texture: Handle<Image>,
    pub pipe_texture: Handle<Image>,
    pub font: Handle<Font>,
}

#[derive(Resource)]
pub struct GameScore(pub u32);

#[derive(Resource)]
pub struct PipeSpawner {
    pub timer: Timer,
    pub last_pipe_x: f32,
}
```

### Архитектура плагинов
Каждый плагин должен регистрировать системы в правильных lifecycle событиях:

```rust
impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::PreGame), spawn_bird)
            .add_systems(
                Update,
                (bird_movement, bird_jump).run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::GameOver), despawn_bird);
    }
}
```

### Состояния и переходы
```rust
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,  // Главное меню
    PreGame,   // Подготовка к игре
    Playing,   // Активная игра
    Paused,    // Пауза
    GameOver,  // Конец игры
}

// Переход между состояниями
fn start_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::PreGame);
}
```

### Системы и фильтры
Используйте точные запросы и фильтры для производительности:

```rust
// Хорошо: точный запрос с фильтром
fn bird_movement(time: Res<Time>, mut query: Query<(&mut Bird, &mut Transform)>) {
    for (mut bird, mut transform) in &mut query {
        bird.velocity -= 2000.0 * time.delta_secs();
        transform.translation.y += bird.velocity * time.delta_secs();
    }
}

// Для UI с hover-эффектами
fn handle_button_click(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButton),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, _button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.4, 0.4, 0.4).into();
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.3, 0.3, 0.3).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.2, 0.2, 0.2).into();
            }
        }
    }
}
```

### Система очистки сущностей
Используйте маркер-компоненты для групповой очистки:

```rust
// Система очистки
pub fn despawn_entities<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

// В плагине
.add_systems(OnExit(GameState::MainMenu), despawn_entities::<OnMainMenuScreen>)
```

### Управление вводом (гибридное)
Поддержка и мыши, и клавиатуры для одного действия:

```rust
fn bird_jump(
    mut query: Query<&mut Bird>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    if keys.just_pressed(KeyCode::Space) || mouse_buttons.just_pressed(MouseButton::Left) {
        for mut bird in &mut query {
            bird.velocity = 500.0;
        }
    }
}
```

### Константы игрового баланса
```rust
// Физика птицы
const GRAVITY: f32 = 2000.0;
const JUMP_FORCE: f32 = 500.0;
const BIRD_SIZE: f32 = 50.0;

// Параметры труб
const PIPE_GAP: f32 = 300.0;
const PIPE_WIDTH: f32 = 80.0;
const PIPE_SPAWN_INTERVAL: f32 = 2.0;
const PIPE_DISTANCE: f32 = 400.0;

// Игровое окно
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
```

### UI компоненты и взаимодействие
```rust
// Кнопки меню
#[derive(Component)]
pub struct MenuButton; // Базовый компонент

#[derive(Component)]
pub struct StartButton; // Конкретная кнопка

#[derive(Component)]
pub struct ExitButton; // Конкретная кнопка

// Создание кнопки с hover-эффектом
fn spawn_menu_button(commands: &mut Commands, text: &str, button_component: impl Component) {
    commands
        .spawn((
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BorderRadius::all(Val::Px(8.0)),
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            MenuButton,
            button_component,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(text),
                TextFont {
                    font: assets.font.clone(),
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}
```

### Система подсчёта очков
```rust
// Используйте Scrollable компонент для отслеживания пройденных труб
fn score_system(
    mut score: ResMut<GameScore>,
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Scrollable>>,
    bird_query: Query<&Transform, With<Bird>>,
) {
    if let Ok(bird_transform) = bird_query.get_single() {
        for (entity, transform) in &query {
            if transform.translation.x < bird_transform.translation.x - 50.0 {
                score.0 += 1;
                commands.entity(entity).remove::<Scrollable>(); // Убираем, чтобы не засчитывать повторно
            }
        }
    }
}
```

## Соглашения по именованию

- **Компоненты**: PascalCase (`Bird`, `Pipe`, `MenuButton`)
- **Системы**: snake_case (`bird_movement`, `spawn_pipes`, `handle_menu_button_clicks`)
- **Ресурсы**: PascalCase (`GameAssets`, `GameScore`, `PipeSpawner`)
- **Константы**: SCREAMING_SNAKE_CASE (`PIPE_WIDTH`, `BIRD_SIZE`, `GRAVITY`)
- **Состояния**: PascalCase (`GameState`, `AppState`)
- **Плагины**: PascalCase с суффиксом `Plugin` (`BirdPlugin`, `PipesPlugin`)
- **Маркер-компоненты**: PascalCase (`OnMainMenuScreen`, `OnGameOverScreen`)

## Специфичные механики игры

### Управление птицей
```rust
// Физика и вращение птицы
fn bird_movement(time: Res<Time>, mut query: Query<(&mut Bird, &mut Transform)>) {
    for (mut bird, mut transform) in &mut query {
        bird.velocity -= GRAVITY * time.delta_secs();
        transform.translation.y += bird.velocity * time.delta_secs();
        
        // Вращение в зависимости от скорости
        transform.rotation = Quat::from_axis_angle(
            Vec3::Z,
            f32::clamp(bird.velocity / VELOCITY_TO_ROTATION_RATIO, -90., 90.).to_radians(),
        );
    }
}
```

### Обработка столкновений
```rust
// Простая AABB коллизия
fn collide(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> bool {
    let a_min = a_pos.truncate() - a_size / 2.0;
    let a_max = a_pos.truncate() + a_size / 2.0;
    let b_min = b_pos.truncate() - b_size / 2.0;
    let b_max = b_pos.truncate() + b_size / 2.0;
    
    a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y
}
```

## Рекомендации по разработке

### Добавление новых состояний
1. Добавьте состояние в соответствующий enum (`GameState` или `AppState`)
2. Используйте `OnEnter` для настройки нового состояния
3. Используйте `OnExit` для очистки предыдущего состояния
4. Добавьте системы с `.run_if(in_state(НовоеСостояние))`

### Расширение UI
1. Создайте новый компонент-маркер для кнопки
2. Добавьте его в базовый `MenuButton` через tuple
3. Обрабатывайте в `handle_menu_button_clicks`
4. Не забудьте hover-эффекты

### Добавление новых игровых механик
1. Создайте отдельный плагин для новой механики
2. Определите необходимые компоненты и ресурсы
3. Зарегистрируйте системы в правильных lifecycle событиях
4. Используйте маркер-компоненты для очистки

### Производительность
- Используйте точные запросы с фильтрами `With<T>` и `Without<T>`
- Избегайте ненужных компонентов и систем
- Используйте `OnEnter`/`OnExit` вместо постоянных проверок в `Update`
- Реализуйте очистку заэкранных объектов для поддержания производительности

Этот файл обеспечивает согласованность разработки в кодовой базе Rust-игры на Bevy, сохраняя русскоязычные элементы UI и следуя лучшим практикам ECS.