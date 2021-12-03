use std::collections::HashMap;

type StateMap = HashMap<SingleBoardState, Scores>;

use crate::{
    is_board_won, open_board_squares, print_superboard, Board, GamePrintGuides, Player,
    EMPTY_BOARD, EMPTY_SUPERBOARD,
};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SingleBoardState {
    pub board: Board,
    pub x_is_next: bool,
}

impl SingleBoardState {
    pub fn new() -> Self {
        Self {
            board: EMPTY_BOARD,
            x_is_next: true,
        }
    }

    pub fn apply(mut self, mov: usize) -> Self {
        /*if self.board[mov].is_some() {
            panic!("Illegal move");
        }*/

        self.board[mov] = Some(if self.x_is_next { b'X' } else { b'O' });
        self.x_is_next = !self.x_is_next;
        self
    }

    pub fn from_board(board: Board, x_is_next: bool) -> Self {
        Self { board, x_is_next }
    }

    pub fn successors(self) -> impl Iterator<Item = Self> {
        open_board_squares(self.board).map(move |mov| self.apply(mov))
    }

    pub fn winner(&self) -> Option<Player> {
        is_board_won(&self.board)
    }
}

pub fn print_single(board: Board) {
    let mut superboard = EMPTY_SUPERBOARD;
    superboard[0] = board;
    print_superboard(&superboard, Some(GamePrintGuides::Board(0)))
}

type Scores = (u32, u32);

pub fn game_tree() -> StateMap {
    let mut table = StateMap::new();
    recursive_step(SingleBoardState::new(), &mut table);
    table
}

fn add_scores((a, b): Scores, (c, d): Scores) -> Scores {
    (a + c, b + d)
}

fn recursive_step(
    state: SingleBoardState,
    table: &mut StateMap,
) -> Scores {
    let score = match state.winner() {
        Some(b'X') => (1, 0),
        Some(b'O') => (0, 1),
        _ => {
            let mut cumulative_score = (0, 0);
            for succ in state.successors() {
                if !invariant_states(succ).iter().any(|s| table.contains_key(s)) {
                    let score = match table.get(&succ) {
                        Some(&s) => s,
                        None => recursive_step(succ, table),
                    };
                    cumulative_score = add_scores(cumulative_score, score);
                }
            }
            cumulative_score 
        }
    };
    table.insert(state, score);
    score
}

fn invariant_states(state: SingleBoardState) -> [SingleBoardState; 8] {
    invariant_boards(state.board).map(|board| SingleBoardState::from_board(board, state.x_is_next))
}

/*
    [
        a, b, c,
        d, e, f,
        g, h, i,
    ]
*/

fn invariant_boards(board: Board) -> [Board; 8] {
    /// Vertical flip
    fn vertical_flip([a, b, c, d, e, f, g, h, i]: Board) -> Board {
        [
            g, h, i, //
            d, e, f, //
            a, b, c, //
        ]
    }

    /// Horizontal flip,
    fn horizontal_flip([a, b, c, d, e, f, g, h, i]: Board) -> Board {
        [
            c, b, a, //
            f, e, d, //
            i, h, g, //
        ] 
    }

    /// Rotate clockwise 90 degrees
    fn rotate_clockwise([a, b, c, d, e, f, g, h, i]: Board) -> Board {
        [
            g, d, a, //
            h, e, b, //
            i, f, c, //
        ]
    }

    /// Rotate counterclockwise 90 degrees
    fn rotate_counterclockwise([a, b, c, d, e, f, g, h, i]: Board) -> Board {
        [
            c, f, i, //
            b, e, h, //
            a, d, g, //
        ]
    }

    let horiz_flipped = horizontal_flip(board);
    [
        board,
        rotate_clockwise(board),
        vertical_flip(horiz_flipped),
        rotate_counterclockwise(board),
        horiz_flipped,
        rotate_clockwise(horiz_flipped),
        vertical_flip(board),
        rotate_counterclockwise(horiz_flipped),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinatorics() {
        let tree = game_tree();
        let distinct_games = tree.keys().filter(|s| open_board_squares(s.board).count() == 0).count();
        let x_wins = tree.keys().filter(|s| s.winner() == Some(b'X')).count();
        let o_wins = tree.keys().filter(|s| s.winner() == Some(b'O')).count();
    }
}