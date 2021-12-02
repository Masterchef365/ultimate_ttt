use rand::{prelude::SliceRandom, thread_rng};

use crate::{print_game_state, successors, GameState, Move};

/// Return a random valid move, if any
pub fn random_move(state: GameState) -> Option<Move> {
    successors(&state).choose(&mut thread_rng()).copied()
}

/// Run a two player game, and return the resultant board when one player has no moves
pub fn two_player_game(
    mut x: impl FnMut(GameState) -> Option<Move>,
    mut o: impl FnMut(GameState) -> Option<Move>,
) -> GameState {
    let mut state = GameState::new(b"XO");
    loop {
        state = match x(state) {
            Some(mov) => state.apply_move(mov),
            None => break state,
        };

        state = match o(state) {
            Some(mov) => state.apply_move(mov),
            None => break state,
        };
    }
}

/// Use the given move function, but print the state
pub fn debug_player(
    state: GameState,
    mut policy: impl FnMut(GameState) -> Option<Move>,
) -> Option<Move> {
    print_game_state(&state, None);
    policy(state)
}