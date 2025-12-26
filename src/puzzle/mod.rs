mod solver;
mod solver_2x2;
mod solver_2x3;
mod solver_3x2;
mod solver_4x4;
mod util;

/* Representing the moves */
#[derive(Debug, Clone, Copy)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

/* Representing the Puzzle State - 4x4 board */
#[derive(Debug)]
pub struct Puzzle {
    grid: Vec<u32>,        // 0 means blank piece
    blank: (usize, usize), // (row,col) of blank piece
    rows: usize,
    cols: usize,
    row_offset: usize,
    col_offset: usize,
}

pub enum ZonePos {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}
