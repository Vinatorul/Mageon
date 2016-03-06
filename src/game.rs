use tile_engine::TileEngine;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use dungeon_generator::{BSPGenerator, Room, DungeonGenerator, RoomType};
use std::collections::HashSet;
use unit::Unit;
use common::*;
use rand::{SeedableRng, StdRng, Rng};

const TEMP_WIDTH: u32 = 800;
const TEMP_HEIGHT: u32 = 600;

#[derive(Debug)]
pub enum TileType {
    None,
    Floor(i32),
}

impl Default for TileType {
    fn default() -> TileType {
        TileType::None
    }
}

pub struct Game {
    pub tiles: TileEngine<TileType>,
    pub player: Unit,
    rooms: Vec<Room>,
    pressed_keys: HashSet<Keycode>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            tiles: TileEngine::<TileType>::default(),
            player: Unit::new((0, 0)),
            rooms: vec![],
            pressed_keys: HashSet::<Keycode>::new(),
        }
    }

    pub fn update(&mut self) {

    }

    pub fn proc_event(&mut self, event: Event) {
        match event {
            Event::KeyDown { keycode: Some(key), .. } =>
                self.proc_key_down(key),
            Event::KeyUp { keycode: Some(key), .. } =>
                self.proc_key_up(key),
            _   => return,
        }
    }

    fn proc_key_down(&mut self, key: Keycode) {
        if self.pressed_keys.contains(&key) {
            return;
        }
        self.pressed_keys.insert(key);
        match key {
            Keycode::Up => {
                if self.check_player_move((0, -1)) {
                    self.player.tile.1 -= 1;
                }
            },
            Keycode::Down => {
                if self.check_player_move((0, 1)) {
                    self.player.tile.1 += 1;
                }
            },
            Keycode::Left => {
                if self.check_player_move((-1, 0)) {
                    self.player.tile.0 -= 1;
                }
            },
            Keycode::Right => {
                if self.check_player_move((1, 0)) {
                    self.player.tile.0 += 1;
                }
            },
            _ => return,
        }

    }

    fn proc_key_up(&mut self, key: Keycode) {
        self.pressed_keys.remove(&key);
    }

    pub fn generate_level(&mut self, seed: &[usize]) {
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        self.rooms = BSPGenerator::default().min_room_splittable_size(100).coridor_width(15).generate(seed, TEMP_WIDTH, TEMP_HEIGHT);
        let players_start_room = &self.rooms[rng.gen_range(0, self.rooms.len())];
        self.player.tile = ((rng.gen_range(players_start_room.x, players_start_room.x + players_start_room.width)/TILE_SIZE) as i32,
                            (rng.gen_range(players_start_room.y, players_start_room.y + players_start_room.height)/TILE_SIZE) as i32);
        for room in self.rooms.iter() {
            let layer_ind = match room.room_type {
                RoomType::BasicRoom => 2,
                RoomType::Coridor => 2,
                _ => unreachable!(),
            };
            let mut x = (room.x/TILE_SIZE)*TILE_SIZE;
            while x < room.x + room.width {
                let mut y = (room.y/TILE_SIZE)*TILE_SIZE;
                while y < room.y + room.height {
                    self.tiles.add_tile((TILE_SCALE*x) as i32,
                                        (TILE_SCALE*y) as i32,
                                        TILE_SIZE*TILE_SCALE,
                                        TILE_SIZE*TILE_SCALE,
                                        layer_ind,
                                        TileType::Floor(1));
                    y += TILE_SIZE;
                }
                x += TILE_SIZE;
            }
        }
    }

    fn check_player_move(&self, delta: (i32, i32)) -> bool {
        let (pos_x, pos_y) = self.player.global_pos();
        let tiles = self.tiles.get_tiles((pos_x - (TILE_SIZE*TILE_SCALE) as i32) as f64,
                                         (pos_y - (TILE_SIZE*TILE_SCALE) as i32) as f64,
                                         (TILE_SIZE*TILE_SCALE*3) as i32,
                                         (TILE_SIZE*TILE_SCALE*3) as i32,
                                         2);
        let new_pos = (pos_x + delta.0*(TILE_SCALE*TILE_SIZE) as i32,
                       pos_y + delta.1*(TILE_SCALE*TILE_SIZE) as i32);
        tiles.contains_key(&new_pos)
    }
}
