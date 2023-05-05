use pyo3::prelude::*;
use once_cell::sync::Lazy;
use crate::game::*;

static ALL_MOVES: Lazy<Vec<Move>> = Lazy::new(|| {
    let mut all_moves = Vec::new();
    for x1 in 0..3 {
        for y1 in 0..3 {
            for x2 in 0..3 {
                for y2 in 0..3 {
                    let move_ = Move((x1, y1), (x2, y2));
                    all_moves.push(move_);
                }
            }
        }
    }
    all_moves
});

pub fn move_to_action(m: Move) -> u8 {
    let Move((x1, y1), (x2, y2)) = m;
    x1 * 27 + y1 * 9 + x2 * 3 + y2
}

pub fn action_to_move(a: u8) -> Move {
    ALL_MOVES[a as usize]
}

fn create_observation(game: &Game) -> [[[u8; 9]; 9]; 3] {
    // observation is a 3x9x9 tensor
    // first layer contains the current player's marks
    // second layer contains the opponent's marks
    // third layer indicates the current player
    let mut observation = [[[0; 9]; 9]; 3];
    let current_player = game.current_player();
    let board = game.board();

    for x in 0..9 {
        for y in 0..9 {
            let field = board[x][y];
            match field {
                Some(player) => if player == Player::X {
                    observation[0][x][y] = 1
                } else {
                    observation[1][x][y] = 1
                }
                None => {}
            }
        }
    }

    if current_player == Player::X {
        observation[2] = [[1; 9]; 9];
    }

    observation
}

#[pyclass]
#[derive(Clone)]
pub struct UTTTEnvImpl {
    pub game: Game,
}

#[pymethods]
impl UTTTEnvImpl {
    #[new]
    pub fn new() -> Self {
        Self {
            game: Game::new(),
        }
    }

    pub fn reset(&mut self) {
        self.game = Game::new();
    }

    pub fn step(&mut self, action: u8) -> ([[[u8; 9]; 9]; 3], f32, bool) {
        let move_ = action_to_move(action);
        if self.game.move_valid(move_) {
            self.game.make_move(move_);
        } else {
            for &move_ in ALL_MOVES.iter() {
                if self.game.move_valid(move_) {
                    self.game.make_move(move_);
                    break;
                }
            }
        }
        let observation = create_observation(&self.game);
        let reward = self.reward();
        let done = self.done();
        (observation, reward, done)
    }

    pub fn render(&self) {
        println!("{}", self.game);
    }

    pub fn valid_actions(&self) -> Vec<u8> {
        let mut valid_actions = Vec::new();
        for (i, &move_) in ALL_MOVES.iter().enumerate() {
            if self.game.move_valid(move_) {
                valid_actions.push(i as u8);
            }
        }
        valid_actions
    }

    pub fn current_player(&self) -> u8 {
        match self.game.current_player() {
            Player::X => 0,
            Player::O => 1,
        }
    }

    pub fn done(&self) -> bool {
        self.game.is_over()
    }

    pub fn reward(&self) -> f32 {
        match self.game.winner() {
            Some(Player::X) => 1.0,
            Some(Player::O) => -1.0,
            None => 0.0,
        }
    }
}
