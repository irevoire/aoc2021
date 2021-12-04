use aoc::answer;
use day4::*;

fn main() {
    let mut bingo: Bingo = aoc::parser::input();

    let winner_score = loop {
        bingo.play_turn();
        if let Some(board) = bingo.win() {
            break score(board) * bingo.current_number();
        }
    };
    answer!("The winner have {}.", winner_score);
}
