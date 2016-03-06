use sdl2::render::Renderer;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use game::Game;
use common::*;

pub struct Visualizer<'a> {
    renderer: Renderer<'a>,
}

impl<'a> Visualizer<'a> {
    pub fn new(renderer: Renderer<'a>) -> Visualizer<'a> {
        Visualizer {
            renderer: renderer,
        }
    }

    pub fn draw(&mut self, game: &Game, lag: f64) {
        let _ = self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        let _ = self.renderer.clear();

        let _ = self.renderer.set_draw_color(Color::RGB(170, 0, 0));
        let rect = Rect::new((DEF_WINDOW_WIDTH as i32)/2 + 5,
                             (DEF_WINDOW_HEIGHT as i32)/2 + 5,
                             TILE_SIZE*TILE_SCALE - 10,
                             TILE_SIZE*TILE_SCALE - 10).unwrap().unwrap();
        let _ = self.renderer.draw_rect(rect);

        let (mut x_offset, mut y_offset) = game.player.global_pos();
        x_offset -= (DEF_WINDOW_WIDTH as i32)/2;
        y_offset -= (DEF_WINDOW_HEIGHT as i32)/2;
        let _ = self.renderer.set_draw_color(Color::RGB(0, 0, 170));
        for tile in game.tiles.get_tiles(x_offset as f64,
                                         y_offset as f64,
                                         DEF_WINDOW_WIDTH as i32,
                                         DEF_WINDOW_HEIGHT as i32,
                                         2).values()
        {
            let rect = Rect::new(tile.x - x_offset, tile.y - y_offset, tile.width, tile.height).unwrap().unwrap();
            let _ = self.renderer.draw_rect(rect);
        }
        //let _ = self.renderer.set_draw_color(Color::RGB(0, 170, 0));
        //for tile in game.tiles.get_tiles(x_offset as f64,
                                         //y_offset as f64,
                                         //DEF_WINDOW_WIDTH as i32,
                                         //DEF_WINDOW_HEIGHT as i32,
                                         //3).values()
        //{
            //let rect = Rect::new(tile.x - x_offset, tile.y - y_offset, tile.width, tile.height).unwrap().unwrap();
            //let _ = self.renderer.draw_rect(rect);
        //}

        let _ = self.renderer.present();
    }
}
