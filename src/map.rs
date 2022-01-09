use crate::components::GridPoint;
use crate::SCALE;
use bevy::prelude::*;
use std::collections::HashMap;

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles_content: HashMap<GridPoint, Vec<Entity>>,
}

impl Default for Map {
    fn default() -> Self {
        Map {
            width: 100,
            height: 100,
            tiles_content: HashMap::new(),
        }
    }
}

impl Map {
    // creating some interface because I wonder if I change the storage model of the map
    pub fn clear(&mut self) {
        self.tiles_content.clear();
    }

    pub fn add(&mut self, point: GridPoint, entity: Entity) {
        let tile_content = self.tiles_content.entry(point).or_default();
        tile_content.push(entity)
    }

    pub fn get(&self, point: GridPoint) -> Vec<Entity> {
        let tile_content = self.tiles_content.get(&point);
        if let Some(content) = tile_content {
            return content.clone();
        } else {
            return Vec::new();
        };
    }
}

// the sprite is anchored in the middle, the why add SCALE/2
pub fn translation_to_point(translation: Vec3) -> GridPoint {
    let point = GridPoint {
        x: ((translation.x + (SCALE as f32 / 2.)) / SCALE as f32).floor() as i32,
        y: ((translation.y + (SCALE as f32 / 2.)) / SCALE as f32).floor() as i32,
    };

    return point;
}

//TODO check that this is ok
pub fn point_to_translation(point: GridPoint) -> Vec3 {
    let translation = Vec3::new((point.x * SCALE) as f32, (point.y * SCALE) as f32, 1.);
    return translation;
}
