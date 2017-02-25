// create a new board
fn create_board(dimension: usize) -> Vec<Vec<i8>> {
    vec!(vec!(0i8; dimension); dimension)
}

fn copy_board(source_board: &[Vec<i8>], target_board: &mut [Vec<i8>]) {
    assert_eq!(source_board.len(), target_board.len());
    for i in 0..source_board.len() {
        copy_row(&source_board[i], target_board[i].as_mut_slice());
    }
}

fn copy_row(source_row: &[i8], target_row: &mut [i8]) {
    assert_eq!(source_row.len(), target_row.len());
    for i in 0..source_row.len() {
        target_row[i] = source_row[i];
    }
}

// print the board
fn print_board(board: &[Vec<i8>]) {
    println!("Board:");
    for row in board {
        for column in row.iter() {
            print!("{:?}", column);
        }
        println!("");
    }
}

fn main() {
    let mut board = create_board(2);
    let mut other_board = create_board(2);

    // set board at position (0, 0) to 1
    board[0][0] = 1;

    print_board(&board);
    print_board(&other_board);

    // copy the change over to the other board
    copy_board(&board, other_board.as_mut_slice());
    print_board(&board);
    print_board(&other_board);
}

#[cfg(test)]
mod tests {
    use super::create_board;
    use super::copy_board;

    #[test]
    fn create_board_test() {
        let dimension: usize = 2;
        let board = create_board(dimension);
        assert_eq!(dimension, board.len());
        assert_eq!(dimension, board[0].len());
    }

    #[test]
    fn copy_board_test() {
        let i: usize = 0;
        let j: usize = 0;

        let delta_value: i8 = 1;

        let mut board = create_board(1);
        board[i][j] = delta_value;
        let mut other_board = create_board(1);

        assert_eq!(delta_value, board.as_slice()[i][j]);
        assert_eq!(0, other_board.as_slice()[i][j]);

        copy_board(&board, other_board.as_mut_slice());

        assert_eq!(delta_value, board.as_slice()[i][j]);
        assert_eq!(delta_value, other_board.as_slice()[i][j]);
    }
}
