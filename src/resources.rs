use crate::components::GridPoint;

#[derive(Default, Clone)]
pub struct MousePosition {
    pub translation_x: f32,
    pub translation_y: f32,
    pub grid: GridPoint,
}
