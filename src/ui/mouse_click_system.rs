use crate::components::Selectable;
use crate::map::Map;
use crate::resources::{MousePosition, SelectedEntity};
use bevy::prelude::*;

pub fn mouse_click_system(
    boutton: Res<Input<MouseButton>>,
    position: Res<MousePosition>,
    mut selected: ResMut<SelectedEntity>,
    map: Res<Map>,
    selectables: Query<&Selectable>,
) {
    if boutton.just_pressed(MouseButton::Left) {
        let point = position.grid;
        let entities = map.get(point);
        //select one of the Selectable entity on position randomly
        //TODO refactor, this system will change a lot anyway
        for entity in entities {
            if let Ok(_) = selectables.get_component::<Selectable>(entity) {
                selected.entity = Some(entity);
            };
        }
    }
}
