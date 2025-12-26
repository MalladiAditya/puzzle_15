/*
 * util.rs
 * Constructors and accessor methods
 * IO related work
 * other utilities
 */

use crate::puzzle::Move;
use crate::puzzle::Puzzle;

impl Puzzle {
    // Initial state constructor
    pub fn new(in_grid: Vec<Vec<u32>>) -> Self {
        let rows = in_grid.len();
        let cols = in_grid[0].len();
        let mut blank = (rows, cols);
        let mut grid = vec![0; rows * cols];
        for r in 0..rows {
            for c in 0..cols {
                grid[r * cols + c] = in_grid[r][c];
                if in_grid[r][c] == 0 {
                    blank = (r, c);
                }
            }
        }
        Self {
            grid,
            blank,
            rows,
            cols,
            row_offset: 0,
            col_offset: 0,
        }
    }

    fn index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    pub(crate) fn get_tile(&self, row: usize, col: usize) -> u32 {
        (((row + self.row_offset) * self.cols + col + self.col_offset + 1)
            % (self.rows * self.cols)) as u32
    }

    pub(crate) fn get(&self, row: usize, col: usize) -> u32 {
        self.grid[self.index(row + self.row_offset, col + self.col_offset)]
    }

    pub(crate) fn set(&mut self, row: usize, col: usize, val: u32) {
        let idx = self.index(row + self.row_offset, col + self.col_offset);
        self.grid[idx] = val;
    }
    // get row of blank piece
    pub(crate) fn get_blank(&self) -> (usize, usize) {
        self.blank
    }

    pub(crate) fn find_pos(&self, tile: u32) -> (usize, usize) {
        for r in 0..(self.rows - self.row_offset) {
            for c in 0..(self.cols - self.col_offset) {
                if self.get(r, c) == tile {
                    return (r, c);
                }
            }
        }
        (0, 0)
    }

    // get inversions
    fn count_inversions(&self) -> usize {
        // O(n^2) brute force should work well as grids are generally small
        // also called only once to check solvability
        let mut inversions = 0;
        let num_elems = self.rows * self.cols;
        for i in 0..num_elems {
            let i_val = self.grid[i];
            for j in i + 1..num_elems {
                let j_val = self.grid[j];
                inversions += if j_val > 0 && i_val > j_val { 1 } else { 0 };
            }
        }
        inversions
    }

    // Check Solvability
    pub fn is_solvable(&self) -> bool {
        if self.cols % 2 == 0 {
            (self.count_inversions() + self.blank.0) % 2 == (self.rows - 1) % 2
        } else {
            self.count_inversions() % 2 == 0
        }
    }

    // Move
    pub fn perform_move(&mut self, dir: Move) {
        // TODO: check for illegal moves
        let (cur_blank_row, cur_blank_col) = self.blank;
        match dir {
            Move::Down => {
                self.blank = (cur_blank_row + 1, cur_blank_col);
            }
            Move::Left => {
                self.blank = (cur_blank_row, cur_blank_col - 1);
            }
            Move::Right => {
                self.blank = (cur_blank_row, cur_blank_col + 1);
            }
            Move::Up => {
                self.blank = (cur_blank_row - 1, cur_blank_col);
            }
        }
        self.set(
            cur_blank_row,
            cur_blank_col,
            self.get(self.blank.0, self.blank.1),
        );
        self.set(self.blank.0, self.blank.1, 0);
    }

    // Check if puzzle is solved
    pub fn is_solved(&self) -> bool {
        self.count_inversions() == 0 && self.grid.last() == Some(&0)
    }
}
