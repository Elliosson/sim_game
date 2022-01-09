use bevy::prelude::*;
use std::cmp::{Eq, PartialEq};

#[derive(Component)]
pub struct Tree {}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Component)]
pub struct GridPoint {
    pub x: i32,
    pub y: i32,
}
