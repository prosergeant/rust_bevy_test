use crate::core::resources::{GameAssets, GameScore, HighScoreEntry, HighScores};
use crate::states::game_state::GameState;
use bevy::prelude::*;
use bevy::text::{TextColor, TextFont};
use bevy::ui::{AlignItems, FlexDirection, Node, UiRect, Val};
use std::fs;
use std::path::PathBuf;

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

/// Отображает рекорды на экране Game Over
pub fn spawn_game_over_high_scores(
    parent: &mut ChildBuilder,
    score: &GameScore,
    high_scores: &HighScores,
    assets: &GameAssets,
) {
    // Отображаем лучший рекорд
    if let Some(best_score) = high_scores.scores.first() {
        let is_new_record = score.0 >= best_score.score;
        let color = if is_new_record {
            Color::srgb(1.0, 0.84, 0.0)
        } else {
            Color::srgb(0.8, 0.8, 0.8)
        };
        let text = if is_new_record {
            format!("🏆 НОВЫЙ РЕКОРД: {}!", best_score.score)
        } else {
            format!("Лучший рекорд: {}", best_score.score)
        };

        parent.spawn((
            Text::new(text),
            TextFont {
                font: assets.font.clone(),
                font_size: 32.0,
                ..default()
            },
            TextColor(color),
            Node {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
        ));
    }

    // Отображаем таблицу рекордов
    if !high_scores.scores.is_empty() {
        parent
            .spawn((Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("📊 Топ рекордов"),
                    TextFont {
                        font: assets.font.clone(),
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    Node {
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    },
                ));

                // Показываем топ 5 рекордов
                for (index, entry) in high_scores.scores.iter().take(5).enumerate() {
                    let medal = match index {
                        0 => "🥇",
                        1 => "🥈",
                        2 => "🥉",
                        _ => "  ",
                    };

                    parent.spawn((
                        Text::new(format!(
                            "{} {}. {} - {} очков",
                            medal,
                            index + 1,
                            entry.date.split(' ').next().unwrap_or(&entry.date),
                            entry.score
                        )),
                        TextFont {
                            font: assets.font.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.6, 0.6)),
                        Node {
                            margin: UiRect::vertical(Val::Px(5.0)),
                            ..default()
                        },
                    ));
                }
            });
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
