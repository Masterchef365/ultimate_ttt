use ultimate_ttt::SuperBoard;

fn main() {
    println!("{}", std::mem::size_of::<SuperBoard>());
}

// Interface idea:
//   A B C
// 1 - - -
// 2 - - -
// 3 - - -
// On the outside for starters, then on the inside. If you specify two letters, it automagically does the move