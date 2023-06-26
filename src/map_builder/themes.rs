use crate::prelude::*;

use super::{rooms::RoomsArchitect, drunkard::DrunkardsWalkArchitect, automata::CellularAutomataArchitect};

pub struct DungeonTheme {}

impl DungeonTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('.'),
            TileType::Wall => to_cp437('#'),
            TileType::Exit => to_cp437('>'),
        }
    }

    fn get_architect(&self) -> Box<dyn super::MapArchitect> {
        Box::new(RoomsArchitect {})
    }
}

pub struct ForestTheme {}

impl ForestTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437(';'),
            TileType::Wall => to_cp437('"'),
            TileType::Exit => to_cp437('>'),
        }
    }

    fn get_architect(&self) -> Box<dyn super::MapArchitect> {
        let mut rng = RandomNumberGenerator::new();
        match rng.range(0, 1) {
            0 => Box::new(DrunkardsWalkArchitect {}),
            _ => Box::new(CellularAutomataArchitect {})
        }
    }
}
