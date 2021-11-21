use ultimate_ttt::*;

fn main() {
    let mut superboard = [[None; 9]; 9];
    let mut a = 0;
    for board in &mut superboard {
        for cell in board {
            *cell = char::from_digit(a as u32 % 36, 36);
            a += 1;
        }
    }

    let sep = || println!("_____________________");

    for i in 0..9 {
        print_superboard(&superboard, Some(GamePrintGuides::Board(i)));
        sep();
    }

    print_superboard(&superboard, None);
    sep();

    print_superboard(&superboard, Some(GamePrintGuides::Superboard));
    sep();
}

// Interface idea:
//   A B C
// 1 - - -
// 2 - - -
// 3 - - -
// On the outside for starters, then on the inside. If you specify two letters, it automagically does the move