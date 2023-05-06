mod sb;
mod bb;
mod player;
mod game;
mod utils;

pub use sb::SmallBoard;
pub use bb::BigBoard;
pub use player::Player;
pub use game::Game;

use utils::get_winner;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move(pub (u8, u8), pub (u8, u8));
