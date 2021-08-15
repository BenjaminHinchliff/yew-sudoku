use std::convert::{TryFrom, TryInto};

use sudoku::Sudoku;

use crate::{board::BoardData, digit::Digit};

pub fn generate_board() -> BoardData {
    let raw = Sudoku::generate().to_bytes();
    let mut board = BoardData::default();
    for (i, row) in raw.chunks_exact(9).enumerate() {
        board[i] = row
            .iter()
            .map(|&b| {
                if b != 0 {
                    Some(Digit::try_from(b).unwrap())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
    }
    board
}
