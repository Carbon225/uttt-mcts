use super::{SmallBoard, Player, get_winner, Move};
use std::iter::zip;

#[derive(Clone)]
pub struct BigBoard {
    fields: [[SmallBoard; 3]; 3],
    n_placed: u8,
    winner: Option<Player>,
}

impl BigBoard {
    pub fn new() -> Self {
        Self {
            fields: [[SmallBoard::new(), SmallBoard::new(), SmallBoard::new()],
                     [SmallBoard::new(), SmallBoard::new(), SmallBoard::new()],
                     [SmallBoard::new(), SmallBoard::new(), SmallBoard::new()]],
            n_placed: 0,
            winner: None,
        }
    }

    pub fn at(&self, (x, y): (u8, u8)) -> &SmallBoard {
        &self.fields[x as usize][y as usize]
    }

    fn simplify(&self) -> [[Option<Player>; 3]; 3] {
        let mut result = [[None; 3]; 3];
        for (row, field_row) in zip(result.iter_mut(), self.fields.iter()) {
            for (field, small_board) in zip(row.iter_mut(), field_row.iter()) {
                *field = small_board.get_winner();
            }
        }
        result
    }

    pub fn get_winner(&self) -> Option<Player> {
        self.winner
    }

    pub fn is_full(&self) -> bool {
        self.n_placed == 81
    }

    pub fn is_over(&self) -> bool {
        self.winner.is_some() || self.is_full()
    }

    pub fn place(&mut self, player: Player, move_: Move) {
        self.fields[move_.0.0 as usize][move_.0.1 as usize].place(player, move_.1);
        self.n_placed += 1;
        self.winner = get_winner(&self.simplify())
    }
}
