use std::collections::HashSet;

use ultimate_ttt::single_board_solve::{
    game_tree, invariant_states, print_single, SingleBoardState,
};

fn main() {
    let tree = game_tree();

    let mut queue = vec![SingleBoardState::new()];
    let mut visited: HashSet<SingleBoardState> = HashSet::new();

    let mut counter = 0;
    while let Some(state) = queue.pop() {
        counter += 1;
        if visited.contains(&state) {
            continue;
        }

        let invariants = invariant_states(state);
        let (x, o) = *invariants
            .into_iter()
            .filter_map(|s| tree.get(&s))
            .next()
            .unwrap();
        visited.extend(&invariants);
        queue.extend(state.successors());

        if x == 0 && o == 0 {
            print_single(state.board);
            println!("X/O: {}/{}", x, o);
        }
    }

    dbg!(visited.len());
    dbg!(counter);
}
