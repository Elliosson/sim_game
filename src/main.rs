mod components;
mod map;
mod map_indexing_system;
mod resources;
mod trees_system;
mod ui;

use bevy::{core::FixedTimestep, prelude::*};
use components::*;
use map::Map;
use map_indexing_system::map_indexing_system;
use resources::*;
use trees_system::trees_system;
use ui::mouse_system;

const TIME_STEP: f32 = 1.0 / 60.0;
const SCALE: i32 = 30;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Map::default())
        .insert_resource(MousePosition::default())
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9))) // background color
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(map_indexing_system)
                .with_system(trees_system)
                .with_system(mouse_system),
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        .insert(GridPoint { x: 0, y: 0 });
}
