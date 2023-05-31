use bracket_lib::{terminal::{ColorPair, to_cp437, Point, BLACK, WHITE, FontCharType}, random::RandomNumberGenerator};
use legion::World;

use crate::{components::{Render, Player}, prelude::{Enemy, MovingRandomly, Health, Name}};

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Player,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@')
            },
            Health {
                current: 20,
                max: 20
            }
        )
    );
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc()
    };
    ecs.push(
        (Enemy, pos, Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,        
        },
            MovingRandomly {},
            Health { current: hp, max: hp },
            Name(name)
        )
    );
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}
