mod template;

use crate::{
    components::{Player, Render},
    prelude::{AmuletOfYala, FieldOfView, Health, Item, Name, Damage},
};
use bracket_lib::{
    random::RandomNumberGenerator,
    terminal::{to_cp437, ColorPair, Point, BLACK, WHITE},
};
use legion::World;

use self::template::Templates;

pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point],
) {
    let template = Templates::load();
    template.spawn_entities(ecs, rng, level, spawn_points);
}

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player { map_level: 0 },
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 10,
            max: 10,
        },
        FieldOfView::new(8),
        Damage(1)
    ));
}

pub fn spawn_amulet_of_yala(ecs: &mut World, position: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        position,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name("Amulet of Yala".to_string()),
    ));
}
