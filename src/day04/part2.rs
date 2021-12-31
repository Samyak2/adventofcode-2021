use aoc::read_string;

mod lib;

use lib::Board;

fn main() {
    let contents = read_string("src/day04/input.in");

    let mut splits = contents.split("\n\n");
    let nums = splits
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u16>().unwrap());

    let mut boards: Vec<Board> = splits.map(|s| s.parse().unwrap()).collect();
    let mut last_win_board: Option<Board> = None;
    let mut last_win_num = 0;

    for num in nums {
        boards.iter_mut().for_each(|board| board.mark(num));

        let win_boards: Vec<usize> = boards.iter().enumerate().filter_map(|(i, board)| {
            if board.score.any_col_set() || board.score.any_row_set() {
                Some(i)
            } else {
                None
            }
        }).rev().collect();

        win_boards.iter().for_each(|&win_board_ind| {
            last_win_board = Some(boards.swap_remove(win_board_ind));
            last_win_num = num;
        });
    }

    let last_win_board = last_win_board.expect("No winning boards!");

    let ans = last_win_board.sum_unmarked() * last_win_num as usize;

    println!("{}", ans);
}
