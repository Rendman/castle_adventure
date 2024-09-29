use bevy::prelude::*;

pub enum CardDir {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: char,
    pub fg: Color,
    pub bg: Color
}

#[derive(Component)]
pub struct CardinalDirection {
    pub dir: CardDir,
}