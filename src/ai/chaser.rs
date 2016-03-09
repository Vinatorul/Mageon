use ai::AI;
use game::Game;
use unit::Unit;
use common::*;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct ChaserAI;

impl AI for ChaserAI {
    fn get_move(&self, unit: &Unit, game: &Game) -> (i32, i32) {
        let tiles = game.tiles.get_tiles((unit.tile.0*TILE_SIZE as i32) as f64 - (DEF_WINDOW_WIDTH/2) as f64,
                                         (unit.tile.1*TILE_SIZE as i32) as f64 - (DEF_WINDOW_HEIGHT/2) as f64,
                                         DEF_WINDOW_WIDTH as i32,
                                         DEF_WINDOW_HEIGHT as i32,
                                         FLOOR_LAYER_IND);
        if !tiles.contains_key(&global_pos(game.player.tile)) {
            return (0, 0);
        }
        let mut table = Vec::<i32>::new();
        let width = (DEF_WINDOW_WIDTH/TILE_SIZE) as i32;
        let height = (DEF_WINDOW_HEIGHT/TILE_SIZE) as i32;
        let start = (unit.tile.0 - width/2, unit.tile.1 - height/2);
        // Mark tiles passable/unpassable
        for j in start.1..(start.1 + height) {
            for i in start.0..(start.0 + width) {
                if tiles.contains_key(&global_pos((i, j))) {
                    table.push(1);
                }
                else {
                    table.push(-100);
                }
            }
        }
        let index = |i: i32, j: i32| -> usize {
            (j*width + i) as usize
        };
        // mark allies as unpassable
        for ally in game.enemies.iter() {
            if (ally.tile.0 == unit.tile.0) && (ally.tile.1 == unit.tile.1) {
                continue;
            }
            let unit_pos = (ally.tile.0 - start.0, ally.tile.1 - start.1);
            if (unit_pos.0 >= 0) && (unit_pos.1 >= 0) && (unit_pos.0 < width) && (unit_pos.1 < height) {
                table[index(unit_pos.0, unit_pos.1)] = -100;
            }
        }
        // Prioritize positions near player
        let player_pos = (game.player.tile.0 - start.0, game.player.tile.1 - start.1);
        if (player_pos.0 < 0) || (player_pos.1 < 0) || (player_pos.0 >= width) || (player_pos.1 >= height) {
            return (0, 0);
        }
        table[index(player_pos.0, player_pos.1)] = 3000;
        // path finding
        let mut queue = VecDeque::<(i32, i32)>::new();
        queue.push_back(player_pos);
        while !queue.is_empty() {
            let pos = queue.pop_front().unwrap();
            if (pos.0 > 0) && (table[index(pos.0 - 1, pos.1)] == 1) {
                table[index(pos.0 - 1, pos.1)] = table[index(pos.0, pos.1)] - 50;
                queue.push_back((pos.0 - 1, pos.1));
            }
            if (pos.0 < width - 1) && (table[index(pos.0 + 1, pos.1)] == 1) {
                table[index(pos.0 + 1, pos.1)] = table[index(pos.0, pos.1)] - 50;
                queue.push_back((pos.0 + 1, pos.1));
            }
            if (pos.1 > 0) && (table[index(pos.0, pos.1 - 1)] == 1) {
                table[index(pos.0, pos.1 - 1)] = table[index(pos.0, pos.1)] - 50;
                queue.push_back((pos.0, pos.1 - 1));
            }
            if (pos.1 < height - 1) && (table[index(pos.0, pos.1 + 1)] == 1) {
                table[index(pos.0, pos.1 + 1)] = table[index(pos.0, pos.1)] - 50;
                queue.push_back((pos.0, pos.1 + 1));
            }
        }
        table[index(player_pos.0, player_pos.1)] = -100;
        let self_pos = (unit.tile.0 - start.0, unit.tile.1 - start.1);
        let mut current_value = table[index(self_pos.0, self_pos.1)];
        let mut mv = (0, 0);
        let temp_value = table[index(self_pos.0 - 1, self_pos.1)];
        if temp_value > current_value {
            mv = (-1, 0);
            current_value = temp_value;
        }
        let temp_value = table[index(self_pos.0 + 1, self_pos.1)];
        if temp_value > current_value {
            mv = (1, 0);
            current_value = temp_value;
        }
        let temp_value = table[index(self_pos.0, self_pos.1 - 1)];
        if temp_value > current_value {
            mv = (0, -1);
            current_value = temp_value;
        }
        let temp_value = table[index(self_pos.0, self_pos.1 + 1)];
        if temp_value > current_value {
            mv = (0, 1);
            current_value = temp_value;
        }
        mv
    }
}