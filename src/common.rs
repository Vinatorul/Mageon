pub const TILE_SIZE: u32 = 40;
pub const BAR_WIDTH: u32 = 160;
pub const MANA_BAR_WIDTH: u32 = 15;
pub const BAR_HEIGHT: u32 = 20;
pub const DEF_WINDOW_WIDTH: u32 = 800;
pub const DEF_WINDOW_HEIGHT: u32 = 600;
pub const PLAYER_MAX_HP: i32 = 50;
pub const PLAYER_MAX_MANA: i32 = 4;

pub const FLOOR_LAYER_IND: i32 = 2;

pub const PLAYER_TEXTURE_Y: u32 = 0;
pub const FLOOR_TEXTURE_Y: u32 = PLAYER_TEXTURE_Y + TILE_SIZE;
pub const ENEMY_TEXTURE_Y: u32 = FLOOR_TEXTURE_Y + TILE_SIZE;
pub const LIGHT_TEXTURE_Y: u32 = ENEMY_TEXTURE_Y + TILE_SIZE;
pub const HP_BAR_Y: u32 = LIGHT_TEXTURE_Y + TILE_SIZE;
pub const BAR_BORDER_Y: u32 = HP_BAR_Y + BAR_HEIGHT;
pub const PORTAL_TEXTURE_Y: u32 = BAR_BORDER_Y + BAR_HEIGHT;
pub const MANA_BAR_Y: u32 = PORTAL_TEXTURE_Y + TILE_SIZE;
pub const MANA_BAR_BORDER_Y: u32 = MANA_BAR_Y + BAR_HEIGHT;

#[derive(Debug)]
pub enum TileType {
    None,
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

pub fn index(i: i32, j: i32) -> usize {
    (j*(DEF_WINDOW_WIDTH/TILE_SIZE) as i32 + i) as usize
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Screen {
    Start,
    Game,
    Lost,
    Win,
}

//pub fn get_fov(tiles: &::std::collections::HashMap<(i32, i32), &::tile_engine::Tile<TileType>>, start_pos: (i32, i32)) -> ::std::collections::HashSet<(i32, i32)> {
    //use std::collections::{HashSet, VecDeque};
    //enum FovType {
        //EastAxis,
        //NorthEastQuadrant,
        //NorthAxis,
        //NorthWestQuadrant,
        //WestAxis,
        //SouthWestQuadrant,
        //SouthAxis,
        //SouthEastQuadrant,
    //}

    //let mut lighted = HashSet::<(i32, i32)>::new();
    //let mut queued = HashSet::<(i32, i32)>::new();
    //let mut queue = VecDeque::<(i32, i32, FovType)>::new();
    //let push_queue = |q: &mut VecDeque<(i32, i32, FovType)>,
                          //pos: (i32, i32, FovType),
                          //queued: &mut HashSet<(i32, i32)> | {
        //if queued.contains(&(pos.0, pos.1)) {
            //return;
        //}
        //queued.insert((pos.0, pos.1));
        //q.push_back(pos);
    //};
    //lighted.insert(global_pos(start_pos));
    //let mut shadows = Vec::<(f64, f64)>::new();
    //push_queue(&mut queue, (start_pos.0 + 1, start_pos.1, FovType::EastAxis), &mut queued);
    //push_queue(&mut queue, (start_pos.0, start_pos.1 - 1, FovType::NorthAxis), &mut queued);
    //push_queue(&mut queue, (start_pos.0 - 1, start_pos.1, FovType::WestAxis), &mut queued);
    //push_queue(&mut queue, (start_pos.0, start_pos.1 + 1, FovType::SouthAxis), &mut queued);
    //while !queue.is_empty() {
        //let mut pos = queue.pop_front().unwrap();
        //lighted.insert(global_pos((pos.0, pos.1)));
        //if !tiles.contains_key(&global_pos((pos.0, pos.1))) {
             //TODO: count shadow
            //continue;
        //}
         //TODO: check shadow
        //match pos.2 {
            //FovType::EastAxis => {
                //push_queue(&mut queue, (pos.0, pos.1 + 1, FovType::SouthEastQuadrant), &mut queued);
                //push_queue(&mut queue, (pos.0 + 1, pos.1, FovType::EastAxis), &mut queued);
                //push_queue(&mut queue, (pos.0, pos.1 - 1, FovType::NorthEastQuadrant), &mut queued);
            //},
            //FovType::NorthEastQuadrant => {
                //push_queue(&mut queue, (pos.0 + 1, pos.1, FovType::NorthEastQuadrant), &mut queued);
                //push_queue(&mut queue, (pos.0, pos.1 - 1, FovType::NorthEastQuadrant), &mut queued);
            //},
            //FovType::NorthAxis => {
                //push_queue(&mut queue, (pos.0 + 1, pos.1, FovType::NorthEastQuadrant), &mut queued);
                //push_queue(&mut queue, (pos.0, pos.1 - 1, FovType::NorthAxis), &mut queued);
                //push_queue(&mut queue, (pos.0 - 1, pos.1, FovType::NorthWestQuadrant), &mut queued);
            //},
            //FovType::NorthWestQuadrant => {
                //push_queue(&mut queue, (pos.0, pos.1 - 1, FovType::NorthWestQuadrant), &mut queued);
                //push_queue(&mut queue, (pos.0 - 1, pos.1, FovType::NorthWestQuadrant), &mut queued);
            //},
            //FovType::WestAxis => {
                //push_queue(&mut queue, (pos.0, pos.1 - 1, FovType::NorthWestQuadrant), &mut queued);
                //push_queue(&mut queue, (pos.0 - 1, pos.1, FovType::WestAxis), &mut queued);
                //push_queue(&mut queue, (pos.0, pos.1 + 1, FovType::SouthWestQuadrant), &mut queued);
            //},
            //FovType::SouthWestQuadrant => {
                //push_queue(&mut queue, (pos.0 - 1, pos.1, FovType::SouthWestQuadrant), &mut queued);
                //push_queue(&mut queue, (pos.0, pos.1 + 1, FovType::SouthWestQuadrant), &mut queued);
            //},
            //FovType::SouthAxis => {
                //push_queue(&mut queue, (pos.0 - 1, pos.1, FovType::SouthWestQuadrant), &mut queued);
                //push_queue(&mut queue, (pos.0, pos.1 + 1, FovType::SouthAxis), &mut queued);
                //push_queue(&mut queue, (pos.0 + 1, pos.1, FovType::SouthEastQuadrant), &mut queued);
            //}
            //FovType::SouthEastQuadrant => {
                //push_queue(&mut queue, (pos.0, pos.1 + 1, FovType::SouthEastQuadrant), &mut queued);
                //push_queue(&mut queue, (pos.0 + 1, pos.1, FovType::SouthEastQuadrant), &mut queued);
            //},
        //}
    //}
    //lighted
//}
