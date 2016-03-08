use common::*;
use game::Game;
use ai::{AI, MeleeAI};

pub struct Unit {
    pub tile: (i32, i32),
    pub ai: Box<AI>,
    pub hp: i32,
}

impl Unit {
    pub fn new(tile: (i32, i32)) -> Unit {
        Unit {
            tile: tile,
            ai: Box::new(MeleeAI),
            hp: 100,
        }
    }

    pub fn make_move(&mut self, delta: (i32, i32)) {
        self.tile.0 += delta.0;
        self.tile.1 += delta.1;
    }

}
