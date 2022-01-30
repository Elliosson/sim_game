use crate::components::{GridPoint, Tree};
use crate::map::Map;
use bevy::prelude::*;

pub fn tree_system(
    mut _commands: Commands,
    mut _map: ResMut<Map>,
    mut _tree_query: Query<(&mut Tree, &Transform, &GridPoint)>,
) {
    //stuff to do about tree go here
}
