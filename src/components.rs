use bevy::prelude::*;
use std::cmp::{Eq, PartialEq};

#[derive(Component)]
pub struct Tree {}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Component, Default)]
pub struct GridPoint {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Component, Default)]
pub struct GName {
    pub text: String,
}

#[derive(Component)]
pub struct MainCamera;
