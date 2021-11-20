/// A Player
pub type Player = char;

/// One square on a board
pub type Square = Option<Player>;

/// Row-major board
pub type Board = [Square; 9];

/// Row-major board of boards
pub type SuperBoard = [Board; 9];

pub fn row_is_won(row: [Square; 3]) -> Option<Player> {
    row[0].and_then(|player| (row[1] == row[0] && row[2] == row[0]).then(|| player))
}

pub fn is_board_won(board: &Board) -> Option<Player> {
    for row in board.chunks_exact(3) {
        let ret = row_is_won([row[0], row[1], row[2]]);
        if ret.is_some() {
            return ret;
        }
    }

    for col_idx in 0..3 {
        let row = [
            board[3*0 + col_idx],
            board[3*1 + col_idx],
            board[3*2 + col_idx],
        ];
        let ret = row_is_won(row);
        if ret.is_some() {
            return ret;
        }
    }

    let row = [
        board[3*0 + 0],
        board[3*1 + 1],
        board[3*2 + 2],
    ];

    let ret = row_is_won(row);
    if ret.is_some() {
        return ret;
    }

    let row = [
        board[3*0 + 2],
        board[3*1 + 1],
        board[3*2 + 0],
    ];

    let ret = row_is_won(row);
    if ret.is_some() {
        return ret;
    }

    None
}

pub fn is_superboard_won(superboard: &SuperBoard) -> Option<Player> {
    is_board_won(&superboard.map(|b| is_board_won(&b)))
}

/// Parse an array of characters into a board
pub fn board_shorthand(chars: [char; 9]) -> Board {
    chars.map(|c| (c != '-').then(|| c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_board_won() {
        assert!(is_board_won(&board_shorthand([
            '-', '-', '-', 
            '-', '-', '-', 
            '-', '-', '-', 
        ])).is_none());

        assert!(is_board_won(&board_shorthand([
            '-', '-', '-', 
            '-', 'X', '-', 
            '-', '-', '-', 
        ])).is_none());

        assert!(is_board_won(&board_shorthand([
            'O', '-', '-', 
            '-', 'X', '-', 
            '-', '-', 'O', 
        ])).is_none());

        assert!(is_board_won(&board_shorthand([
            'X', '-', 'X', 
            '-', 'X', '-', 
            '-', '-', 'O', 
        ])).is_none());

        assert!(is_board_won(&board_shorthand([
            'X', '-', 'X', 
            '-', 'X', '-', 
            '-', '-', 'X', 
        ])) == Some('X'));

        assert!(is_board_won(&board_shorthand([
            'O', '-', 'X', 
            '-', 'X', '-', 
            'X', '-', 'X', 
        ])) == Some('X'));

        assert!(is_board_won(&board_shorthand([
            'O', '-', 'X', 
            '-', 'X', 'X', 
            'O', '-', 'X', 
        ])) == Some('X'));

        assert!(is_board_won(&board_shorthand([
            'X', 'X', 'X', 
            '-', 'O', '-', 
            'O', '-', '-', 
        ])) == Some('X'));
    }

    #[test]
    fn test_is_super_board_won() {
        let superboard = [
            board_shorthand([
                'X', 'X', 'X', 
                '-', 'O', '-', 
                'O', '-', '-', 
            ]),
            board_shorthand([
                'X', 'X', 'X', 
                '-', 'O', '-', 
                'O', '-', '-', 
            ]),
            board_shorthand([
                'X', 'X', 'X', 
                '-', 'O', '-', 
                'O', '-', '-', 
            ]),
            board_shorthand([
                'X', 'O', 'X', 
                '-', 'O', '-', 
                'O', '-', '-', 
            ]),
            board_shorthand([
                'X', 'O', 'X', 
                '-', 'O', '-', 
                'O', '-', '-', 
            ]),
            board_shorthand([
                'X', 'O', 'X', 
                '-', 'O', '-', 
                'O', '-', '-', 
            ]),
            board_shorthand([
                'X', 'O', 'X', 
                '-', 'O', '-', 
                'O', '-', '-', 
            ]),
            board_shorthand([
                'X', 'O', 'X', 
                '-', 'O', '-', 
                'O', '-', '-', 
            ]),
            board_shorthand([
                'X', 'X', 'X', 
                '-', 'O', '-', 
                'O', '-', '-', 
            ]),
        ];

        assert_eq!(is_superboard_won(&superboard), Some('X'));

        let superboard = [
            board_shorthand([
                'X', 'X', 'X', 
                '-', 'O', '-', 
                'O', '-', '-', 
            ]),
            board_shorthand([
                'X', '-', 'X', 
                '-', 'O', '-', 
                'O', '-', '-', 
            ]),
            board_shorthand([
                'X', 'O', 'X', 
                '-', 'O', '-', 
                'O', '-', '-', 
            ]),
            board_shorthand([
                'X', 'X', 'X', 
                '-', 'O', '-', 
                'O', '-', '-', 
            ]),
            board_shorthand([
                'X', 'O', 'X', 
                '-', 'O', '-', 
                'O', '-', '-', 
            ]),
            board_shorthand([
                'X', 'X', 'X', 
                '-', 'O', '-', 
                'O', '-', '-', 
            ]),
            board_shorthand([
                'X', '-', 'X', 
                '-', 'O', '-', 
                'O', '-', '-', 
            ]),
            board_shorthand([
                'X', 'X', 'X', 
                '-', 'O', '-', 
                'O', '-', '-', 
            ]),
            board_shorthand([
                'X', '-', 'X', 
                '-', 'O', '-', 
                'O', '-', '-', 
            ]),
        ];

        assert!(is_superboard_won(&superboard).is_none());
    }
}