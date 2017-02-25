// create a new board
fn create_board(capacity: usize, first_value: i8) -> Vec<Vec<i8>> {
    let mut board = vec!(vec!(0i8; capacity); capacity);
    board[0][0] = first_value;
    board
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
    let board = create_board(2, 1);
    let mut other_board = create_board(2, 0);

    print_board(&board);
    print_board(&other_board);
    copy_board(&board, other_board.as_mut_slice());
    print_board(&board);
    print_board(&other_board);
}
