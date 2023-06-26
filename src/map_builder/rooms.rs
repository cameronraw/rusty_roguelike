use super::{themes::DungeonTheme, MapArchitect};
use crate::prelude::*;

pub struct RoomsArchitect {}

impl MapArchitect for RoomsArchitect {
    fn create_map_builder(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            spawn_locations: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            theme: DungeonTheme::new(),
        };

        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();
        mb.amulet_start = mb.find_most_distant();
        for room in mb.rooms.iter().skip(1) {
            mb.spawn_locations.push(SpawnLocation {
                point: room.center(),
                preferred_entity: None,
            });
        }

        mb
    }
}
