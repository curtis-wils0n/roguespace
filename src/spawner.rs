use super::{
    AreaOfEffect, BlocksTile, CombatStats, Confusion, Consumable, DefenseBonus, EquipmentSlot,
    Equippable, InflictsDamage, Item, MAP_WIDTH, Map, MeleePowerBonus, Monster, Name, Player,
    Position, ProvidesHealing, Ranged, Rect, Renderable, SerializeMe, TileType, Viewshed,
};
use crate::random_table::RandomTable;
use rltk::{RGB, RandomNumberGenerator};
use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SimpleMarker};
use std::collections::HashMap;

const MAX_MONSTERS: i32 = 4;

pub fn player(ecs: &mut World, player_x: i32, player_y: i32) -> Entity {
    ecs.create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: 25,
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            render_order: 0,
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name {
            name: "Player".to_string(),
        })
        .with(CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

fn orc(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, 127, RGB::named(rltk::GREEN2), "Orc");
}
fn goblin(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, 123, RGB::named(rltk::GREEN2), "Goblin");
}

fn monster<S: ToString>(
    ecs: &mut World,
    x: i32,
    y: i32,
    glyph: rltk::FontCharType,
    fg: RGB,
    name: S,
) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph,
            fg,
            bg: RGB::named(rltk::BLACK),
            render_order: 1,
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Monster {})
        .with(Name {
            name: name.to_string(),
        })
        .with(BlocksTile {})
        .with(CombatStats {
            max_hp: 16,
            hp: 16,
            defense: 1,
            power: 4,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn health_potion(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: 669,
            fg: RGB::named(rltk::MAGENTA),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "Health Potion".to_string(),
        })
        .with(Item {})
        .with(Consumable {})
        .with(ProvidesHealing { heal_amount: 8 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn create_base_scroll<S: ToString>(
    ecs: &mut World,
    x: i32,
    y: i32,
    name: S,
    fg: RGB,
) -> EntityBuilder<'_> {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: 768,
            fg,
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: name.to_string(),
        })
        .with(Item {})
        .with(Consumable {})
        .with(Ranged { range: 6 })
        .marked::<SimpleMarker<SerializeMe>>()
}

fn magic_missile_scroll(ecs: &mut World, x: i32, y: i32) {
    create_base_scroll(ecs, x, y, "Magic Missile Scroll", RGB::named(rltk::CYAN))
        .with(InflictsDamage { damage: 8 })
        .build();
}

fn fireball_scroll(ecs: &mut World, x: i32, y: i32) {
    create_base_scroll(ecs, x, y, "Fireball Scroll", RGB::named(rltk::ORANGE))
        .with(InflictsDamage { damage: 20 })
        .with(AreaOfEffect { radius: 3 })
        .build();
}

fn confusion_scroll(ecs: &mut World, x: i32, y: i32) {
    create_base_scroll(ecs, x, y, "Confusion Scroll", RGB::named(rltk::PINK))
        .with(Confusion { turns: 4 })
        .build();
}

type EntitySpawner = for<'a> fn(ecs: &'a mut World, x: i32, y: i32);

fn room_table(map_depth: i32) -> RandomTable<EntitySpawner> {
    RandomTable::<EntitySpawner>::new()
        .add(goblin, 10)
        .add(orc, 1 + map_depth)
        .add(health_potion, 7)
        .add(fireball_scroll, 2 + map_depth)
        .add(confusion_scroll, 2 + map_depth)
        .add(magic_missile_scroll, 4)
        .add(dagger, 3)
        .add(shield, 3)
        .add(longsword, map_depth - 1)
        .add(tower_shield, map_depth - 1)
}

pub fn spawn_room(ecs: &mut World, room: &Rect, map_depth: i32, map: &Map) {
    let spawn_table = room_table(map_depth);
    let mut spawn_points: HashMap<usize, Option<EntitySpawner>> = HashMap::new();
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        let num_spawns = rng.roll_dice(1, MAX_MONSTERS + 3) + (map_depth - 1) - 3;

        for _ in 0..num_spawns {
            let mut tries = 0;
            while tries < 20 {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAP_WIDTH) + x;

                if spawn_points.contains_key(&idx) {
                    // We're already spawning something at this location, so we try again
                    tries += 1;
                    continue;
                }

                // Don't spawn on DownStairs tiles
                if map.tiles[idx] == TileType::DownStairs {
                    tries += 1;
                    continue;
                }

                // We've found a good spot, so we add a spawn and move on
                spawn_points.insert(idx, spawn_table.roll(&mut rng));
                break;
            }
        }
    }

    for (spawn_index, spawner) in spawn_points.iter() {
        let x = (*spawn_index % MAP_WIDTH) as i32;
        let y = (*spawn_index / MAP_WIDTH) as i32;

        if let Some(spawner) = spawner {
            spawner(ecs, x, y);
        }
    }
}

fn dagger(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: 330,
            fg: RGB::named(rltk::CYAN),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "Dagger".to_string(),
        })
        .with(Item {})
        .with(Equippable {
            slot: EquipmentSlot::Melee,
        })
        .with(MeleePowerBonus { power: 2 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn shield(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: 138,
            fg: RGB::named(rltk::CYAN),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "Shield".to_string(),
        })
        .with(Item {})
        .with(Equippable {
            slot: EquipmentSlot::Shield,
        })
        .with(DefenseBonus { defense: 2 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn longsword(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: 378,
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "Longsword".to_string(),
        })
        .with(Item {})
        .with(Equippable {
            slot: EquipmentSlot::Melee,
        })
        .with(MeleePowerBonus { power: 4 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn tower_shield(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: 187,
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "Tower Shield".to_string(),
        })
        .with(Item {})
        .with(Equippable {
            slot: EquipmentSlot::Shield,
        })
        .with(DefenseBonus { defense: 3 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}
