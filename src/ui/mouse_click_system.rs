use crate::components::{Selectable, WantMove};
use crate::map::Map;
use crate::resources::{MousePosition, SelectedEntity};
use bevy::prelude::*;

pub fn mouse_click_system(
    mut commands: Commands,
    boutton: Res<Input<MouseButton>>,
    position: Res<MousePosition>,
    mut selected: ResMut<SelectedEntity>,
    map: Res<Map>,
    selectables: Query<&Selectable>,
) {
    let click_point = position.grid;
    //Select entity with left click
    //if an entity is already selected, then we will have to choose an action to do
    if boutton.just_pressed(MouseButton::Left) {
        match selected.entity {
            None => {
                let entities = map.get(click_point);
                //select one of the Selectable entity on position randomly
                //TODO refactor, this system will change a lot anyway
                for entity in entities {
                    if let Ok(_) = selectables.get_component::<Selectable>(entity) {
                        selected.entity = Some(entity);
                    };
                }
            }
            Some(entity) => {
                // for now, let say we will alway move to the location of the click
                commands.entity(entity).insert(WantMove {
                    target: click_point,
                });
            }
        }
    }

    //unselect entity with right click
    if boutton.just_pressed(MouseButton::Right) {
        selected.entity = None;
    }
}
