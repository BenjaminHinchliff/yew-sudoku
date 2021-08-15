use std::convert::{TryFrom, TryInto};

use crate::digit::Digit;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Cell {
    pub value: Option<Digit>,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Board {
    pub data: [[Cell; 9]; 9],
}

impl From<[u8; 81]> for Board {
    fn from(board: [u8; 81]) -> Self {
        Board {
            data: board
                .chunks_exact(9)
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
                .unwrap(),
        }
    }
}

impl From<Board> for [u8; 81] {
    fn from(board: Board) -> Self {
        board
            .data
            .iter()
            .flatten()
            .map(|c| {
                if let Some(v) = c.value {
                    u8::from(v)
                } else {
                    0
                }
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
}
