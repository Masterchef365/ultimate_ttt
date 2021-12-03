use ultimate_ttt::{
    ai::{random_move, two_player_game},
    human::human_player,
};

fn main() {
    two_player_game(
        //|state| debug_player(state, random_move),
        random_move,
        //|state| debug_player(state, random_move),
        human_player,
    );
}
