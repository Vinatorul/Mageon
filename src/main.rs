extern crate sdl2;
extern crate rand;
extern crate tile_engine;
extern crate chrono;
extern crate dungeon_generator;
extern crate sdl2_image;

mod visualizer;
mod game;
mod unit;
mod common;
mod ai;
mod portal;

use sdl2::event::Event;
use visualizer::Visualizer;
use game::Game;
use chrono::{DateTime, UTC};
use std::time::Duration;
use sdl2_image::{INIT_PNG, INIT_JPG};

const MS_PER_UPDATE: i64 = 15;

fn main() {
    // start sdl2 with everything
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let mut lag = 0;
    let mut last_tick: DateTime<UTC> = UTC::now();

    let _image_context = sdl2_image::init(INIT_PNG | INIT_JPG).unwrap();

    // Create a window
    let window  = match video_ctx.window("Mageon", common::DEF_WINDOW_WIDTH, common::DEF_WINDOW_HEIGHT).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {:?}", err)
    };

    // Create a rendering context
    let renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {:?}", err)
    };

    let mut events = ctx.event_pump().unwrap();
    let mut game = Game::new();
    
    let mut visualizer = Visualizer::new(renderer);
    // loop until we receive a QuitEvent
    'event : loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,
                _               => game.proc_event(event),
            }
        }
        let ms = (UTC::now() - last_tick).num_milliseconds();
        last_tick = UTC::now();
        lag = lag + ms;
        while lag > MS_PER_UPDATE {
            game.update();
            lag = lag - MS_PER_UPDATE;
        }
        // println!("{}", 1000.0/(ms as f64));
        visualizer.draw(&game, (lag as f64)/(MS_PER_UPDATE as f64));
        std::thread::sleep(Duration::from_millis(1));
    }
}
