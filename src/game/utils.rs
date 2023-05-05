use super::Player;

pub fn get_winner(board: &[[Option<Player>; 3]; 3]) -> Option<Player> {
    for row in board {
        if let Some(player) = row[0] {
            if row.iter().all(|x| x == &Some(player)) {
                return Some(player);
            }
        }
    }

    for col in 0..3 {
        if let Some(player) = board[0][col] {
            if board.iter().all(|row| row[col] == Some(player)) {
                return Some(player);
            }
        }
    }

    if let Some(player) = board[0][0] {
        if (0..3).all(|i| board[i][i] == Some(player)) {
            return Some(player);
        }
    }

    if let Some(player) = board[0][2] {
        if (0..3).all(|i| board[i][2 - i] == Some(player)) {
            return Some(player);
        }
    }

    None
}
