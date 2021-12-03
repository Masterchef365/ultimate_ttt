use ultimate_ttt::{ai::two_player_game, human::human_player, is_superboard_won, print_game_state};

fn main() {
    let state = two_player_game(human_player, human_player);
    print_game_state(&state, None);
    if let Some(winner) = is_superboard_won(&state.superboard) {
        println!("{} wins!", winner);
    }
}
