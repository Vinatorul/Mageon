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
    renderer: Renderer<'a>,
}

impl<'a> Visualizer<'a> {
    pub fn new(renderer: Renderer<'a>) -> Visualizer<'a> {
        Visualizer {
            texture: renderer.load_texture(Path::new("./assets/Floor.png")).unwrap(),
            background: renderer.load_texture(Path::new("./assets/background.png")).unwrap(),
            renderer: renderer,
        }
    }

    pub fn draw(&mut self, game: &Game, lag: f64) {
        let _ = self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        let _ = self.renderer.clear();

        let (mut x_offset, mut y_offset) = global_pos(game.player.tile);
        x_offset -= (DEF_WINDOW_WIDTH as i32)/2;
        y_offset -= (DEF_WINDOW_HEIGHT as i32)/2;
        let tiles = game.tiles.get_tiles(x_offset as f64 - (TILE_SIZE) as f64,
                                         y_offset as f64 - (TILE_SIZE) as f64,
                                         (DEF_WINDOW_WIDTH + 2*TILE_SIZE) as i32,
                                         (DEF_WINDOW_HEIGHT + 2*TILE_SIZE) as i32,
                                         FLOOR_LAYER_IND);
        //let visible = get_fov(&tiles, game.player.tile);
        let texture_rect = Rect::new(x_offset/12,
                                     y_offset/12,
                                     DEF_WINDOW_WIDTH,
                                     DEF_WINDOW_HEIGHT);
        self.renderer.copy(&self.background, Some(texture_rect), None);
        // draw floor
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
        for unit in game.enemies.iter() {
            let rect = Rect::new(unit.tile.0*TILE_SIZE as i32 - x_offset - 10,
                                 unit.tile.1*TILE_SIZE as i32 - y_offset - 10,
                                 TILE_SIZE + 20,
                                 TILE_SIZE + 20);
            let texture_rect = Rect::new((TILE_SIZE*0) as i32,
                                         LIGHT_TEXTURE_Y as i32,
                                         TILE_SIZE,
                                         TILE_SIZE);
            self.renderer.copy(&self.texture, Some(texture_rect), Some(rect));
            let rect = Rect::new(unit.tile.0*TILE_SIZE as i32 - x_offset,
                                 unit.tile.1*TILE_SIZE as i32 - y_offset,
                                 TILE_SIZE,
                                 TILE_SIZE);
            let texture_rect = Rect::new((TILE_SIZE*0) as i32,
                                         ENEMY_TEXTURE_Y as i32,
                                         TILE_SIZE,
                                         TILE_SIZE);
            self.renderer.copy(&self.texture, Some(texture_rect), Some(rect));
        }
        let _ = self.renderer.present();
    }
}
