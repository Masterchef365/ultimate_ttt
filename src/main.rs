use std::{collections::VecDeque, fmt::Display, io::Write};
use ultimate_ttt::*;

fn main() {
    let mut queue = vec![];
    queue.push(GameState::new(vec!['X', 'O', 'R']));
    let mut i = 1;
    while let Some(state) = queue.pop() {
        queue.extend(
            successors(&state)
                .into_iter()
                .map(|mov| state.apply_move(mov)),
        );
        if let Some('O' | 'R') = is_superboard_won(&state.superboard) {
            print_game_state(&state, None);
            i += 1;
        }
        if i % 50000 == 0 {
            println!("{}", i);
        }
    }
}

fn human_player(state: &GameState) -> Move {
    todo!()
    /*
    let board_idx = None;
    if state.sent_to.is_none() {
        print_game_state(state, Some(GamePrintGuides::Superboard));
        prompt_parse("Please pick", |s| s)
        board_idx = Some();
    }

    Move {
        superboard: board_idx,
        board,
    }
    */
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