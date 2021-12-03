use std::fmt::Display;
use std::io::Write;

use crate::{GameState, successors, fmt_move, print_game_state, GamePrintGuides, Move};

pub fn human_player(state: GameState) -> Option<Move> {
    let succ = successors(&state);
    if succ.is_empty() {
        return None;
    }

    print!("Possible moves are: ");
    for &mov in &succ {
        print!("{}, ", fmt_move(mov));
    }
    println!();

    loop {
        let mut picked_superboard_idx = None;
        if state.sent_to.is_none() {
            print_game_state(&state, Some(GamePrintGuides::Superboard));
            picked_superboard_idx = Some(prompt_parse("Please pick a sub-board", parse_coord));
        }

        let superboard_idx = picked_superboard_idx.or(state.sent_to).unwrap();
        print_game_state(&state, Some(GamePrintGuides::Board(superboard_idx)));
        let board_idx = prompt_parse("Please pick a square on the sub-board", parse_coord);

        let mov = Move {
            superboard: picked_superboard_idx,
            board: board_idx,
        };
        if succ.contains(&mov) {
            break Some(mov);
        } else {
            println!("Invalid move!");
        }
    }
}

fn parse_coord(s: String) -> Option<usize> {
    let mut c = s.chars();

    let columns = ['a', 'b', 'c', 'A', 'B', 'C'];
    let rows = ['1', '2', '3'];

    let (row, col) = match c.next().zip(c.next())? {
        (a, b) if rows.contains(&a) && columns.contains(&b) => (a, b),
        (a, b) if columns.contains(&a) && rows.contains(&b) => (b, a),
        _ => return None,
    };

    let col = col.to_ascii_lowercase() as u8 - 'a' as u8;
    let row = row as u8 - '1' as u8;
    let idx = (row * 3 + col) as usize;

    Some(idx)
}

fn prompt_parse<T>(msg: impl Display + Copy, parser: fn(String) -> Option<T>) -> T {
    loop {
        if let Some(val) = parser(prompt_string(msg)) {
            break val;
        }
    }
}

fn prompt_string(msg: impl Display) -> String {
    print!("{}: ", msg);
    std::io::stdout().flush().expect("IO flush failed");

    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("IO line read failed");

    line.trim_end().to_string()
}


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