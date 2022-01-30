use crate::components::GridPoint;
use crate::SCALE;
use bevy::prelude::*;
use std::collections::HashMap;
use std::convert::TryInto;

use bracket_pathfinding::prelude::*; //probably can only use pathfinding

const MAP_HEIGHT: i32 = 100;
const MAP_WIDTH: i32 = 100;

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles_content: HashMap<GridPoint, Vec<Entity>>,
    pub tiles_blocked: HashMap<GridPoint, bool>,
}

impl Default for Map {
    fn default() -> Self {
        Map {
            width: 100,
            height: 100,
            tiles_content: HashMap::new(),
            tiles_blocked: HashMap::new(),
        }
    }
}

impl Map {
    // creating some interface because I wonder if I change the storage model of the map
    pub fn clear(&mut self) {
        self.tiles_content.clear();
        self.tiles_blocked.clear();
    }

    pub fn add(&mut self, point: GridPoint, entity: Entity, blocking: bool) {
        let tile_content = self.tiles_content.entry(point).or_default();
        tile_content.push(entity);

        if blocking {
            self.tiles_blocked.insert(point, true);
        }
    }

    pub fn get(&self, point: GridPoint) -> Vec<Entity> {
        let tile_content = self.tiles_content.get(&point);
        if let Some(content) = tile_content {
            return content.clone();
        } else {
            return Vec::new();
        };
    }

    pub fn is_blocked(&self, point: GridPoint) -> bool {
        *self.tiles_blocked.get(&point).unwrap_or(&false) // todo why can't I just return the value by copy? to search it.
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        // todo, not really convinced by this
        let destination = loc + delta; // ok, I need to be able to do this map, it look cool

        //I guess I should convert destination to a GridPoint?
        if self.in_bounds(destination) {
            // add in bound

            //todo refactor this shit
            let is_blocked = self.is_blocked(GridPoint::from_point(destination));

            if is_blocked {
                None
            } else {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            }
        } else {
            None
        }
    }

    //create a transformation from a point to a usize index
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

impl BaseMap for Map {
    fn is_opaque(&self, _idx: usize) -> bool {
        false // todo
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }

        if let Some(idx) = self.valid_exit(location, Point::new(-1, -1)) {
            exits.push((idx, 1.4))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 1)) {
            exits.push((idx, 1.4))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, -1)) {
            exits.push((idx, 1.4))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 1)) {
            exits.push((idx, 1.4))
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

impl Algorithm2D for Map {
    // the poin corres pond to the size of the map, so if the map is -10 to +10, point should be 20
    fn dimensions(&self) -> Point {
        // ok, does it assume non negative map position?
        Point::new(MAP_WIDTH, MAP_HEIGHT)
    }

    /* All this shit is very ugly, TODO refactor all this*/
    /// Convert a Point (x/y) to an array index. Defaults to an index based on an array
    /// strided X first.
    fn point2d_to_index(&self, pt: Point) -> usize {
        let bounds = self.dimensions();
        (((pt.y + bounds.y / 2) * bounds.x) + (pt.x + bounds.x / 2))
            .try_into()
            .expect("Not a valid usize")
    }

    /// Convert an array index to a point. Defaults to an index based on an array
    /// strided X first.
    fn index_to_point2d(&self, idx: usize) -> Point {
        let bounds = self.dimensions();
        let w: usize = bounds.x.try_into().expect("Not a valid usize");
        Point::new(
            (idx % w) as i32 - bounds.y / 2,
            (idx / w) as i32 - bounds.x / 2,
        )
    }

    fn in_bounds(&self, _pos: Point) -> bool {
        //todo create real bound
        true
    }
}
