use bevy::prelude::*;

/// The TileComponent is a bevy component that sets which tile of the grid it maps to
#[derive(Component)]
pub struct TileComponent {
    pub x: usize,
    pub y: usize,
}
