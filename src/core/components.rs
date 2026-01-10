use bevy::prelude::*;

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

#[derive(Component)]
pub struct Scrollable;

#[derive(Component)]
pub struct MenuButton;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct ExitButton;
