use bevy::prelude::*;

use crate::constants::{HEIGHT, UNIT, WIDTH};

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
#[derive(Copy, Clone)]
pub enum TileType {
    I,
    J,
    L,
    O,
    S,
    Z,
    T,
    No,
}

impl TileType {
    pub fn get_color(&self) -> Color {
        match self {
            TileType::I => hex_color(0x00ffff),
            TileType::J => hex_color(0x0000ff),
            TileType::L => hex_color(0xff7f00),
            TileType::O => hex_color(0xffff00),
            TileType::S => hex_color(0x00ff00),
            TileType::Z => hex_color(0xff0000),
            TileType::T => hex_color(0x800080),
            TileType::No => hex_color(0x2b2b2b),
        }
    }
}
