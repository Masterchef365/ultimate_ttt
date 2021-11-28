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
