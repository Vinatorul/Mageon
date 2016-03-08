use game::Game;
use unit::Unit;

mod melee;

pub use ai::melee::MeleeAI;

pub trait AI {
    fn get_move(&self, unit: &Unit, game: &Game) -> (i32, i32);
}
