use bevy::prelude::*;
use std::cmp::{Eq, PartialEq};

#[derive(Component, Debug, Default, Clone)]
pub struct Tree {}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Component, Default, Debug)]
pub struct GridPoint {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Component, Default, Debug)]
pub struct GName {
    pub text: String,
}

#[derive(Component, Debug, Default, Clone)]

pub struct MainCamera;
#[derive(Component, Debug, Default, Clone)]

pub struct Selectable;
