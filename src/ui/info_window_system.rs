use crate::components::GName;
use crate::map::Map;
use crate::resources::MousePosition;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub fn info_window_system(
    egui_ctx: ResMut<EguiContext>,

    mouse_position: Res<MousePosition>,
    map: Res<Map>,
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

                ui.text_edit_multiline(&mut text)
            })
        });
}
