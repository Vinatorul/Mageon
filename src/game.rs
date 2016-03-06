use tile_engine::TileEngine;
use sdl2::event::Event;
use dungeon_generator::{BSPGenerator, Room, DungeonGenerator, RoomType};

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
    rooms: Vec<Room>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            tiles: TileEngine::<TileType>::default(),
            rooms: vec![],
        }
    }

    pub fn update(&mut self) {

    }

    pub fn proc_event(&mut self, event: Event) {

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
                    self.tiles.add_tile(x as i32, y as i32, TILE_SIZE, TILE_SIZE, layer_ind, TileType::Floor(1));
                    y += TILE_SIZE;
                }
                x += TILE_SIZE;
            }
        }
    }
}
