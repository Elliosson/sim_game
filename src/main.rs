use bevy::{core::FixedTimestep, prelude::*};

const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Component)]
struct Tree {}

struct Map {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Map {})
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9))) // background color
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(map_system),
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn map_system(
    mut commands: Commands,
    mut scoreboard: ResMut<Map>,
    mut ball_query: Query<(&Transform)>,
) {
    //populate the map here
}

fn tree_system(
    mut commands: Commands,
    mut scoreboard: ResMut<Map>,
    mut ball_query: Query<(&mut Tree, &Transform)>,
) {
    //stuff to do about tree go here
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add the game's entities to our world

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
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
        .insert(Tree {});

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
        .insert(Tree {});

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
        .insert(Tree {});
}
