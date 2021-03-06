mod components;
mod map;
mod map_indexing_system;
mod move_cooldown_system;
mod movement_system;
mod resources;
mod tree_system;
mod ui;

use bevy::{core::FixedTimestep, prelude::*};
use bevy_egui::EguiPlugin;
use components::*;
use map::Map;
use map_indexing_system::map_indexing_system;
use move_cooldown_system::move_cooldown_system;
use movement_system::movement_system;
use resources::*;
use tree_system::tree_system;
use ui::*;

const TIME_STEP: f32 = 1.0 / 60.0;
const SCALE: i32 = 30;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .insert_resource(Map::default())
        .insert_resource(MousePosition::default())
        .insert_resource(SelectedEntity::default())
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9))) // background color
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(map_indexing_system)
                .with_system(tree_system)
                .with_system(info_window_system)
                .with_system(mouse_position_system)
                .with_system(movement_system)
                .with_system(move_cooldown_system)
                .with_system(mouse_click_system),
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // Add the game's entities to our world

    // cameras
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    commands.spawn_bundle(UiCameraBundle::default());

    // trees
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(30.0, 30.0, 0.0),
                translation: Vec3::new(0.0, -60.0, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.0, 1., 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Tree {})
        .insert(Selectable {})
        .insert(Blocking {})
        .insert(GName {
            text: "tree".to_string(),
        })
        .insert(GridPoint { x: 0, y: -2 });

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(30.0, 30.0, 0.0),
                translation: Vec3::new(0.0, 60.0, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.0, 1., 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Tree {})
        .insert(Selectable {})
        .insert(Blocking {})
        .insert(GName {
            text: "tree".to_string(),
        })
        .insert(GridPoint { x: 0, y: 2 });

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(30.0, 30.0, 0.0),
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.0, 1., 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Tree {})
        .insert(Selectable {})
        .insert(Blocking {})
        .insert(GName {
            text: "tree".to_string(),
        })
        .insert(GridPoint { x: 0, y: 0 });

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(30.0, 30.0, 0.0),
                translation: Vec3::new(30.0, 30.0, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 0., 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Colonist {})
        .insert(Movable { speed: 5. })
        .insert(Selectable {})
        .insert(GName {
            text: "colonist".to_string(),
        })
        .insert(GridPoint { x: 1, y: 1 });
}
