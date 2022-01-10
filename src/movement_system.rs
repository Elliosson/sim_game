use crate::components::{GridPoint, Movable, WantMove};
use crate::map::Map;
use bevy::prelude::*;

pub fn movement_system(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut q_moves: Query<(Entity, &mut WantMove, &Movable, &Transform, &GridPoint)>,
) {
    let mut to_remove = Vec::new();

    for (entity, want_move, movable, transform, point) in q_moves.iter() {
        to_remove.push(entity);

        //TODO handle the movement
        eprintln!("Want to move {:?} to {:?}", entity, want_move.target);
    }

    //not very clean, I would prefer to be able to clean all the WantMove componenet in one command
    // right now entity not movalble but with a want to move keep it forever, it's not ok
    for entity in to_remove {
        commands.entity(entity).remove::<WantMove>();
    }

    //stuff to do about tree go here
}
