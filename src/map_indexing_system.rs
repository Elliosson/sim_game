use crate::components::GridPoint;
use crate::map::Map;
use bevy::prelude::*;

pub fn map_indexing_system(
    mut commands: Commands,
    mut map: ResMut<Map>,
    transforms: Query<(Entity, &GridPoint)>,
) {
    //populate the map here

    map.clear();

    for (entity, point) in transforms.iter() {
        map.add(point.clone(), entity);
    }
}
