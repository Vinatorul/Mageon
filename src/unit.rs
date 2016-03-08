use common::*;
use game::Game;
use ai::{AI, MeleeAI};

pub struct Unit {
    pub tile: (i32, i32),
    pub ai: Box<AI>,
}

impl Unit {
    pub fn new(tile: (i32, i32)) -> Unit {
        Unit {
            tile: tile,
            ai: Box::new(MeleeAI),
        }
    }

    pub fn make_move(&mut self, delta: (i32, i32)) {
        self.tile.0 += delta.0;
        self.tile.1 += delta.1;
    }

    pub fn check_move(&self, delta: (i32, i32), game: &Game) -> bool {
        let (pos_x, pos_y) = global_pos(self.tile);
        let tiles = game.tiles.get_tiles((pos_x - (TILE_SIZE) as i32) as f64,
                                         (pos_y - (TILE_SIZE) as i32) as f64,
                                         (TILE_SIZE*3) as i32,
                                         (TILE_SIZE*3) as i32,
                                         2);
        let new_pos = (pos_x + delta.0*(TILE_SIZE) as i32,
                       pos_y + delta.1*(TILE_SIZE) as i32);
        let mut collide_enemy = false;
        for unit in game.enemies.iter() {
            if global_pos(unit.tile) == new_pos {
                collide_enemy = true;
                break;
            }
        }
        (new_pos != global_pos(game.player.tile)) && !collide_enemy && tiles.contains_key(&new_pos)
    }
}
