use crate::digit::Digit;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Cell {
    pub value: Option<Digit>,
    pub disabled: bool,
}

pub type Board = [[Cell; 9]; 9];
