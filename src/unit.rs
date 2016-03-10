use ai::{AI, ChaserAI};
use common::*;

const ANIMATION_LENGTH: u32 = 16;

pub struct Unit {
    pub tile: (i32, i32),
    pub ai: Box<AI>,
    pub hp: i32,
    move_animation_tick: u32,
    move_delta: (i32, i32),
}

impl Unit {
    pub fn new(tile: (i32, i32)) -> Unit {
        Unit {
            tile: tile,
            ai: Box::new(ChaserAI),
            hp: 100,
            move_animation_tick: 0,
            move_delta: (0, 0)
        }
    }

    pub fn make_move(&mut self, delta: (i32, i32)) {
        self.move_animation_tick = ANIMATION_LENGTH;
        self.move_delta = delta;
        self.tile.0 += delta.0;
        self.tile.1 += delta.1;
    }

    pub fn global_pos(&self) -> (i32, i32) {
        let coeff = (self.move_animation_tick as f64)/(ANIMATION_LENGTH as f64);
        (self.tile.0*TILE_SIZE as i32 - ((self.move_delta.0 as f64)*(TILE_SIZE as f64)*coeff) as i32,
         self.tile.1*TILE_SIZE as i32 - ((self.move_delta.1 as f64)*(TILE_SIZE as f64)*coeff) as i32)
    }

    pub fn update(&mut self) {
        if self.move_animation_tick > 0 {
            self.move_animation_tick -= 1;
        }
        // skipping last frame
        if self.move_animation_tick == 1 {
            self.move_delta = (0, 0);
        }
    }

    pub fn is_moving(&self) -> bool {
        self.move_delta != (0, 0)
    }
}
