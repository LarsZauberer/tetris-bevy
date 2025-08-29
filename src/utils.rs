use bevy::prelude::*;

use crate::constants::{CUTOFF, HEIGHT, SPAWNLOCATION, UNIT, WIDTH};
use crate::resources::World;

/// This is a helper function that can convert a hex encoded i32 to a bevy Color
pub fn hex_color(hex: i32) -> Color {
    let mut hex = hex;

    let blue = (hex % 256) as f32 / 256.0;
    hex = hex / 256;
    let green = (hex % 256) as f32 / 256.0;
    hex = hex / 256;
    let red = (hex % 256) as f32 / 256.0;

    Color::srgb(red, green, blue)
}

/// This is a helper function that helps to compute the position of the grid tiles to the transform on screen
pub fn compute_grid_coordinate(x: usize, y: usize) -> (f32, f32) {
    (
        UNIT / 2.0 + UNIT * x as f32 - UNIT * WIDTH / 2.0,
        UNIT / 2.0 - UNIT * y as f32 + UNIT * HEIGHT / 2.0 - UNIT,
    )
}

/// Enum for all the different types a tile could be
/// This basically maps to color
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlockType {
    I,
    J,
    L,
    O,
    S,
    Z,
    T,
    No,
}

impl BlockType {
    pub fn get_color(&self) -> Color {
        match self {
            BlockType::I => hex_color(0x00ffff),
            BlockType::J => hex_color(0x0000ff),
            BlockType::L => hex_color(0xff7f00),
            BlockType::O => hex_color(0xffff00),
            BlockType::S => hex_color(0x00ff00),
            BlockType::Z => hex_color(0xff0000),
            BlockType::T => hex_color(0x800080),
            BlockType::No => hex_color(0x2b2b2b),
        }
    }
}

pub struct CurrentBlock {
    pub location: (i32, i32),
    pub kind: BlockType,
}

impl CurrentBlock {
    pub fn new(kind: BlockType) -> Self {
        Self {
            location: SPAWNLOCATION,
            kind: BlockType::I,
        }
    }
}

/// Returns all the basic positions in relation to the origin point of a block
pub fn get_locations(kind: BlockType) -> Vec<(i32, i32)> {
    match kind {
        BlockType::I => {
            vec![(0, 0), (0, 1), (0, -1), (0, -2)]
        }
        BlockType::J => {
            vec![(0, 0), (0, 1), (0, -1), (-1, -1)]
        }
        BlockType::L => {
            vec![(0, 0), (0, 1), (0, -1), (1, -1)]
        }
        BlockType::O => {
            vec![(0, 0), (0, 1), (1, 0), (1, 1)]
        }
        BlockType::T => {
            vec![(0, 0), (0, -1), (-1, 0), (1, 0)]
        }
        BlockType::S => {
            vec![(0, 0), (0, -1), (1, -1), (-1, 0)]
        }
        BlockType::Z => {
            vec![(0, 0), (0, -1), (-1, -1), (1, 0)]
        }
        BlockType::No => {
            vec![]
        }
    }
}

/// Fills all the tiles at the locations with the origin offset with the given filler block type
pub fn fill(
    world: &mut World,
    locs: Vec<(i32, i32)>,
    (off_x, off_y): (i32, i32),
    filler: BlockType,
) {
    for (a, b) in locs {
        if off_x + a < 0 || off_x + a >= WIDTH as i32 {
            continue;
        }
        if off_y + b < 0 || off_y + b >= HEIGHT as i32 {
            continue;
        }

        let x = off_x + a;
        let y = off_y + b;
        world.grid[y as usize][x as usize] = filler;
    }
}

/// Checks if the move position of the block is legal
pub fn valid_position(world: &World, locs: &[(i32, i32)], (off_x, off_y): (i32, i32)) -> bool {
    for (a, b) in locs {
        let x = off_x + a;
        let y = off_y + b;

        if x < 0 || x >= WIDTH as i32 {
            return false;
        }
        if y >= HEIGHT as i32 {
            // We don't do any height checking (above is infinite space)
            return false;
        }

        if y >= 0 && world.grid[y as usize][x as usize] != BlockType::No {
            // Bound check here is still needed
            return false;
        }
    }
    return true;
}

pub fn check_game_over(world: &World) -> bool {
    !world.grid[CUTOFF as usize]
        .iter()
        .all(|x| x == &BlockType::No)
}

pub fn row_clearing(world: &mut World) {
    // Check Row finished
    for y in 0..HEIGHT as usize {
        let mut could_be = true;
        for x in 0..WIDTH as usize {
            if world.grid[y][x] == BlockType::No {
                could_be = false;
            }
        }
        if could_be {
            // Row is cleared

            // Remove the row
            for x in 0..WIDTH as usize {
                world.grid[y][x] = BlockType::No;
            }

            // Remove all the rows above
            for i in 1..(y + 1) {
                let row = y - i;
                for x in 0..WIDTH as usize {
                    world.grid[row + 1][x] = world.grid[row][x];
                }
            }
        }
    }
}
