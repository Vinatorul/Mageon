pub const TILE_SIZE: u32 = 40;
pub const DEF_WINDOW_WIDTH: u32 = 800;
pub const DEF_WINDOW_HEIGHT: u32 = 600;

pub const WALL_LAYER_IND: i32 = 1;
pub const FLOOR_LAYER_IND: i32 = 2;

pub const PLAYER_TEXTURE_Y: u32 = 0;
pub const WALL_TEXTURE_Y: u32 = PLAYER_TEXTURE_Y + TILE_SIZE;
pub const FLOOR_TEXTURE_Y: u32 = WALL_TEXTURE_Y + TILE_SIZE;
pub const ENEMY_TEXTURE_Y: u32 = FLOOR_TEXTURE_Y + TILE_SIZE;

#[derive(Debug)]
pub enum TileType {
    None,
    Wall(u32),
    Floor(u32),
}

impl Default for TileType {
    fn default() -> TileType {
        TileType::None
    }
}

pub fn global_pos(tile: (i32, i32)) -> (i32, i32) {
    (tile.0*(TILE_SIZE) as i32,
     tile.1*(TILE_SIZE) as i32)
}

pub fn get_fov(tiles: &::std::collections::HashMap<(i32, i32), &::tile_engine::Tile<TileType>>, start_pos: (i32, i32)) -> ::std::collections::HashSet<(i32, i32)> {
    use std::collections::{HashSet, VecDeque};
    enum FovType {
        EastAxis,
        NorthEastQuadrant,
        NorthAxis,
        NorthWestQuadrant,
        WestAxis,
        SouthWestQuadrant,
        SouthAxis,
        SouthEastQuadrant,
    }

    let mut lighted = HashSet::<(i32, i32)>::new();
    let mut queue = VecDeque::<(i32, i32, FovType)>::new();
    lighted.insert(global_pos(start_pos));
    let mut shadows = Vec::<(f64, f64)>::new();
    queue.push_back((start_pos.0 + 1, start_pos.1, FovType::EastAxis));
    queue.push_back((start_pos.0, start_pos.1 - 1, FovType::NorthAxis));
    queue.push_back((start_pos.0 - 1, start_pos.1, FovType::WestAxis));
    queue.push_back((start_pos.0, start_pos.1 + 1, FovType::SouthAxis));
    while !queue.is_empty() {
        let mut pos = queue.pop_front().unwrap();
        if !tiles.contains_key(&global_pos((pos.0, pos.1))) {
            // TODO: count shadow
        }
        // TODO: check shadow
        lighted.insert(global_pos((pos.0, pos.1)));
        match pos.2 {
            FovType::EastAxis => {
                queue.push_back((pos.0, pos.1 + 1, FovType::SouthEastQuadrant));
                queue.push_back((pos.0 + 1, pos.1, FovType::EastAxis));
                queue.push_back((pos.0, pos.1 - 1, FovType::NorthEastQuadrant));
            },
            FovType::NorthEastQuadrant => {
                queue.push_back((pos.0 + 1, pos.1, FovType::NorthEastQuadrant));
                queue.push_back((pos.0, pos.1 - 1, FovType::NorthEastQuadrant));
            },
            FovType::NorthAxis => {
                queue.push_back((pos.0 + 1, pos.1, FovType::NorthEastQuadrant));
                queue.push_back((pos.0, pos.1 - 1, FovType::NorthAxis));
                queue.push_back((pos.0 - 1, pos.1, FovType::NorthWestQuadrant));
            },
            FovType::NorthWestQuadrant => {
                queue.push_back((pos.0, pos.1 - 1, FovType::NorthWestQuadrant));
                queue.push_back((pos.0 - 1, pos.1, FovType::NorthWestQuadrant));
            },
            FovType::WestAxis => {
                queue.push_back((pos.0, pos.1 - 1, FovType::NorthWestQuadrant));
                queue.push_back((pos.0 - 1, pos.1, FovType::WestAxis));
                queue.push_back((pos.0, pos.1 + 1, FovType::SouthWestQuadrant));
            },
            FovType::SouthWestQuadrant => {
                queue.push_back((pos.0 - 1, pos.1, FovType::SouthWestQuadrant));
                queue.push_back((pos.0, pos.1 + 1, FovType::SouthWestQuadrant));
            },
            FovType::SouthAxis => {
                queue.push_back((pos.0 - 1, pos.1, FovType::SouthWestQuadrant));
                queue.push_back((pos.0, pos.1 + 1, FovType::SouthAxis));
                queue.push_back((pos.0 + 1, pos.1, FovType::SouthEastQuadrant));
            }
            FovType::SouthEastQuadrant => {
                queue.push_back((pos.0, pos.1 + 1, FovType::SouthEastQuadrant));
                queue.push_back((pos.0 + 1, pos.1, FovType::SouthEastQuadrant));
            },
        }
    }
    lighted
}
