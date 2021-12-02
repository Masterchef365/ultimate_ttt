/// A Player
pub type Player = u8;

/// One square on a board
pub type Square = Option<Player>;

/// Row-major board
pub type Board = [Square; 9];

/// Row-major board of boards
pub type SuperBoard = [Board; 9];

pub const EMPTY_BOARD: Board = [None; 9];
pub const EMPTY_SUPERBOARD: SuperBoard = [EMPTY_BOARD; 9];
pub const MAX_PLAYERS: usize = 4;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Move {
    /// If the user has the choice to pick the superboard square of their next move, this is Some(index) and otherwise None
    pub superboard: Option<usize>,
    /// Which square of the sub-board to make the move to
    pub board: usize,
}

// TODO: Make all usize u8
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct GameState {
    /// The board state
    pub superboard: SuperBoard,
    /// Index of the player from GameSetup::players who will make the next move
    pub next_to_play: usize,
    /// If any, the index of the superboard square the player has been sent to.
    pub sent_to: Option<usize>, 
    /// The players in this game, ordered by who goes first.
    pub players: [Player; MAX_PLAYERS], // TODO: Try using a Vec<> and profiling...
    /// Number of players
    pub num_players: usize,
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
    pub fn new(players: &[Player]) -> Self {
        let mut players_array = [b'*'; MAX_PLAYERS];
        assert!(players.len() <= MAX_PLAYERS, "MAX_PLAYERS ({}) exceeded.", MAX_PLAYERS);
        players_array[..players.len()].copy_from_slice(players);

        Self {
            superboard: EMPTY_SUPERBOARD,
            next_to_play: 0,
            sent_to: None,
            players: players_array,
            num_players: players.len(),
        }
    }

    pub fn next_to_play(&self) -> Player {
        self.players[self.next_to_play]
    }

    pub fn apply_move(&self, mov: Move) -> Self {
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
        let player = self.players[self.next_to_play];
        board[mov.board] = Some(player);

        let mut superboard = self.superboard;
        superboard[board_idx] = board;

        // Determine if the next player is sent
        let sent_to = open_board_squares(superboard[mov.board])
            .next()
            .is_some()
            .then(|| mov.board);

        // Calculate the next player
        let next_to_play = (self.next_to_play + 1) % self.num_players;

        Self {
            superboard,
            next_to_play,
            sent_to,
            players: self.players,
            num_players: self.num_players,
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
pub fn board_shorthand(chars: [u8; 9]) -> Board {
    chars.map(|c| (c != b'-').then(|| c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_board_won() {
        assert!(is_board_won(&board_shorthand(
            *b"---------"
        ))
        .is_none());

        assert!(is_board_won(&board_shorthand(
            *b"----X----"
        ))
        .is_none());

        assert!(is_board_won(&board_shorthand(
            *b"O---X---O"
        ))
        .is_none());

        assert!(is_board_won(&board_shorthand(
*b"X-X-X---O"
))
        .is_none());

        assert!(
            is_board_won(&board_shorthand(
*b"X-X-X---X"
))==Some(b'X')
        );

        assert!(
            is_board_won(&board_shorthand(
*b"O-X-X-X-X"
))==Some(b'X')
        );

        assert!(
            is_board_won(&board_shorthand(
*b"O-X-XXO-X"
))==Some(b'X')
        );

        assert!(
            is_board_won(&board_shorthand(
*b"XXX-O-O--"
))==Some(b'X')
        );
    }

    #[test]
    fn test_is_super_board_won() {
        let superboard = [
            board_shorthand(*b"XXX-O-O--"),
            board_shorthand(*b"XXX-O-O--"),
            board_shorthand(*b"XXX-O-O--"),
            board_shorthand(*b"XOX-O-O--"),
            board_shorthand(*b"XOX-O-O--"),
            board_shorthand(*b"XOX-O-O--"),
            board_shorthand(*b"XOX-O-O--"),
            board_shorthand(*b"XOX-O-O--"),
            board_shorthand(*b"XXX-O-O--"),
        ];

        assert_eq!(is_superboard_won(&superboard), Some(b'X'));

        let superboard = [
            board_shorthand(*b"XXX-O-O--"),
            board_shorthand(*b"X-X-O-O--"),
            board_shorthand(*b"XOX-O-O--"),
            board_shorthand(*b"XXX-O-O--"),
            board_shorthand(*b"XOX-O-O--"),
            board_shorthand(*b"XXX-O-O--"),
            board_shorthand(*b"X-X-O-O--"),
            board_shorthand(*b"XXX-O-O--"),
            board_shorthand(*b"X-X-O-O--"),
        ];

        assert!(is_superboard_won(&superboard).is_none());
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GamePrintGuides {
    Superboard,
    Board(usize),
}

pub fn print_game_state(state: &GameState, guides: Option<GamePrintGuides>) {
    if let Some(winner) = is_superboard_won(&state.superboard) {
        println!("{} won.", winner);
    } else {
        println!("{} to play.", state.players[state.next_to_play]);
    }

    print_superboard(&state.superboard, guides);
}

pub fn print_superboard(superboard: &SuperBoard, guides: Option<GamePrintGuides>) {
    for (superboard_row_idx, superboard_row) in superboard.chunks_exact(3).enumerate() {
        // Headers for this superboard row, if any
        print!(" ");
        for superboard_column_idx in 0..3 {
            print!(" ");
            let superboard_idx = superboard_row_idx * 3 + superboard_column_idx;
            match guides {
                Some(GamePrintGuides::Board(board_idx)) if board_idx == superboard_idx => {
                    print!("A B C");
                }
                Some(GamePrintGuides::Superboard) if superboard_row_idx == 0 => {
                    print!("  {}  ", ['A', 'B', 'C'][superboard_column_idx]);
                }
                _ => print!("     "),
            }
            print!(" ");
        }
        println!();

        // Print board cells
        for board_row in 0..3 {
            for superboard_column_idx in 0..3 {
                let superboard_idx = superboard_row_idx * 3 + superboard_column_idx;
                match guides {
                    Some(GamePrintGuides::Board(board_idx)) if superboard_idx == board_idx => {
                        print!(" {}", board_row + 1);
                    }
                    Some(GamePrintGuides::Superboard) if board_row == 1 && superboard_column_idx == 0 => {
                        print!("{} ", superboard_row_idx + 1);
                    }
                    _ => print!("  "),
                }

                let board = superboard_row[superboard_column_idx];
                let row = &board[board_row * 3..][..3];
                let disp = |i: usize| row[i].unwrap_or(b'-');
                print!("{} {} {}", disp(0), disp(1), disp(2));
            }
            println!();
        }
    }
}

fn coord_to_chars(c: usize) -> [char; 2] {
    [
        ((c % 3) as u8 + 'A' as u8) as char,
        ((c / 3) as u8 + '1' as u8) as char,
    ]
}

pub fn fmt_move(mov: Move) -> String {
    let mut s = String::new();
    if let Some(superboard) = mov.superboard {
        s.extend(coord_to_chars(superboard));
        s.push('>');
    }
    s.extend(coord_to_chars(mov.board));
    s
}