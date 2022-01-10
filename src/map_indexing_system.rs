use crate::components::GridPoint;
use crate::map::Map;
use bevy::prelude::*;

// Put every component with a position in the grid
pub fn map_indexing_system(
    mut commands: Commands,
    mut map: ResMut<Map>,
    points: Query<(Entity, &GridPoint)>,
) {
    //populate the map here

    map.clear();

    for (entity, point) in points.iter() {
        map.add(point.clone(), entity);
    }
}
