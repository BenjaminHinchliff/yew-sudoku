use sudoku::Sudoku;

use crate::board::Board;

pub fn generate_board() -> Board {
    Sudoku::generate().to_bytes().into()
}
