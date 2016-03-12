use ai::{AI, ChaserAI};
use common::*;

pub const ANIMATION_LENGTH: u32 = 16;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Animation {
    Idle,
    Move,
    Atack,
}

pub struct Unit {
    pub tile: (i32, i32),
    pub ai: Box<AI>,
    pub hp: i32,
    pub damage: i32,
    animation: Animation,
    animation_tick: u32,
    animation_delay: u32,
    delta: (i32, i32),
}

impl Unit {
    pub fn new(tile: (i32, i32), hp: i32, dmg: i32) -> Unit {
        Unit {
            tile: tile,
            ai: Box::new(ChaserAI),
            hp: hp,
            damage: dmg,
            animation: Animation::Idle,
            animation_tick: 0,
            animation_delay: 0,
            delta: (0, 0)
        }
    }

    pub fn make_move(&mut self, delta: (i32, i32), delay: u32) {
        self.animation_tick = ANIMATION_LENGTH;
        self.delta = delta;
        self.animation = Animation::Move;
        self.tile.0 += delta.0;
        self.tile.1 += delta.1;
        self.animation_delay = delay;
    }

    pub fn atack(&mut self, delta: (i32, i32), delay: u32) {
        self.animation_tick = ANIMATION_LENGTH;
        self.delta = delta;
        self.animation = Animation::Atack;
        self.animation_delay = delay;
    }

    pub fn takes_damage(&mut self, damage: i32) {
        self.hp -= damage;
    }

    pub fn global_pos(&self) -> (i32, i32) {
        let mut result = (self.tile.0*TILE_SIZE as i32, self.tile.1*TILE_SIZE as i32);
        let coef = match self.animation {
            Animation::Move => {
                -(self.animation_tick as f64)/(ANIMATION_LENGTH as f64)
            },
            Animation::Atack => {
                if self.animation_tick > 2*ANIMATION_LENGTH/3 {
                    -((ANIMATION_LENGTH - self.animation_tick) as f64)/(ANIMATION_LENGTH as f64)
                }
                else {
                    (self.animation_tick as f64)/(ANIMATION_LENGTH as f64)
                }
            },
            Animation::Idle => {
                0.
            }
        };
        result.0 += ((self.delta.0 as f64)*(TILE_SIZE as f64)*coef) as i32;
        result.1 += ((self.delta.1 as f64)*(TILE_SIZE as f64)*coef) as i32;
        result
    }

    pub fn update(&mut self) {
        if self.animation_delay > 0 {
            self.animation_delay -= 1;
            return;
        }
        if self.animation_tick > 0 {
            self.animation_tick -= 1;
        }
        // skipping last frames
        if self.animation_tick == 1 {
            self.animation = Animation::Idle;
        }
    }

    pub fn is_animation_playing(&self) -> bool {
        self.animation != Animation::Idle
    }

    pub fn alive(&self) -> bool {
        self.hp > 0
    }

    pub fn stop_animation(&mut self) {
        self.animation = Animation::Idle
    }
}
