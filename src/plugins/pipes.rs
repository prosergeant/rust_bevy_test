use crate::{
    core::{components::{
        Collider,
        Scrollable
    }, resources::*, systems::despawn_entities},
    plugins::bird::Bird,
    states::game_state::GameState,
};
use bevy::prelude::*;

const PIPE_GAP: f32 = 300.0;
const PIPE_WIDTH: f32 = 80.0;

#[derive(Component)]
pub struct Pipe;

pub struct PipesPlugin;

impl Plugin for PipesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::PreGame), spawn_pipes)
            .add_systems(
                Update,
                (move_pipes, check_collisions, score_system).run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::GameOver), despawn_pipes);
    }
}

fn spawn_pipes(mut commands: Commands, assets: Res<GameAssets>, windows: Query<&Window>) {
    let window = windows.single();
    let window_height = window.height();

    for i in 0..3 {
        let pipe_x = 400.0 + i as f32 * 300.0;
        let gap_y = rand::random::<f32>() * 200.0 - 100.0;

        // Верхняя труба
        commands.spawn((
            Sprite {
                image: assets.pipe_texture.clone(),
                custom_size: Some(Vec2::new(PIPE_WIDTH, window_height)),
                ..default()
            },
            Transform {
                translation: Vec3::new(pipe_x, gap_y + PIPE_GAP / 2.0 + window_height / 2.0, 0.0),
                scale: Vec3::new(1.0, -1.0, 1.0), // Переворачиваем
                ..default()
            },
            Pipe,
            Collider {
                size: Vec2::new(PIPE_WIDTH, window_height),
            },
        ));

        // Нижняя труба
        commands.spawn((
            Sprite {
                image: assets.pipe_texture.clone(),
                custom_size: Some(Vec2::new(PIPE_WIDTH, window_height)),
                ..default()
            },
            Transform {
                translation: Vec3::new(pipe_x, gap_y - PIPE_GAP / 2.0 - window_height / 2.0, 0.0),
                ..default()
            },
            Pipe,
            Collider {
                size: Vec2::new(PIPE_WIDTH, window_height),
            },
            Scrollable,
        ));
    }
}

fn move_pipes(mut query: Query<&mut Transform, With<Pipe>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.translation.x -= 100.0 * time.delta_secs();
    }
}

fn check_collisions(
    bird_query: Query<&Transform, With<Bird>>,
    pipe_query: Query<&Transform, With<Pipe>>,
    collider_query: Query<&Collider>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok(bird_transform) = bird_query.get_single() {
        let bird_collider = Collider {
            size: Vec2::new(40.0, 40.0),
        };

        for (pipe_transform, pipe_collider) in pipe_query.iter().zip(collider_query.iter()) {
            if collide(
                bird_transform.translation,
                bird_collider.size,
                pipe_transform.translation,
                pipe_collider.size,
            ) {
                next_state.set(GameState::GameOver);
                return;
            }
        }
    }
}

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
                commands.entity(entity).remove::<Scrollable>();
            }
        }
    }
}

fn despawn_pipes(commands: Commands, query: Query<Entity, With<Pipe>>) {
    despawn_entities::<Pipe>(commands, query);
}

fn collide(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> bool {
    let a_min = a_pos.truncate() - a_size / 2.0;
    let a_max = a_pos.truncate() + a_size / 2.0;
    let b_min = b_pos.truncate() - b_size / 2.0;
    let b_max = b_pos.truncate() + b_size / 2.0;

    a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y
}
