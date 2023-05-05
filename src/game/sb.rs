use super::{Player, get_winner};

#[derive(Clone)]
pub struct SmallBoard {
    fields: [[Option<Player>; 3]; 3],
    n_placed: u8,
    winner: Option<Player>,
}

impl SmallBoard {
    pub fn new() -> Self {
        Self {
            fields: [[None; 3]; 3],
            n_placed: 0,
            winner: None,
        }
    }

    pub fn at(&self, (x, y): (u8, u8)) -> Option<Player> {
        self.fields[x as usize][y as usize]
    }

    pub fn get_winner(&self) -> Option<Player> {
        self.winner
    }

    pub fn is_full(&self) -> bool {
        self.n_placed == 9
    }

    pub fn is_over(&self) -> bool {
        self.winner.is_some() || self.is_full()
    }

    pub fn place(&mut self, player: Player, (x, y): (u8, u8)) {
        self.fields[x as usize][y as usize] = Some(player);
        self.n_placed += 1;
        if self.n_placed >= 3 {
            self.winner = get_winner(&self.fields)
        }
    }
}
