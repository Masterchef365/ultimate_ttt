use std::{fmt::Display, io::Write};
use ultimate_ttt::{self::*, human::human_player};

fn main() {
    let mut state = GameState::new(b"XO");
    loop {
        let mov = human_player(state);
        state = state.apply_move(mov);

        if let Some(winner) = is_superboard_won(&state.superboard) {
            println!("{} wins!", winner);
            print_game_state(&state, None);
            break;
        }
    }
}

//let an_int: u32 = prompt_parse("Please enter a u32", |s| s.parse().ok());

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_coord() {
        assert_eq!(parse_coord("A1".into()), Some(0));
        assert_eq!(parse_coord("1A".into()), Some(0));
        assert_eq!(parse_coord("3A".into()), Some(6));
        assert_eq!(parse_coord("2B".into()), Some(4));
        assert_eq!(parse_coord("3C".into()), Some(8));
        assert_eq!(parse_coord("B".into()), None);
        assert_eq!(parse_coord("1".into()), None);
        assert_eq!(parse_coord("A0".into()), None);
        assert_eq!(parse_coord("".into()), None);
    }
}