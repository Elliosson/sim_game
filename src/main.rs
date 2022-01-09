use bevy::{core::FixedTimestep, prelude::*};
use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;

const TIME_STEP: f32 = 1.0 / 60.0;
const SCALE: i32 = 30;

#[derive(Component)]
struct Tree {}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Component)]
struct GridPoint {
    x: i32,
    y: i32,
}

struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles_content: HashMap<GridPoint, Vec<Entity>>,
}

impl Default for Map {
    fn default() -> Self {
        Map {
            width: 100,
            height: 100,
            tiles_content: HashMap::new(),
        }
    }
}

impl Map {
    // creating some interface because I wonder if I change the storage model of the map
    pub fn clear(&mut self) {
        self.tiles_content.clear();
    }

    pub fn add(&mut self, point: GridPoint, entity: Entity) {
        let tile_content = self.tiles_content.entry(point).or_default();
        tile_content.push(entity)
    }

    pub fn get(&self, point: GridPoint) -> Vec<Entity> {
        let tile_content = self.tiles_content.get(&point);
        if let Some(content) = tile_content {
            return content.clone();
        } else {
            return Vec::new();
        };
    }
}

//TODO do somthing coherant for the convertion
fn translation_to_point(translation: Vec3) -> GridPoint {
    let point = GridPoint {
        x: (translation.x as i32 / SCALE),
        y: (translation.y as i32 / SCALE),
    };

    return point;
}

fn point_to_translation(point: GridPoint) -> Vec3 {
    let translation = Vec3::new((point.x * SCALE) as f32, (point.y * SCALE) as f32, 1.);
    return translation;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Map::default())
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9))) // background color
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(map_indexing_system)
                .with_system(tree_system),
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn map_indexing_system(
    mut commands: Commands,
    mut map: ResMut<Map>,
    transforms: Query<(Entity, &GridPoint)>,
) {
    //populate the map here

    map.clear();

    for (entity, point) in transforms.iter() {
        map.add(point.clone(), entity);
    }
}

fn tree_system(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut tree_query: Query<(&mut Tree, &Transform)>,
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
