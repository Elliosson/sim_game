use crate::components::GridPoint;
use bevy::prelude::*;

#[derive(Default, Clone)]
pub struct MousePosition {
    pub translation_x: f32,
    pub translation_y: f32,
    pub grid: GridPoint,
}

#[derive(Default, Clone)]
pub struct SelectedEntity {
    pub entity: Option<Entity>,
}
