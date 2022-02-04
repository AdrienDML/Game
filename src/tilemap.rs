mod grid;
mod heightmap;
mod tile;

use bevy::prelude::Plugin;
pub use grid::*;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {}
}
