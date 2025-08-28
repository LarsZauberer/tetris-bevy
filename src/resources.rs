use crate::{
    constants::{HEIGHT, WIDTH},
    utils::{BlockType, CurrentBlock},
};
use bevy::prelude::*;

/// The World keeps track of what type which tiles are.
#[derive(Resource)]
pub struct World {
    pub grid: [[BlockType; 10]; 20],
    pub current: CurrentBlock,
}

impl World {
    pub fn new() -> Self {
        Self {
            grid: [[BlockType::No; WIDTH as usize]; HEIGHT as usize],
            current: CurrentBlock::new(BlockType::L),
        }
    }
}

#[derive(Resource)]
pub struct GameTick(pub Timer);
