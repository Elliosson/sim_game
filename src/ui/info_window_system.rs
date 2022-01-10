use crate::components::GName;
use crate::map::Map;
use crate::resources::{MousePosition, SelectedEntity};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

// Create a Window with a lot of info about the current mouse tile and the world
// Just add some usefull info in the window if you want

pub fn info_window_system(
    egui_ctx: ResMut<EguiContext>,

    mouse_position: Res<MousePosition>,
    map: Res<Map>,
    selected_entity: Res<SelectedEntity>,
    names: Query<&GName>,
) {
    egui::Window::new("Info")
        .hscroll(true)
        .show(egui_ctx.ctx(), |ui| {
            ui.vertical(|ui| {
                let mut text = format!(
                    " mouse fposition: x = {}, y = {}",
                    mouse_position.translation_x, mouse_position.translation_y
                );

                text = format!(
                    "{}\n mouse grid position: x = {}, y = {}",
                    text, mouse_position.grid.x, mouse_position.grid.y
                );

                for entity in map.get(mouse_position.grid) {
                    if let Ok(name) = names.get_component::<GName>(entity) {
                        text = format!(
                            "{}\n entity: {:?}, name: {}",
                            text,
                            entity,
                            name.text.clone()
                        );
                    }
                }

                if let Some(selected) = selected_entity.entity {
                    if let Ok(name) = names.get_component::<GName>(selected) {
                        text = format!(
                            "{}\n selected entity: {:?}, name: {}",
                            text,
                            selected,
                            name.text.clone()
                        );
                    } else {
                        eprintln!("Error: the selected entity don't have a GName")
                    }
                }

                ui.text_edit_multiline(&mut text)
            })
        });
}
