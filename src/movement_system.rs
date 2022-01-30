use crate::components::{GridPoint, Movable, MoveCooldown, WantMove};
use crate::map::{point_to_translation, Map};
use bevy::prelude::*;
use bracket_pathfinding::prelude::*;

pub fn movement_system(
    mut commands: Commands,
    map: Res<Map>,
    mut q_moves: Query<
        (
            Entity,
            &mut WantMove,
            &Movable,
            &mut Transform,
            &mut GridPoint,
        ),
        Without<MoveCooldown>,
    >,
) {
    let mut to_remove = Vec::new();

    //todo we should do something to ensure that we don't have want to move on non movable entity.
    for (entity, want_move, movable, transform, point) in q_moves.iter_mut() {
        eprintln!("Want to move {:?} to {:?}", entity, want_move.target);

        let path = a_star_search(
            map.point2d_to_index(point.to_point()),
            map.point2d_to_index(want_move.target.to_point()),
            &*map, //todo trouver un moyen de faire ca propre
        );
        if path.success {
            if path.steps.len() < 2 {
                //We are on target, so remove want move
                to_remove.push(entity);
            } else {
                let next_cell = GridPoint::from_point(map.index_to_point2d(path.steps[1]));
                eprintln!("next cell = {:?}", next_cell);

                //todo make something to be sure that speed  can't be 0.
                if movable.speed > 0. {
                    commands.entity(entity).insert(MoveCooldown {
                        time_left: 1. / movable.speed, // todo do something to avoir division by 0
                    });

                    //just move it for now.
                    //todo, do something progressive for the movement.
                    make_move(next_cell, point, transform);
                } else {
                    eprintln!("Error: speed can't be inferior or equal to 0");
                    to_remove.push(entity);
                }
            }
        } else {
            eprintln!("pathfinding fail");
            to_remove.push(entity);
        }
    }

    for entity in to_remove.drain(..) {
        commands.entity(entity).remove::<WantMove>();
    }
}

fn make_move(destination: GridPoint, mut point: Mut<GridPoint>, mut transform: Mut<Transform>) {
    *point = destination.clone();
    transform.translation = point_to_translation(destination);
}
