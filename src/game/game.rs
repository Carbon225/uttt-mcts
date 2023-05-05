use super::{BigBoard, Player, Move};

#[derive(Clone)]
pub struct Game {
    board: BigBoard,
    current_player: Player,
    last_move: Option<Move>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: BigBoard::new(),
            current_player: Player::X,
            last_move: None,
        }
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn winner(&self) -> Option<Player> {
        self.board.get_winner()
    }

    pub fn is_over(&self) -> bool {
        self.board.is_over()
    }

    pub fn make_move(&mut self, move_: Move) {
        self.board.place(self.current_player, move_);
        self.current_player = self.current_player.other();
        self.last_move = Some(move_);
    }

    pub fn move_valid(&self, move_: Move) -> bool {
        if self.is_over() {
            false
        } else {
            match self.last_move {
                None => true,
                Some(last_move) =>
                    self.board.at(move_.0).at(move_.1).is_none() &&
                    (self.board.at(last_move.1).is_over() || last_move.1 == move_.0)
            }
        }
    }

    pub fn board(&self) -> [[Option<Player>; 9]; 9] {
        let mut board = [[None; 9]; 9];
        board.iter_mut().flatten().zip(FORMAT_ORDER.iter())
            .for_each(|(field, (i, j, k, l))|
                *field = self.board.at((*i, *j)).at((*k, *l)));
        board
    }

    pub fn valid_moves(&self) -> Vec<Move> {
        if self.is_over() {
            Vec::new()
        } else {
            match self.last_move {
                None => {
                    let mut result = Vec::new();
                    for i in 0..3 {
                        for j in 0..3 {
                            for k in 0..3 {
                                for l in 0..3 {
                                    result.push(Move((i, j), (k, l)));
                                }
                            }
                        }
                    }
                    result
                },
                Some(last_move) => {
                    let mut result = Vec::new();
                    if self.board.at(last_move.1).is_over() {
                        for i in 0..3 {
                            for j in 0..3 {
                                for k in 0..3 {
                                    for l in 0..3 {
                                        if self.board.at((i, j)).at((k, l)).is_none() {
                                            result.push(Move((i, j), (k, l)));
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        for i in 0..3 {
                            for j in 0..3 {
                                if self.board.at(last_move.1).at((i, j)).is_none() {
                                    result.push(Move(last_move.1, (i, j)));
                                }
                            }
                        }
                    }
                    result
                }
            }
        }
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let fields =
            FORMAT_ORDER
            .iter()
            .map(|(i, j, k, l)|
                if let Some(player) = self.board.at((*i, *j)).at((*k, *l)) {
                    format!("{}", player)
                } else {
                    " ".to_owned()
                }
            );

        let mut board_string = BOARD_STRING.replace("X", "{}");
        for field in fields {
            board_string = board_string.replacen("{}", &field, 1);
        }

        write!(f, "{}", board_string)
    }
}

const BOARD_STRING: &str = "
 X | X | X    X | X | X    X | X | X
---|---|---  ---|---|---  ---|---|---
 X | X | X    X | X | X    X | X | X
---|---|---  ---|---|---  ---|---|---
 X | X | X    X | X | X    X | X | X

 X | X | X    X | X | X    X | X | X
---|---|---  ---|---|---  ---|---|---
 X | X | X    X | X | X    X | X | X
---|---|---  ---|---|---  ---|---|---
 X | X | X    X | X | X    X | X | X

 X | X | X    X | X | X    X | X | X
---|---|---  ---|---|---  ---|---|---
 X | X | X    X | X | X    X | X | X
---|---|---  ---|---|---  ---|---|---
 X | X | X    X | X | X    X | X | X
";

const FORMAT_ORDER: &[(u8, u8, u8, u8)] = &[
    (0, 0, 0, 0),
    (0, 0, 0, 1),
    (0, 0, 0, 2),
    (0, 1, 0, 0),
    (0, 1, 0, 1),
    (0, 1, 0, 2),
    (0, 2, 0, 0),
    (0, 2, 0, 1),
    (0, 2, 0, 2),

    (0, 0, 1, 0),
    (0, 0, 1, 1),
    (0, 0, 1, 2),
    (0, 1, 1, 0),
    (0, 1, 1, 1),
    (0, 1, 1, 2),
    (0, 2, 1, 0),
    (0, 2, 1, 1),
    (0, 2, 1, 2),

    (0, 0, 2, 0),
    (0, 0, 2, 1),
    (0, 0, 2, 2),
    (0, 1, 2, 0),
    (0, 1, 2, 1),
    (0, 1, 2, 2),
    (0, 2, 2, 0),
    (0, 2, 2, 1),
    (0, 2, 2, 2),

    (1, 0, 0, 0),
    (1, 0, 0, 1),
    (1, 0, 0, 2),
    (1, 1, 0, 0),
    (1, 1, 0, 1),
    (1, 1, 0, 2),
    (1, 2, 0, 0),
    (1, 2, 0, 1),
    (1, 2, 0, 2),

    (1, 0, 1, 0),
    (1, 0, 1, 1),
    (1, 0, 1, 2),
    (1, 1, 1, 0),
    (1, 1, 1, 1),
    (1, 1, 1, 2),
    (1, 2, 1, 0),
    (1, 2, 1, 1),
    (1, 2, 1, 2),
    
    (1, 0, 2, 0),
    (1, 0, 2, 1),
    (1, 0, 2, 2),
    (1, 1, 2, 0),
    (1, 1, 2, 1),
    (1, 1, 2, 2),
    (1, 2, 2, 0),
    (1, 2, 2, 1),
    (1, 2, 2, 2),

    (2, 0, 0, 0),
    (2, 0, 0, 1),
    (2, 0, 0, 2),
    (2, 1, 0, 0),
    (2, 1, 0, 1),
    (2, 1, 0, 2),
    (2, 2, 0, 0),
    (2, 2, 0, 1),
    (2, 2, 0, 2),

    (2, 0, 1, 0),
    (2, 0, 1, 1),
    (2, 0, 1, 2),
    (2, 1, 1, 0),
    (2, 1, 1, 1),
    (2, 1, 1, 2),
    (2, 2, 1, 0),
    (2, 2, 1, 1),
    (2, 2, 1, 2),

    (2, 0, 2, 0),
    (2, 0, 2, 1),
    (2, 0, 2, 2),
    (2, 1, 2, 0),
    (2, 1, 2, 1),
    (2, 1, 2, 2),
    (2, 2, 2, 0),
    (2, 2, 2, 1),
    (2, 2, 2, 2),
];
