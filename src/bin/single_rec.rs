use ultimate_ttt::single_board_solve::{game_tree, SingleBoardState, print_single};

fn main() {
    let tree = game_tree();
    dbg!(tree.len());
    /*for succ in SingleBoardState::new().successors() {
        print_single(succ.board);
        let (x, o) = tree.get(&succ).unwrap();
        println!("X/O: {}/{} {}", x, o, x + o);
    }*/
}