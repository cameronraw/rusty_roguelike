use bracket_lib::terminal::{ColorPair, to_cp437, Point, BLACK, WHITE};
use legion::World;

use crate::components::{Render, Player};

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Player,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@')
            }
        )
    );
}
