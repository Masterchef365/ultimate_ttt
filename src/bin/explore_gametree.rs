use std::collections::HashMap;
use ultimate_ttt::{open_board_squares, Board, EMPTY_BOARD, SuperBoard, GameState, print_superboard, EMPTY_SUPERBOARD, GamePrintGuides};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct SingleBoardState {
    board: Board,
    x_is_next: bool,
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

        self.board[mov] = Some(if self.x_is_next { 'X' } else { 'O' });
        self.x_is_next = !self.x_is_next;
        self
    }

    pub fn from_board(board: Board, x_is_next: bool) -> Self {
        Self {
            board,
            x_is_next
        }
        
    }
}

fn display_single_board(board: Board) {
    let mut superboard = EMPTY_SUPERBOARD;
    superboard[0] = board;
    print_superboard(&superboard, Some(GamePrintGuides::Board(0)))
}

fn main() {
    let mut tree = HashMap::new();
    let mut queue = vec![GameState::new(vec!['X', 'O'])];
    while let Some(state) = queue.pop() {
        print_superboard(&state.superboard, Some(GamePrintGuides::Superboard));
        println!("--------------------------------------------------");
        if !tree.contains_key(&state) {
            let successor_states: Vec<SingleBoardState> = open_board_squares(state.board)
                .map(|mov| state.apply(mov))
                .collect();
            queue.extend(successor_states.iter().copied());
            tree.insert(state, successor_states);
        }
    }
}