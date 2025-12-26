/*
 * Base case for solving the bottom left pair
 *
 */

use std::cmp::max;

use crate::puzzle::Move;
use crate::puzzle::Puzzle;
use crate::puzzle::ZonePos;

use super::solver_4x4::ZoneOrigin;

impl Puzzle {
    fn _solve_2x3(&mut self) {
        self._solve_bottom_left_pair();
        self.col_offset += 1;
        self.blank = (self.blank.0, self.blank.1 - 1);
        self.solve_2x2();
    }

    fn _solve_bottom_left_pair(&mut self) {
        let one = self.get_tile(self.rows - self.row_offset - 2, 0);
        let four = self.get_tile(self.rows - self.row_offset - 1, 0);
        /* Get 1 to bottom left corner */
        if self.prep_start_zone(one).1 == 1 {
            self.cycle(one, 0, 1, ZonePos::TopLeft, ZonePos::BottomLeft);
        }
        self.cycle(one, 0, 0, ZonePos::BottomRight, ZonePos::BottomLeft);
        // handling bad config - 4 @ (0,0)
        if self.get(0, 0) == four {
            // banish 4 to the last col
            self.cycle(four, 0, 0, ZonePos::BottomRight, ZonePos::TopRight);
            self.cycle(four, 0, 1, ZonePos::TopLeft, ZonePos::TopRight);
            // with 4 out of the way, get back 1 in proper position
            self.cycle(one, 0, 0, ZonePos::BottomRight, ZonePos::BottomLeft);
        }
        /* Get 4 to just beside 1 */
        self.cycle(four, 0, 1, ZonePos::TopLeft, ZonePos::BottomLeft);
        /* Complete the bottom left pair */
        self.cycle(four, 0, 0, ZonePos::BottomRight, ZonePos::BottomLeft);
    }

    pub(crate) fn prep_start_zone(&mut self, tile: u32) -> ZoneOrigin {
        let (tile_row, tile_col) = self.find_pos(tile);
        // bring to same zone
        if self.blank.0 == 0 {
            self.perform_move(Move::Down);
        }
        if self.blank.1 == 0 {
            self.perform_move(Move::Right);
        }
        while self.blank.1 > tile_col + 1 {
            self.perform_move(Move::Left);
        }
        while self.blank.1 + 1 < tile_col {
            self.perform_move(Move::Right);
        }

        while self.blank.0 > tile_row + 1 {
            self.perform_move(Move::Up);
        }
        while self.blank.0 + 1 < tile_row {
            self.perform_move(Move::Down);
        }

        // may not be adjacent to each other (important in some cases)
        if self.blank.0 < tile_row && self.blank.1 > tile_col {
            self.perform_move(Move::Down);
        } else if self.blank.0 > tile_row && self.blank.1 < tile_col {
            self.perform_move(Move::Right);
        }
        // return zone
        let max_row = max(self.blank.0, tile_row);
        let zone_row = if max_row > 0 { max_row - 1 } else { 0 };
        let max_col = max(self.blank.1, tile_col);
        let zone_col = if max_col > 0 { max_col - 1 } else { 0 };
        (zone_row, zone_col)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_puzzle_2x3() {
        let grid = vec![vec![4, 3, 2], vec![1, 0, 5]];
        let mut p = Puzzle::new(grid);
        p._solve_bottom_left_pair();
        assert_eq!(p.grid[0], 1);
        assert_eq!(p.grid[3], 4);
        p._solve_2x3();
        assert!(p.is_solved());
    }
}
