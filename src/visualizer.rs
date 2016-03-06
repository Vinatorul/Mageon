use sdl2::render::{Renderer, Texture};
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use game::{Game, TileType};
use common::*;
use sdl2_image::LoadTexture;
use std::path::Path;

pub struct Visualizer<'a> {
    texture: Texture,
    renderer: Renderer<'a>,
}

impl<'a> Visualizer<'a> {
    pub fn new(renderer: Renderer<'a>) -> Visualizer<'a> {
        Visualizer {
            texture: renderer.load_texture(Path::new("./assets/Floor.png")).unwrap(),
            renderer: renderer,
        }
    }

    pub fn draw(&mut self, game: &Game, lag: f64) {
        let _ = self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        let _ = self.renderer.clear();

        // wall
        for tile in game.tiles.get_tiles((TILE_SIZE) as f64,
                                         (TILE_SIZE) as f64,
                                         (DEF_WINDOW_WIDTH + 2*TILE_SIZE) as i32,
                                         (DEF_WINDOW_HEIGHT + 2*TILE_SIZE) as i32,
                                         WALL_LAYER_IND).values()
        {
            let rect = Rect::new(tile.x, tile.y, tile.width, tile.height);
            let texture_x = if let TileType::Wall(texture_x) = tile.tile_info {
                texture_x
            }
            else {
                panic!("Errous tile passed to wall layer")
            };
            let texture_rect = Rect::new((TILE_SIZE*texture_x) as i32,
                                         WALL_TEXTURE_Y as i32,
                                         TILE_SIZE,
                                         TILE_SIZE);
            self.renderer.copy(&self.texture, Some(texture_rect), Some(rect));
        }

        let (mut x_offset, mut y_offset) = game.player.global_pos();
        x_offset -= (DEF_WINDOW_WIDTH as i32)/2;
        y_offset -= (DEF_WINDOW_HEIGHT as i32)/2;
        // floor
        for tile in game.tiles.get_tiles(x_offset as f64 - (TILE_SIZE) as f64,
                                         y_offset as f64 - (TILE_SIZE) as f64,
                                         (DEF_WINDOW_WIDTH + 2*TILE_SIZE) as i32,
                                         (DEF_WINDOW_HEIGHT + 2*TILE_SIZE) as i32,
                                         FLOOR_LAYER_IND).values()
        {
            let rect = Rect::new(tile.x - x_offset, tile.y - y_offset, tile.width, tile.height);
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
        let rect = Rect::new((DEF_WINDOW_WIDTH as i32)/2,
                             (DEF_WINDOW_HEIGHT as i32)/2,
                             TILE_SIZE,
                             TILE_SIZE);
        let texture_rect = Rect::new((TILE_SIZE*0) as i32,
                                     PLAYER_TEXTURE_Y as i32,
                                     TILE_SIZE,
                                     TILE_SIZE);
        self.renderer.copy(&self.texture, Some(texture_rect), Some(rect));
        let _ = self.renderer.present();
    }
}
