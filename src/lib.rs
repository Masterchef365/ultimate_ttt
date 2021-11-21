/// A Player
pub type Player = char;

/// One square on a board
pub type Square = Option<Player>;

/// Row-major board
pub type Board = [Square; 9];

/// Row-major board of boards
pub type SuperBoard = [Board; 9];

pub struct GameSetup {
    /// The players in this game, ordered by who goes first.
    players: Vec<Player>,
}

#[derive(Copy, Clone, Debug)]
pub struct Move {
    /// If the user has the choice to pick the superboard square of their next move, this is Some(index) and otherwise None
    superboard: Option<usize>,
    /// Which square of the sub-board to make the move to
    board: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct GameState {
    /// The board state
    superboard: SuperBoard,
    /// Index of the player from GameSetup::players who will make the next move
    next_to_play: usize,
    /// If any, the index of the superboard square the player has been sent to.
    sent_to: Option<usize>,
}

/// Return the successors of the current board state. Will return an empty vector if the game is finished.
pub fn successors(state: &GameState) -> Vec<Move> {
    if let Some(send) = state.sent_to {
        open_board_squares(state.superboard[send])
            .map(|square| Move {
                superboard: None,
                board: square,
            })
            .collect()
    } else {
        state
            .superboard
            .iter()
            .enumerate()
            .map(|(superboard_idx, board)| {
                open_board_squares(*board).map(move |square| Move {
                    superboard: Some(superboard_idx),
                    board: square,
                })
            })
            .flatten()
            .collect()
    }
}

impl GameState {
    pub fn apply_move(&self, mov: Move, setup: &GameSetup) -> Self {
        // Check if the superboard move is legal
        let board_idx = match (self.sent_to, mov.superboard) {
            (Some(i), None) | (None, Some(i)) => i,
            (Some(_), Some(_)) => panic!("Move included superboard index when not needed"),
            (None, None) => panic!("Move did not include superboard index when needed"),
        };

        // Check if the board move is legal
        let mut board = self.superboard[board_idx];
        if board[mov.board].is_some() {
            panic!("Illegal move in sub-board {}; {}", board_idx, mov.board);
        }
        
        // Make the move
        let player = setup.players[self.next_to_play];
        board[mov.board] = Some(player);

        let mut superboard = self.superboard;
        superboard[board_idx] = board;

        // Determine if the next player is sent
        let sent_to = open_board_squares(superboard[mov.board]).next().is_some().then(|| mov.board);

        // Calculate the next player
        let next_to_play = (self.next_to_play + 1) % setup.players.len();

        Self {
            superboard,
            next_to_play,
            sent_to,
        }
    }
}

/// Returns an iterator over the squares yet to be populated in this board
pub fn open_board_squares(board: Board) -> impl Iterator<Item = usize> {
    board
        .into_iter()
        .enumerate()
        .filter_map(|(idx, square)| (square == Square::None).then(|| idx))
}

pub fn row_is_won(row: [Square; 3]) -> Option<Player> {
    row[0].and_then(|player| (row[1] == row[0] && row[2] == row[0]).then(|| player))
}

pub fn is_board_won(board: &Board) -> Option<Player> {
    for row in board.chunks_exact(3) {
        let ret = row_is_won([row[0], row[1], row[2]]);
        if ret.is_some() {
            return ret;
        }
    }

    for col_idx in 0..3 {
        let row = [
            board[3 * 0 + col_idx],
            board[3 * 1 + col_idx],
            board[3 * 2 + col_idx],
        ];
        let ret = row_is_won(row);
        if ret.is_some() {
            return ret;
        }
    }

    let row = [board[3 * 0 + 0], board[3 * 1 + 1], board[3 * 2 + 2]];

    let ret = row_is_won(row);
    if ret.is_some() {
        return ret;
    }

    let row = [board[3 * 0 + 2], board[3 * 1 + 1], board[3 * 2 + 0]];

    let ret = row_is_won(row);
    if ret.is_some() {
        return ret;
    }

    None
}

pub fn is_superboard_won(superboard: &SuperBoard) -> Option<Player> {
    is_board_won(&superboard.map(|b| is_board_won(&b)))
}

/// Parse an array of characters into a board
pub fn board_shorthand(chars: [char; 9]) -> Board {
    chars.map(|c| (c != '-').then(|| c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_board_won() {
        assert!(is_board_won(&board_shorthand([
            '-', '-', '-', '-', '-', '-', '-', '-', '-',
        ]))
        .is_none());

        assert!(is_board_won(&board_shorthand([
            '-', '-', '-', '-', 'X', '-', '-', '-', '-',
        ]))
        .is_none());

        assert!(is_board_won(&board_shorthand([
            'O', '-', '-', '-', 'X', '-', '-', '-', 'O',
        ]))
        .is_none());

        assert!(is_board_won(&board_shorthand([
            'X', '-', 'X', '-', 'X', '-', '-', '-', 'O',
        ]))
        .is_none());

        assert!(
            is_board_won(&board_shorthand([
                'X', '-', 'X', '-', 'X', '-', '-', '-', 'X',
            ])) == Some('X')
        );

        assert!(
            is_board_won(&board_shorthand([
                'O', '-', 'X', '-', 'X', '-', 'X', '-', 'X',
            ])) == Some('X')
        );

        assert!(
            is_board_won(&board_shorthand([
                'O', '-', 'X', '-', 'X', 'X', 'O', '-', 'X',
            ])) == Some('X')
        );

        assert!(
            is_board_won(&board_shorthand([
                'X', 'X', 'X', '-', 'O', '-', 'O', '-', '-',
            ])) == Some('X')
        );
    }

    #[test]
    fn test_is_super_board_won() {
        let superboard = [
            board_shorthand(['X', 'X', 'X', '-', 'O', '-', 'O', '-', '-']),
            board_shorthand(['X', 'X', 'X', '-', 'O', '-', 'O', '-', '-']),
            board_shorthand(['X', 'X', 'X', '-', 'O', '-', 'O', '-', '-']),
            board_shorthand(['X', 'O', 'X', '-', 'O', '-', 'O', '-', '-']),
            board_shorthand(['X', 'O', 'X', '-', 'O', '-', 'O', '-', '-']),
            board_shorthand(['X', 'O', 'X', '-', 'O', '-', 'O', '-', '-']),
            board_shorthand(['X', 'O', 'X', '-', 'O', '-', 'O', '-', '-']),
            board_shorthand(['X', 'O', 'X', '-', 'O', '-', 'O', '-', '-']),
            board_shorthand(['X', 'X', 'X', '-', 'O', '-', 'O', '-', '-']),
        ];

        assert_eq!(is_superboard_won(&superboard), Some('X'));

        let superboard = [
            board_shorthand(['X', 'X', 'X', '-', 'O', '-', 'O', '-', '-']),
            board_shorthand(['X', '-', 'X', '-', 'O', '-', 'O', '-', '-']),
            board_shorthand(['X', 'O', 'X', '-', 'O', '-', 'O', '-', '-']),
            board_shorthand(['X', 'X', 'X', '-', 'O', '-', 'O', '-', '-']),
            board_shorthand(['X', 'O', 'X', '-', 'O', '-', 'O', '-', '-']),
            board_shorthand(['X', 'X', 'X', '-', 'O', '-', 'O', '-', '-']),
            board_shorthand(['X', '-', 'X', '-', 'O', '-', 'O', '-', '-']),
            board_shorthand(['X', 'X', 'X', '-', 'O', '-', 'O', '-', '-']),
            board_shorthand(['X', '-', 'X', '-', 'O', '-', 'O', '-', '-']),
        ];

        assert!(is_superboard_won(&superboard).is_none());
    }
}