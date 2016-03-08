use ai::AI;
use game::Game;
use unit::Unit;

pub struct MeleeAI;

impl AI for MeleeAI {
    fn get_move(&self, unit: &Unit, game: &Game) -> (i32, i32) {
        (0, 0)
    }
}
