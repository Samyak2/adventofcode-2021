use aoc::read_string;

use lib::Board;

mod lib;

fn main() {
    let contents = read_string("src/day04/input.in");

    let mut splits = contents.split("\n\n");
    let nums = splits
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u16>().unwrap());

    let mut boards: Vec<Board> = splits.map(|s| s.parse().unwrap()).collect();

    for num in nums {
        boards.iter_mut().for_each(|board| board.mark(num));

        let win = boards
            .iter()
            .find(|board| board.score.any_col_set() || board.score.any_row_set());

        if let Some(win_board) = win {
            let sum = win_board.sum_unmarked();
            println!("{}", sum * num as usize);
            break;
        }
    }
}
