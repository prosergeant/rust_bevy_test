use crate::core::resources::{GameAssets, GameScore, HighScoreEntry, HighScores};
use crate::states::game_state::GameState;
use bevy::prelude::*;
use std::fs;
use std::path::PathBuf;

/// Компонент-маркер для отображения таблицы рекордов
#[derive(Component)]
pub struct HighScoreDisplay;

/// Плагин для управления системой рекордов
pub struct HighScorePlugin;

impl Plugin for HighScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScores>()
            .add_systems(Startup, load_high_scores)
            .add_systems(OnEnter(GameState::GameOver), update_high_scores_and_save)
            .add_systems(OnExit(GameState::GameOver), save_high_scores);
    }
}

/// Загружает рекорды из файла при запуске
pub fn load_high_scores(mut high_scores: ResMut<HighScores>) {
    if let Some(path) = get_high_scores_path() {
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => match serde_json::from_str::<HighScores>(&content) {
                    Ok(loaded_scores) => {
                        *high_scores = loaded_scores;
                        println!("Загружено {} рекордов", high_scores.scores.len());
                    }
                    Err(e) => {
                        eprintln!("Ошибка десериализации рекордов: {}", e);
                    }
                },
                Err(e) => {
                    eprintln!("Ошибка чтения файла рекордов: {}", e);
                }
            }
        }
    }
}

/// Сохраняет рекорды в файл
pub fn save_high_scores(high_scores: Res<HighScores>) {
    if let Some(path) = get_high_scores_path() {
        // Создаем директорию если её нет
        if let Some(parent) = path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("Ошибка создания директории для рекордов: {}", e);
                return;
            }
        }

        match serde_json::to_string_pretty(&*high_scores) {
            Ok(json) => {
                if let Err(e) = fs::write(&path, json) {
                    eprintln!("Ошибка сохранения рекордов: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Ошибка сериализации рекордов: {}", e);
            }
        }
    }
}

/// Обновляет рекорды и сохраняет их при окончании игры
pub fn update_high_scores_and_save(
    mut high_scores: ResMut<HighScores>,
    game_score: Res<GameScore>,
) {
    if game_score.0 > 0 {
        let new_entry = HighScoreEntry::new(game_score.0, "Normal".to_string());

        // Добавляем новый рекорд
        high_scores.scores.push(new_entry);

        // Сортируем по убыванию очков
        high_scores.scores.sort_by(|a, b| b.score.cmp(&a.score));

        // Ограничиваем количество записей
        let max_entries = high_scores.max_entries;
        if high_scores.scores.len() > max_entries {
            high_scores.scores.truncate(max_entries);
        }

        println!("Обновлены рекорды. Текущий счёт: {}", game_score.0);
    }
}

/// Отображает таблицу рекордов на экране
pub fn display_high_scores(
    mut commands: Commands,
    high_scores: Res<HighScores>,
    assets: Res<GameAssets>,
) {
    if high_scores.scores.is_empty() {
        commands.spawn((
            Text::new("Пока нет рекордов!"),
            TextFont {
                font: assets.font.clone(),
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
            HighScoreDisplay,
            Node {
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            },
        ));
    } else {
        commands
            .spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
                HighScoreDisplay,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("🏆 Рекорды"),
                    TextFont {
                        font: assets.font.clone(),
                        font_size: 32.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 0.84, 0.0)),
                    Node {
                        margin: UiRect::bottom(Val::Px(15.0)),
                        ..default()
                    },
                ));

                for (index, entry) in high_scores.scores.iter().enumerate() {
                    parent.spawn((
                        Text::new(format!(
                            "{}. {} - {} очков ({})",
                            index + 1,
                            entry.date,
                            entry.score,
                            entry.difficulty
                        )),
                        TextFont {
                            font: assets.font.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect::vertical(Val::Px(5.0)),
                            ..default()
                        },
                    ));
                }
            });
    }
}

/// Удаляет отображение рекордов
pub fn despawn_high_score_display(
    mut commands: Commands,
    query: Query<Entity, With<HighScoreDisplay>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

/// Возвращает путь к файлу с рекордами
fn get_high_scores_path() -> Option<PathBuf> {
    match dirs::data_dir() {
        Some(mut path) => {
            path.push("my_project");
            path.push("high_scores.json");
            Some(path)
        }
        None => {
            // Fallback к локальному файлу если системная директория недоступна
            Some(PathBuf::from("high_scores.json"))
        }
    }
}

/// Получает лучший рекорд
pub fn get_best_score(high_scores: &HighScores) -> Option<u32> {
    high_scores.scores.first().map(|entry| entry.score)
}

/// Проверяет является ли текущий счёт рекордом
pub fn is_new_high_score(score: u32, high_scores: &HighScores) -> bool {
    if high_scores.scores.is_empty() {
        return true;
    }

    if let Some(best_score) = get_best_score(high_scores) {
        score > best_score
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_high_score_entry_creation() {
        let entry = HighScoreEntry::new(100, "Normal".to_string());
        assert_eq!(entry.score, 100);
        assert_eq!(entry.difficulty, "Normal");
        assert!(!entry.date.is_empty());
    }

    #[test]
    fn test_high_scores_sorting() {
        let mut high_scores = HighScores::default();
        high_scores
            .scores
            .push(HighScoreEntry::new(50, "Normal".to_string()));
        high_scores
            .scores
            .push(HighScoreEntry::new(100, "Normal".to_string()));
        high_scores
            .scores
            .push(HighScoreEntry::new(75, "Normal".to_string()));

        high_scores.scores.sort_by(|a, b| b.score.cmp(&a.score));

        assert_eq!(high_scores.scores[0].score, 100);
        assert_eq!(high_scores.scores[1].score, 75);
        assert_eq!(high_scores.scores[2].score, 50);
    }
}
