use bevy::prelude::*;

pub fn despawn_entities<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
