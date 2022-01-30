use bevy::prelude::*;
use bracket_pathfinding::prelude::Point;
use std::cmp::{Eq, PartialEq};

#[derive(Component, Debug, Default, Clone)]
pub struct Tree {}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Component, Default, Debug)]
pub struct GridPoint {
    pub x: i32,
    pub y: i32,
}

impl GridPoint {
    pub fn from_point(point: Point) -> Self {
        GridPoint {
            x: point.x,
            y: point.y,
        }
    }

    pub fn to_point(self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Clone, Component, Default, Debug)]
pub struct GName {
    pub text: String,
}

#[derive(Component, Debug, Default, Clone)]

pub struct MainCamera;

#[derive(Component, Debug, Default, Clone)]

pub struct Selectable;
#[derive(Component, Debug, Default, Clone)]

pub struct Colonist;

#[derive(Component, Debug, Default, Clone)]

pub struct Movable {
    pub speed: f32,
}

#[derive(Component, Debug, Default, Clone)]

pub struct WantMove {
    pub target: GridPoint,
}

#[derive(Component, Debug, Default, Clone)]
pub struct MoveCooldown {
    pub time_left: f32,
}

#[derive(Component, Debug, Default, Clone)]
pub struct Blocking {}
