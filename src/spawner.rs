use crate::prelude::*;

//TODO: Add monster to spawn with the amulet

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
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph, fov) = match rng.roll_dice(1, 10) {
        1..=5 => goblin(),
        6..=8 => double_headed(),
        _ => orc(),
    };
    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        ChasingPlayer {},
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
        FieldOfView::new(fov),
    ));
}

fn goblin() -> (i32, String, FontCharType, i32) {
    (1, "Goblin".to_string(), to_cp437('g'), 5)
}

fn orc() -> (i32, String, FontCharType, i32) {
    (2, "Orc".to_string(), to_cp437('o'), 6)
}

fn double_headed() -> (i32, String, FontCharType, i32) {
    (4, "Double".to_string(), to_cp437('E'), 10)
}

pub fn spawn_amulet_of_justicialismo(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name("Dije justicialista".to_string()),
    ));
}

pub fn spawn_entity(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let roll = rng.roll_dice(1, 10);
    match roll {
        1..=2 => spawn_healing_potion(ecs, pos),
        3 => spawn_magic_map(ecs, pos),
        _ => spawn_monster(ecs, rng, pos),
    }
}

pub fn spawn_healing_potion(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('!'),
        },
        Name("Poción sanadora".to_string()),
        ProvidesHealing { amount: 6 },
    ));
}

pub fn spawn_magic_map(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('{'),
        },
        Name("Mapa secreto".to_string()),
        ProvidesDungeonMap {},
    ));
}
