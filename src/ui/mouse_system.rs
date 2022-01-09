use crate::components::MainCamera;
use crate::map::translation_to_point;
use crate::resources::MousePosition;
use bevy::prelude::*;

pub fn mouse_system(
    // need to get window dimensions
    wnds: Res<Windows>,
    mut mouse_position: ResMut<MousePosition>,
    // query to get camera transform
    q_camera: Query<&Transform, With<MainCamera>>,
) {
    // get the primary window
    let wnd = wnds.get_primary().unwrap();

    // check if the cursor is in the primary window
    if let Some(pos) = wnd.cursor_position() {
        // get the size of the window
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = pos - size / 2.0;

        // assuming there is exactly one main camera entity, so this is OK
        let camera_transform = q_camera.single();

        // apply the camera transform
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

        //store the result for futur use
        mouse_position.translation_x = pos_wld.x;
        mouse_position.translation_y = pos_wld.y;
        let point = translation_to_point(Vec3::new(pos_wld.x, pos_wld.y, 1.));
        mouse_position.grid_x = point.x;
        mouse_position.grid_y = point.y;
    }
}
