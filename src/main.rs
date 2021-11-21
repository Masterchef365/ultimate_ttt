use std::{fmt::Display, io::Write};
use ultimate_ttt::*;

fn main() {
    let an_int: u32 = prompt_valid("Please enter a u32", |s| s.parse().ok());
    dbg!(an_int);
    /*
    let mut superboard = [[None; 9]; 9];
    let mut a = 0;
    for board in &mut superboard {
        for cell in board {
            *cell = char::from_digit(a as u32 % 36, 36);
            a += 1;
        }
    }

    loop {
        let sep = || {
            std::thread::sleep(std::time::Duration::from_millis(600));
            println!("\x1B[2J\x1B[1;1H");
            //println!("_____________________");
        };

        for i in 0..9 {
            print_superboard(&superboard, Some(GamePrintGuides::Board(i)));
            sep();
        }

        print_superboard(&superboard, None);
        sep();

        print_superboard(&superboard, Some(GamePrintGuides::Superboard));
        sep();
    }
    */
}

fn prompt_valid<T>(msg: impl Display + Copy, validation: fn(String) -> Option<T>) -> T {
    loop {
        if let Some(val) = validation(prompt_string(msg)) {
            break val;
        }
    }
}

fn prompt_string(msg: impl Display) -> String {
    print!("{}: ", msg);
    std::io::stdout()
        .flush()
        .expect("IO flush failed");

    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("IO line read failed");

    line.trim_end().to_string()
}