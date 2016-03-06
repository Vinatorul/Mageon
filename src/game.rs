use tile_engine::TileEngine;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use dungeon_generator::{BSPGenerator, Room, DungeonGenerator, RoomType};
use std::collections::HashSet;

const TEMP_WIDTH: u32 = 800;
const TEMP_HEIGHT: u32 = 600;
const TILE_SIZE: u32 = 10;

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
    pub x_offset: i32,
    pub y_offset: i32,
    rooms: Vec<Room>,
    pressed_keys: HashSet<Keycode>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            tiles: TileEngine::<TileType>::default(),
            x_offset: 0,
            y_offset: 0,
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
                self.y_offset -= 3*TILE_SIZE as i32;
            },
            Keycode::Down => {
                self.y_offset += 3*TILE_SIZE as i32;
            },
            Keycode::Left => {
                self.x_offset -= 3*TILE_SIZE as i32;
            },
            Keycode::Right => {
                self.x_offset += 3*TILE_SIZE as i32;
            },
            _ => return,
        }

    }

    fn proc_key_up(&mut self, key: Keycode) {
        self.pressed_keys.remove(&key);
    }

    pub fn generate_level(&mut self, seed: &[usize]) {
        self.rooms = BSPGenerator::default().min_room_splittable_size(100).coridor_width(15).generate(seed, TEMP_WIDTH, TEMP_HEIGHT);
        for room in self.rooms.iter() {
            let layer_ind = match room.room_type {
                RoomType::BasicRoom => 2,
                RoomType::Coridor => 3,
                _ => unreachable!(),
            };
            let mut x = (room.x/TILE_SIZE)*TILE_SIZE;
            while x < room.x + room.width {
                let mut y = (room.y/TILE_SIZE)*TILE_SIZE;
                while y < room.y + room.height {
                    self.tiles.add_tile(3*x as i32, 3*y as i32, TILE_SIZE*3, TILE_SIZE*3, layer_ind, TileType::Floor(1));
                    y += TILE_SIZE;
                }
                x += TILE_SIZE;
            }
        }
    }
}
