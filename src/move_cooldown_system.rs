use crate::components::{GridPoint, MoveCooldown, Tree};
use crate::map::Map;
use bevy::prelude::*;

pub fn move_cooldown_system(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut cooldowns: Query<(Entity, &mut MoveCooldown)>,
) {
    let mut to_remove = Vec::new();
    for (entity, mut cooldown) in cooldowns.iter_mut() {
        eprintln!("cooldown {}", cooldown.time_left);
        cooldown.time_left -= 0.1;
        if cooldown.time_left < 0. {
            to_remove.push(entity);
        }
    }

    for entity in to_remove.drain(..) {
        commands.entity(entity).remove::<MoveCooldown>();
        eprintln!("remove cooldown")
    }
}
