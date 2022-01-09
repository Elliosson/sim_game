use crate::resources::MousePosition;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub fn info_window_system(egui_ctx: ResMut<EguiContext>, mouse_position: Res<MousePosition>) {
    egui::Window::new("Info")
        .hscroll(true)
        .show(egui_ctx.ctx(), |ui| {
            ui.vertical(|ui| {
                let text = format!(
                    " mouse fposition: x = {}, y = {}",
                    mouse_position.translation_x, mouse_position.translation_y
                );

                let mut text = format!(
                    "{}\n mouse grid position: x = {}, y = {}",
                    text, mouse_position.grid_x, mouse_position.grid_y
                );

                ui.text_edit_multiline(&mut text)
            })
        });
}
