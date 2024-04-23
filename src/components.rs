pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;
pub struct Enemy;
pub struct MovingRandomly;
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}
pub struct Health {
    pub current: i32,
    pub max: i32,
}
