use std::convert::{TryFrom, TryInto};

use sudoku::Sudoku;

use crate::{
    board::{Board, Cell},
    digit::Digit,
};

pub fn generate_board() -> Board {
    let raw = Sudoku::generate().to_bytes();
    raw.chunks_exact(9)
        .map(|row| {
            row.iter()
                .map(|&b| {
                    if b != 0 {
                        Cell {
                            value: Some(Digit::try_from(b).unwrap()),
                            disabled: true,
                        }
                    } else {
                        Cell {
                            value: None,
                            disabled: false,
                        }
                    }
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
