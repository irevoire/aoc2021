use aoc::answer;
use day4::*;

fn main() {
    let mut bingo: Bingo = aoc::parser::input();

    let looser_score = 'block: loop {
        bingo.play_turn();
        while let Some(board) = bingo.win() {
            if bingo.grids.len() == 1 {
                break 'block score(board) * bingo.current_number();
            } else {
                let board = board.clone();
                bingo.grids.retain(|grid| *grid != board);
            }
        }
    };
    answer!("The looser have {}.", looser_score);
}
