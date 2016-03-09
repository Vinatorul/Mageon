use tile_engine::TileEngine;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use dungeon_generator::{BSPGenerator, Room, DungeonGenerator};
use std::collections::HashSet;
use unit::Unit;
use common::*;
use rand::{SeedableRng, StdRng, Rng};

const TEMP_WIDTH: u32 = 800;
const TEMP_HEIGHT: u32 = 600;
const TILE_SCALE: u32 = 4;
const TEMP_TILE_SIZE: u32 = TILE_SIZE/TILE_SCALE;

pub struct Game {
    pub tiles: TileEngine<TileType>,
    pub player: Unit,
    pub enemies: Vec<Unit>,
    rooms: Vec<Room>,
    pressed_keys: HashSet<Keycode>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            tiles: TileEngine::<TileType>::default(),
            player: Unit::new((0, 0)),
            enemies: vec![],
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
            Keycode::Up | Keycode::J => {
                self.make_move((0, -1));
            },
            Keycode::Down | Keycode::K => {
                self.make_move((0, 1));
            },
            Keycode::Left | Keycode::H => {
                self.make_move((-1, 0));
            },
            Keycode::Right | Keycode::L => {
                self.make_move((1, 0));
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
        self.player.tile = ((rng.gen_range(players_start_room.x, players_start_room.x + players_start_room.width)/TEMP_TILE_SIZE) as i32,
                            (rng.gen_range(players_start_room.y, players_start_room.y + players_start_room.height)/TEMP_TILE_SIZE) as i32);
        for room in self.rooms.iter() {
            let unit_pos = ((rng.gen_range(room.x, room.x + room.width)/TEMP_TILE_SIZE) as i32,
                           (rng.gen_range(room.y, room.y + room.height)/TEMP_TILE_SIZE) as i32);
            self.enemies.push(Unit::new(unit_pos));
            let mut x = (room.x/TEMP_TILE_SIZE)*TEMP_TILE_SIZE;
            while x < room.x + room.width {
                let mut y = (room.y/TEMP_TILE_SIZE)*TEMP_TILE_SIZE;
                while y < room.y + room.height {
                    self.tiles.add_tile((TILE_SCALE*x) as i32,
                                        (TILE_SCALE*y) as i32,
                                        TILE_SIZE,
                                        TILE_SIZE,
                                        FLOOR_LAYER_IND,
                                        TileType::Floor(rng.gen_range(0, 4)));
                    y += TEMP_TILE_SIZE;
                }
                x += TEMP_TILE_SIZE;
            }
        }
    }

    fn is_empty(&self, pos: (i32, i32)) -> bool {
        let (pos_x, pos_y) = global_pos(pos);
        let mut enemy_collide = false;
        for enemy in self.enemies.iter() {
            if (pos.0 == enemy.tile.0) && (pos.1 == enemy.tile.1) {
                enemy_collide = true;
                break;
            }
        }
        enemy_collide || self.tiles.tile_at(pos_x, pos_y, FLOOR_LAYER_IND).is_none()
    }

    fn make_move(&mut self, delta: (i32, i32)) {
        if self.is_empty((self.player.tile.0 + delta.0,
                          self.player.tile.1 + delta.1)) {
            return
        }
        self.player.make_move(delta);
        // AI Works here
        for i in 0..self.enemies.len() {
            let mv = self.enemies[i].ai.get_move(&self.enemies[i], &self);
            self.enemies[i].make_move(mv);
        }
    }
}
