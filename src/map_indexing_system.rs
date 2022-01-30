use crate::components::{Blocking, GridPoint};
use crate::map::Map;
use bevy::prelude::*;

// Put every component with a position in the grid
pub fn map_indexing_system(
    mut _commands: Commands,
    mut map: ResMut<Map>,
    passables: Query<(Entity, &GridPoint), Without<Blocking>>,
    blockings: Query<(Entity, &GridPoint), With<Blocking>>,
) {
    //populate the map here

    map.clear();

    for (entity, point) in passables.iter() {
        map.add(point.clone(), entity, false);
    }

    for (entity, point) in blockings.iter() {
        map.add(point.clone(), entity, true);
    }
}
