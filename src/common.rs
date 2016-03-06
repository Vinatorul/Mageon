pub const TILE_SIZE: u32 = 40;
pub const DEF_WINDOW_WIDTH: u32 = 800;
pub const DEF_WINDOW_HEIGHT: u32 = 600;

pub const WALL_LAYER_IND: i32 = 1;
pub const FLOOR_LAYER_IND: i32 = 2;

pub const PLAYER_TEXTURE_Y: u32 = 0;
pub const WALL_TEXTURE_Y: u32 = PLAYER_TEXTURE_Y + TILE_SIZE;
pub const FLOOR_TEXTURE_Y: u32 = WALL_TEXTURE_Y + TILE_SIZE;
