use sdl2::render::{Renderer, Texture};
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use game::Game;
use common::*;
use sdl2_image::LoadTexture;
use std::path::Path;


pub struct Visualizer<'a> {
    texture: Texture,
    background: Texture,
    foreground: Texture,
    start_screen: Texture,
    lost_screen: Texture,
    win_screen: Texture,
    renderer: Renderer<'a>,
}

impl<'a> Visualizer<'a> {
    pub fn new(renderer: Renderer<'a>) -> Visualizer<'a> {
        Visualizer {
            texture: renderer.load_texture(Path::new("./assets/Floor.png")).unwrap(),
            background: renderer.load_texture(Path::new("./assets/background.png")).unwrap(),
            foreground: renderer.load_texture(Path::new("./assets/fog.png")).unwrap(),
            start_screen: renderer.load_texture(Path::new("./assets/start_screen.png")).unwrap(),
            lost_screen: renderer.load_texture(Path::new("./assets/lost_screen.png")).unwrap(),
            win_screen: renderer.load_texture(Path::new("./assets/win_screen.png")).unwrap(),
            renderer: renderer,
        }
    }


    pub fn draw(&mut self, game: &Game, lag: f64) {
        let _ = self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        let _ = self.renderer.clear();
        match game.screen {
            Screen::Start => self.draw_start(),
            Screen::Game => self.draw_game(game),
            Screen::Lost => self.draw_lost(),
            Screen::Win => self.draw_win(),
        }
        let _ = self.renderer.present();
    }

    fn draw_lost(&mut self) {
        self.renderer.copy(&self.lost_screen, None, None);
    }

    fn draw_start(&mut self) {
        self.renderer.copy(&self.start_screen, None, None);
    }

    fn draw_win(&mut self) {
        self.renderer.copy(&self.win_screen, None, None);
    }

    fn draw_game(&mut self, game: &Game) {
        let (mut x_offset, mut y_offset) = game.player.global_pos();
        x_offset -= (DEF_WINDOW_WIDTH as i32)/2;
        y_offset -= (DEF_WINDOW_HEIGHT as i32)/2;
        // BACKGROUND
        let texture_rect = Rect::new(x_offset/4,
                                     y_offset/4,
                                     DEF_WINDOW_WIDTH,
                                     DEF_WINDOW_HEIGHT);
        self.renderer.copy(&self.background, Some(texture_rect), None);
        // FLOOR
        let tiles = game.tiles.get_tiles(x_offset as f64 - (TILE_SIZE) as f64,
                                         y_offset as f64 - (TILE_SIZE) as f64,
                                         (DEF_WINDOW_WIDTH + 2*TILE_SIZE) as i32,
                                         (DEF_WINDOW_HEIGHT + 2*TILE_SIZE) as i32,
                                         FLOOR_LAYER_IND);
        for tile in tiles.values() {
            let rect = Rect::new(tile.x - x_offset, tile.y - y_offset, tile.width, tile.height);
            //if !visible.contains(&(tile.x, tile.y)) {
                //continue;
            //}
            let texture_x = if let TileType::Floor(texture_x) = tile.tile_info {
                texture_x
            }
            else {
                panic!("Errous tile passed to floor layer")
            };
            let texture_rect = Rect::new((TILE_SIZE*texture_x) as i32,
                                         FLOOR_TEXTURE_Y as i32,
                                         TILE_SIZE,
                                         TILE_SIZE);
            self.renderer.copy(&self.texture, Some(texture_rect), Some(rect));
        }
        // BODIES
        for unit in game.dead_enemies.iter() {
            let unit_pos = unit.global_pos();
            let rect = Rect::new(unit_pos.0 - x_offset,
                                 unit_pos.1 - y_offset,
                                 TILE_SIZE,
                                 TILE_SIZE);
            let texture_rect = Rect::new((TILE_SIZE*1) as i32,
                                         ENEMY_TEXTURE_Y as i32,
                                         TILE_SIZE,
                                         TILE_SIZE);
            self.renderer.copy(&self.texture, Some(texture_rect), Some(rect));
        }
        // PORTALS
        for portal in game.portals.iter() {
            let portal_pos = portal.tile;
            let rect = Rect::new(portal_pos.0*TILE_SIZE as i32 - x_offset,
                                 portal_pos.1*TILE_SIZE as i32 - y_offset,
                                 TILE_SIZE,
                                 TILE_SIZE);
            let texture_rect = Rect::new((TILE_SIZE*0) as i32,
                                         PORTAL_TEXTURE_Y as i32,
                                         TILE_SIZE,
                                         TILE_SIZE);
            self.renderer.copy(&self.texture, Some(texture_rect), Some(rect));
        }
        // ENEMIES
        for unit in game.enemies.iter() {
            let unit_pos = unit.global_pos();
            let rect = Rect::new(unit_pos.0 - x_offset - 10,
                                 unit_pos.1 - y_offset - 10,
                                 TILE_SIZE + 20,
                                 TILE_SIZE + 20);
            let texture_rect = Rect::new((TILE_SIZE*0) as i32,
                                         LIGHT_TEXTURE_Y as i32,
                                         TILE_SIZE,
                                         TILE_SIZE);
            self.renderer.copy(&self.texture, Some(texture_rect), Some(rect));
            let rect = Rect::new(unit_pos.0 - x_offset,
                                 unit_pos.1 - y_offset,
                                 TILE_SIZE,
                                 TILE_SIZE);
            let texture_rect = Rect::new((TILE_SIZE*0) as i32,
                                         ENEMY_TEXTURE_Y as i32,
                                         TILE_SIZE,
                                         TILE_SIZE);
            self.renderer.copy(&self.texture, Some(texture_rect), Some(rect));
        }
        // PLAYER
        let rect = Rect::new((DEF_WINDOW_WIDTH as i32)/2 - 10,
                             (DEF_WINDOW_HEIGHT as i32)/2 - 10,
                             TILE_SIZE + 20,
                             TILE_SIZE + 20);
        let texture_rect = Rect::new((TILE_SIZE*0) as i32,
                                     LIGHT_TEXTURE_Y as i32,
                                     TILE_SIZE,
                                     TILE_SIZE);
        self.renderer.copy(&self.texture, Some(texture_rect), Some(rect));
        let rect = Rect::new((DEF_WINDOW_WIDTH as i32)/2,
                             (DEF_WINDOW_HEIGHT as i32)/2,
                             TILE_SIZE,
                             TILE_SIZE);
        let texture_rect = Rect::new((TILE_SIZE*0) as i32,
                                     PLAYER_TEXTURE_Y as i32,
                                     TILE_SIZE,
                                     TILE_SIZE);
        self.renderer.copy(&self.texture, Some(texture_rect), Some(rect));
        // FOREGROUND
        let texture_rect = Rect::new(1920 - x_offset/4,
                                     1080 - y_offset/4,
                                     DEF_WINDOW_WIDTH,
                                     DEF_WINDOW_HEIGHT);
        self.renderer.copy(&self.foreground, Some(texture_rect), None);
        // HITPOINTS
        let rect = Rect::new((DEF_WINDOW_WIDTH - BAR_WIDTH - 50) as i32 + 2,
                              20,
                              ((BAR_WIDTH as f64)/(PLAYER_MAX_HP as f64)*(game.player.hp as f64)) as u32 - 2,
                              BAR_HEIGHT);
        let texture_rect = Rect::new(0,
                                     HP_BAR_Y as i32,
                                     BAR_WIDTH,
                                     BAR_HEIGHT);
        self.renderer.copy(&self.texture, Some(texture_rect), Some(rect));
        // BORDER
        let rect = Rect::new((DEF_WINDOW_WIDTH - BAR_WIDTH - 50) as i32,
                              20,
                              BAR_WIDTH,
                              BAR_HEIGHT);
        let texture_rect = Rect::new(0,
                                     BAR_BORDER_Y as i32,
                                     BAR_WIDTH,
                                     BAR_HEIGHT);
        self.renderer.copy(&self.texture, Some(texture_rect), Some(rect));
        // MANA BARS
        for i in 0..PLAYER_MAX_MANA - 1 {
            let rect = Rect::new((DEF_WINDOW_WIDTH - BAR_WIDTH - 50) as i32 + i*20,
                                 20 + BAR_HEIGHT as i32 + 5,
                                 MANA_BAR_WIDTH,
                                 BAR_HEIGHT);
            if game.portals_passed > i {
                let texture_rect = Rect::new(0,
                                             MANA_BAR_Y as i32,
                                             MANA_BAR_WIDTH,
                                             BAR_HEIGHT);
                self.renderer.copy(&self.texture, Some(texture_rect), Some(rect));
            }
            let texture_rect = Rect::new(0,
                                         MANA_BAR_BORDER_Y as i32,
                                         MANA_BAR_WIDTH,
                                         BAR_HEIGHT);
            self.renderer.copy(&self.texture, Some(texture_rect), Some(rect));
        }
    }
}
