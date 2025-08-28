use crate::{
    constants::{HEIGHT, WIDTH},
    utils::TileType,
};
use bevy::prelude::*;

/// The World keeps track of what type which tiles are.
#[derive(Resource)]
pub struct World {
    pub grid: [[TileType; 10]; 20],
}

impl World {
    pub fn new() -> Self {
        Self {
            grid: [[TileType::No; WIDTH as usize]; HEIGHT as usize],
        }
    }
}
