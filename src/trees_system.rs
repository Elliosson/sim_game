use crate::components::{GridPoint, Tree};
use crate::map::Map;
use bevy::prelude::*;

pub fn trees_system(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut tree_query: Query<(&mut Tree, &Transform, &GridPoint)>,
) {
    //stuff to do about tree go here
}
