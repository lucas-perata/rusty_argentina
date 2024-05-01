pub use crate::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player {
    pub map_level: u32,
}
pub struct Enemy;
pub struct MovingRandomly;
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}
pub struct Health {
    pub current: i32,
    pub max: i32,
}
pub struct Points {
    pub current: i32,
}
pub struct Name(pub String);
pub struct ChasingPlayer;
pub struct Item;
pub struct AmuletOfYala;
pub struct ProvidesHealing {
    pub amount: i32,
}
pub struct ProvidesDungeonMap;
pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity,
}
pub struct Damage(pub i32);
pub struct Weapon;

#[derive(Clone, Debug, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Carried(pub Entity);
