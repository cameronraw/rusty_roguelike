use bracket_lib::random::RandomNumberGenerator;
use bracket_lib::terminal::Point;
use legion::systems::CommandBuffer;
use legion::World;
use mockall::*;
use ron::de::from_reader;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::File;

use crate::prelude::*;

#[derive(Clone, Deserialize, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub frequency: i32,
    pub name: String,
    pub glyph: char,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
    pub base_damage: Option<i32>,
}

#[automock]
pub trait CanLoadEntities {
    fn load_entities(&self) -> Vec<Template>;
}

pub struct FileEntityLoader {
    file_path: String,
}

impl FileEntityLoader {
    pub fn new(file_path: String) -> Self {
        FileEntityLoader { file_path }
    }
}

impl CanLoadEntities for FileEntityLoader {
    fn load_entities(&self) -> Vec<Template> {
        let file = File::open(&self.file_path).expect("Failed to open file");
        let template_collection: TemplateCollection = from_reader(file).expect("Unable to load templates");
        template_collection.entities
    }
}

#[derive(Clone, Deserialize, Debug)]
struct TemplateCollection {
    pub entities: Vec<Template>
}


#[derive(Clone, Deserialize, Debug)]
pub struct TemplateSpawner<T>
where
    T: CanSpawnEntities,
{
    pub entities: Vec<Template>,
    #[serde(skip)]
    pub spawner: T,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum EntityType {
    Enemy,
    Item,
}

impl<T: CanSpawnEntities + Default> Default for TemplateSpawner<T> {
    fn default() -> Self {
        Self {
            entities: Default::default(),
            spawner: Default::default(),
        }
    }
}

#[derive(Default)]
pub struct Spawner {}

#[automock]
pub trait CanSpawnEntities {
    fn spawn_entity(&self, pt: &Point, template: &Template, commands: &mut CommandBuffer);
}

#[automock]
pub trait CanReadEntitiesFromFile {
    fn read_entities<T: CanSpawnEntities + Default + 'static>(&self) -> TemplateSpawner<T>;
}

#[derive(Default)]
pub struct FileReader {}

impl CanSpawnEntities for Spawner {
    fn spawn_entity(&self, pt: &Point, template: &Template, commands: &mut CommandBuffer) {
        let entity = commands.push((
            pt.clone(),
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437(template.glyph),
            },
            Name(template.name.clone()),
        ));
        match template.entity_type {
            EntityType::Item => commands.add_component(entity, Item {}),
            EntityType::Enemy => {
                commands.add_component(entity, Enemy {});
                commands.add_component(entity, FieldOfView::new(6));
                commands.add_component(entity, ChasingPlayer {});
                commands.add_component(
                    entity,
                    Health {
                        current: template.hp.unwrap(),
                        max: template.hp.unwrap(),
                    },
                )
            }
        }

        if let Some(effects) = &template.provides {
            effects
                .iter()
                .for_each(|(provides, n)| match provides.as_str() {
                    "Healing" => commands.add_component(entity, ProvidesHealing { amount: *n }),
                    "MagicMap" => commands.add_component(entity, ProvidesDungeonMap {}),
                    _ => {
                        println!("Warning: we don't know how to provide {}", provides)
                    }
                })
        }

        if let Some(damage) = &template.base_damage {
            commands.add_component(entity, Damage(*damage));
            if template.entity_type == EntityType::Item {
                commands.add_component(entity, Weapon {})
            }
        }
    }
}

impl<T> TemplateSpawner<T>
where
    T: CanSpawnEntities + Default + 'static,
{
    pub fn new(loader: &dyn CanLoadEntities, spawner: T) -> Self {
        let entities = loader.load_entities();
        TemplateSpawner { entities, spawner }
    }

    pub fn spawn_entities(
        &self,
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
    ) {
        let mut available_entities = Vec::new();
        self.entities
            .iter()
            .filter(|e| e.levels.contains(&level))
            .for_each(|t| {
                for _ in 0..t.frequency {
                    available_entities.push(t);
                }
            });
        let mut commands = CommandBuffer::new(ecs);
        spawn_points.iter().for_each(|pt| {
            if let Some(entity) = rng.random_slice_entry(&available_entities) {
                self.spawner.spawn_entity(pt, entity, &mut commands);
            }
        });
        commands.flush(ecs);
    }
}

#[cfg(test)]
pub mod test {

    use super::*;

    #[test]
    fn should_spawn_the_correct_entities() {
        let mut mock_spawner = MockCanSpawnEntities::new();
        let mut mock_loader = MockCanLoadEntities::new();
        let entities: Vec<Template> = vec![
            Template {
                entity_type: EntityType::Item,
                levels: HashSet::from([0, 1, 2]),
                frequency: 5,
                name: String::from("Test Potion"),
                glyph: '/',
                provides: Some(Vec::<(String, i32)>::from([(
                    String::from("ProvidesHealing"),
                    5,
                )])),
                hp: None,
                base_damage: None,
            },
            Template {
                entity_type: EntityType::Item,
                levels: HashSet::from([0, 1, 2]),
                frequency: 5,
                name: String::from("Test Potion"),
                glyph: '/',
                provides: Some(Vec::<(String, i32)>::from([(
                    String::from("ProvidesHealing"),
                    5,
                )])),
                hp: None,
                base_damage: None,
            },
            Template {
                entity_type: EntityType::Item,
                levels: HashSet::from([0, 1, 2]),
                frequency: 5,
                name: String::from("Test Potion"),
                glyph: '/',
                provides: Some(Vec::<(String, i32)>::from([(
                    String::from("ProvidesHealing"),
                    5,
                )])),
                hp: None,
                base_damage: None,
            },
        ];
        mock_loader.expect_load_entities().returning(move || entities.clone());
        mock_spawner.expect_spawn_entity().with(predicate::always(), predicate::always(), predicate::always())
            .returning(|_, _, _| ())
            .times(3);
        let templates = TemplateSpawner::new(&mock_loader, mock_spawner);
        let mut world = World::default();
        let mut rng = RandomNumberGenerator::new();
        templates.spawn_entities(&mut world, &mut rng, 0, &[Point::new(1, 1), Point::new(2,2), Point::new(3,3)]);
    }
}
