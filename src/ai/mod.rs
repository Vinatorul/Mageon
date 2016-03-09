use game::Game;
use unit::Unit;

mod chaser;

pub use ai::chaser::ChaserAI;

pub trait AI {
    fn get_move(&self, unit: &Unit, game: &Game) -> (i32, i32);
}
