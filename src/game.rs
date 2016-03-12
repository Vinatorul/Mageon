use tile_engine::TileEngine;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use dungeon_generator::{BSPGenerator, Room, DungeonGenerator};
use std::collections::HashSet;
use unit::{self, Unit};
use common::*;
use rand::{SeedableRng, StdRng, Rng};
use portal::Portal;

const TEMP_WIDTH: u32 = 800;
const TEMP_HEIGHT: u32 = 600;
const TILE_SCALE: u32 = 4;
const TEMP_TILE_SIZE: u32 = TILE_SIZE/TILE_SCALE;

pub struct Game {
    pub tiles: TileEngine<TileType>,
    pub player: Unit,
    pub enemies: Vec<Unit>,
    pub dead_enemies: Vec<Unit>,
    pub portals: Vec<Portal>,
    pub portals_passed: i32,
    block_input: bool,
    rooms: Vec<Room>,
    pressed_keys: HashSet<Keycode>,
    rand: StdRng,
}

impl Game {
    pub fn new() -> Game {
        Game {
            tiles: TileEngine::<TileType>::default(),
            player: Unit::new((0, 0), PLAYER_MAX_HP, 5),
            enemies: vec![],
            dead_enemies: vec![],
            portals: vec![],
            portals_passed: 0,
            block_input: false,
            rooms: vec![],
            pressed_keys: HashSet::<Keycode>::new(),
            rand: StdRng::new().unwrap(),
        }
    }

    pub fn update(&mut self) {
        if self.block_input {
            let mut block_input = self.player.is_animation_playing();
            if block_input {
                self.player.update();
            }
            else {
                for unit in self.enemies.iter_mut() {
                    unit.update();
                    block_input = block_input || unit.is_animation_playing();
                }
            }
            self.block_input = block_input;
        }
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
        if self.block_input || self.pressed_keys.contains(&key) {
            return;
        }
        self.pressed_keys.insert(key);
        match key {
            Keycode::Up | Keycode::J | Keycode::Kp8 => {
                self.make_move((0, -1));
            },
            Keycode::Down | Keycode::K | Keycode::Kp2 => {
                self.make_move((0, 1));
            },
            Keycode::Left | Keycode::H | Keycode::Kp4 => {
                self.make_move((-1, 0));
            },
            Keycode::Right | Keycode::L | Keycode::Kp6 => {
                self.make_move((1, 0));
            },
            Keycode::Space | Keycode::Period | Keycode::Kp5 => {
                self.make_move((0, 0));
            }
            Keycode::Y | Keycode::Kp7 => {
                self.make_move((-1, -1));
            }
            Keycode::U | Keycode::Kp9 => {
                self.make_move((1, -1));
            }
            Keycode::B | Keycode::Kp1 => {
                self.make_move((-1, 1));
            }
            Keycode::N | Keycode::Kp3 => {
                self.make_move((1, 1));
            }
            _ => {},
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
        let mut portal_dests = Vec::<usize>::new();
        for room in self.rooms.iter() {
            let unit_pos = ((rng.gen_range(room.x, room.x + room.width)/TEMP_TILE_SIZE) as i32,
                           (rng.gen_range(room.y, room.y + room.height)/TEMP_TILE_SIZE) as i32);
            let portal_pos = ((rng.gen_range(room.x, room.x + room.width)/TEMP_TILE_SIZE) as i32,
                           (rng.gen_range(room.y, room.y + room.height)/TEMP_TILE_SIZE) as i32);
            portal_dests.push(rng.gen_range(0, self.rooms.len()));
            self.enemies.push(Unit::new(unit_pos, 5, 5));
            self.portals.push(Portal::new(portal_pos));
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
        for i in 0..portal_dests.len() {
            let room = self.rooms[portal_dests[i]];
            let portal_dest = ((rng.gen_range(room.x, room.x + room.width)/TEMP_TILE_SIZE) as i32,
                               (rng.gen_range(room.y, room.y + room.height)/TEMP_TILE_SIZE) as i32);
            self.portals[i].connect(portal_dest);
        }
    }

    fn is_empty(&self, pos: (i32, i32)) -> bool {
        let (pos_x, pos_y) = global_pos(pos);
        self.tiles.tile_at(pos_x, pos_y, FLOOR_LAYER_IND).is_none()
    }

    fn make_move(&mut self, delta: (i32, i32)) {
        let new_pos = (self.player.tile.0 + delta.0,
                       self.player.tile.1 + delta.1);
        if self.is_empty(new_pos) {
                              return
                          }
        self.block_input = true;
        let mut enemy_collide_ind = -1;
        for i in 0..self.enemies.len() {
            if (new_pos.0 == self.enemies[i].tile.0) && (new_pos.1 == self.enemies[i].tile.1) {
                enemy_collide_ind = i as i32;
                break;
            }
        }
        if enemy_collide_ind >= 0 {
            self.player.atack(delta, 0);
            if self.rand.gen_range(0, 10) != 3 { // 10% to miss
                self.enemies[enemy_collide_ind as usize].takes_damage(self.player.damage + self.rand.gen_range(-2, 2));
                if !self.enemies[enemy_collide_ind as usize].alive() {
                    self.dead_enemies.push(self.enemies.remove(enemy_collide_ind as usize));
                }
            }
        }
        else {
            let mut enter_portal_ind = -1;
            for i in 0..self.portals.len() {
                if (new_pos.0 == self.portals[i].tile.0) && (new_pos.1 == self.portals[i].tile.1) {
                    enter_portal_ind = i as i32;
                    break;
                }
            }
            if enter_portal_ind == -1 {
                self.player.make_move(delta, 0);
            }
            else {
                let new_delta = (self.portals[enter_portal_ind as usize].destination.0 - self.player.tile.0,
                                 self.portals[enter_portal_ind as usize].destination.1 - self.player.tile.1);
                self.portals.remove(enter_portal_ind as usize);
                self.portals_passed += 1;
                if self.portals_passed == PLAYER_MAX_MANA {
                    self.player.hp = PLAYER_MAX_HP;
                    self.portals_passed = 0;
                }
                self.player.make_move(new_delta, 0);
            }
        }
        // AI Works here
        for i in 0..self.enemies.len() {
            let mv = self.enemies[i].ai.get_move(&self.enemies[i], &self);
            let new_pos = (self.enemies[i].tile.0 + mv.0, self.enemies[i].tile.1 + mv.1);
            if new_pos == self.player.tile {
                self.enemies[i].atack(mv, self.rand.gen_range(0, unit::ANIMATION_LENGTH));
                if self.rand.gen_range(0, 10) == 7 { // 10% to miss
                    continue;
                }
                self.player.takes_damage(self.enemies[i].damage + self.rand.gen_range(-2, 2));
            }
            else {
                let mut enter_portal_ind = -1;
                for i in 0..self.portals.len() {
                    if (new_pos.0 == self.portals[i].tile.0) && (new_pos.1 == self.portals[i].tile.1) {
                        enter_portal_ind = i as i32;
                        break;
                    }
                }
                if enter_portal_ind == -1 {
                    self.enemies[i].make_move(mv, self.rand.gen_range(0, unit::ANIMATION_LENGTH));
                }
                else {
                    let new_delta = (self.portals[enter_portal_ind as usize].destination.0 - self.enemies[i].tile.0,
                                     self.portals[enter_portal_ind as usize].destination.1 - self.enemies[i].tile.1);
                    self.enemies[i].make_move(new_delta, self.rand.gen_range(0, unit::ANIMATION_LENGTH));
                }
            }
        }
    }
}
