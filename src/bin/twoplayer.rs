use ultimate_ttt::{human::human_player, ai::two_player_game, print_game_state, is_superboard_won};

fn main() {
    let state = two_player_game(human_player, human_player);
    print_game_state(&state, None);
    if let Some(winner) = is_superboard_won(&state.superboard) {
        println!("{} wins!", winner);
    }
}