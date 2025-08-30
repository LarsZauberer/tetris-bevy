use crate::{
    constants::{HEIGHT, WIDTH},
    utils::{BlockType, CurrentBlock},
};
use bevy::prelude::*;
use std::fmt;

/// The World keeps track of what type which tiles are.
#[derive(Resource)]
pub struct World {
    pub grid: [[BlockType; WIDTH as usize]; HEIGHT as usize],
    pub current: CurrentBlock,
}

impl World {
    pub fn new() -> Self {
        Self {
            grid: [[BlockType::No; WIDTH as usize]; HEIGHT as usize],
            current: CurrentBlock::new(),
        }
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for block in row {
                write!(f, "{}", block)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Resource)]
pub struct GameTick(pub Timer);
