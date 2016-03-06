use common::*;

pub struct Unit {
    pub tile: (i32, i32),
}

impl Unit {
    pub fn new(tile: (i32, i32)) -> Unit {
        Unit {
            tile: tile,
        }
    }

    pub fn global_pos(&self) -> (i32, i32) {
        (self.tile.0*(TILE_SIZE) as i32,
         self.tile.1*(TILE_SIZE) as i32)
    }
}
