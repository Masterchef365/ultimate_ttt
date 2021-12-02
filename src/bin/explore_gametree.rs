use std::{collections::HashMap, time::Instant};
use ultimate_ttt::{GameState, successors, is_superboard_won};


fn main() {
    let mut tree = HashMap::new();
    let mut queue = vec![GameState::new(b"XO")];
    let mut i = 0;
    let mut complete_games = 0;
    let mut o_wins = 0;
    let mut x_wins = 0;


    let start = Instant::now();
    while let Some(state) = queue.pop() {
        //print_superboard(&state.superboard, Some(GamePrintGuides::Superboard));
        //println!("--------------------------------------------------");
        if !tree.contains_key(&state) {
            let successor_states: Vec<GameState> = successors(&state)
                .into_iter()
                .map(|mov| state.apply_move(mov))
                .collect();
            queue.extend(successor_states.iter().copied());
            tree.insert(state, successor_states);
        }

        if is_superboard_won(&state.superboard).is_some() {
            complete_games += 1;
        }

        match is_superboard_won(&state.superboard) {
            Some(b'X') => x_wins += 1,
            Some(b'O') => o_wins += 1,
            _ => (),
        }

        i += 1;
        if i % 10_000 == 0 {
            let elapsed = start.elapsed().as_secs_f32();
            let rate = i as f32 / elapsed;
            println!("{:2} / sec, idx: {}, tree: {}, complete: {}, O/X: {}/{}", rate, i, tree.len(), complete_games, o_wins, x_wins);
        }
    }
}