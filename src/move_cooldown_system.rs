use crate::components::MoveCooldown;
use bevy::prelude::*;

pub fn move_cooldown_system(
    mut commands: Commands,
    mut cooldowns: Query<(Entity, &mut MoveCooldown)>,
) {
    // todo we should probably use a date instead of a improvised cooldown
    let mut to_remove = Vec::new();
    for (entity, mut cooldown) in cooldowns.iter_mut() {
        cooldown.time_left -= 0.1; //todo do something serious with a timer
        if cooldown.time_left < 0. {
            to_remove.push(entity);
        }
    }

    for entity in to_remove.drain(..) {
        commands.entity(entity).remove::<MoveCooldown>();
    }
}
