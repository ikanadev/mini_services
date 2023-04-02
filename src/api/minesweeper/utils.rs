use crate::api::minesweeper::types::{Board, CellMap};

use super::types::BoardCell;

pub fn cell_map_to_str(cell_map: CellMap) -> String {
    let keys = cell_map.keys().map(|s| s.to_string()).collect::<Vec<_>>();
    keys.join(",")
}

pub fn board_to_str_arr(board: Board) -> Vec<String> {
    board
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| match cell {
                    BoardCell::Mine => "M".to_string(),
                    BoardCell::Blank => "B".to_string(),
                    BoardCell::Number(num) => num.to_string(),
                })
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
}
