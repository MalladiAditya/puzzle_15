/*
 * Base case for solving the top right pair
 *
 */

use std::cmp::max;

use crate::puzzle::*;

use super::solver_4x4::ZoneOrigin;

impl ZonePos {
    pub fn to_coords(&self) -> (usize, usize) {
        match self {
            ZonePos::BottomLeft => (1, 0),
            ZonePos::BottomRight => (1, 1),
            ZonePos::TopLeft => (0, 0),
            ZonePos::TopRight => (0, 1),
        }
    }
}

impl Puzzle {
    fn _solve_3x2(&mut self) {
        self._solve_top_right_pair();
        self.row_offset += 1;
        self.blank = (self.blank.0 - 1, self.blank.1);
        self.solve_2x2();
    }

    fn _solve_top_right_pair(&mut self) {
        let one = self.get_tile(0, self.cols - self.col_offset - 2);
        let two = self.get_tile(0, self.cols - self.col_offset - 1);
        /* Get 1 to top right corner */
        let mut start_zone = self._prep_start_zone_vertical(one).0;
        while start_zone > 0 {
            self.cycle(one, start_zone, 0, ZonePos::TopLeft, ZonePos::TopRight);
            start_zone -= 1;
        }
        self.cycle(one, 0, 0, ZonePos::BottomRight, ZonePos::TopRight);
        // handling bad config - 2 @ TopLeft
        if self.get(0, 0) == two {
            // banish 2 to the last row
            self.cycle(two, 0, 0, ZonePos::BottomLeft, ZonePos::BottomRight);
            self.cycle(two, 1, 0, ZonePos::TopRight, ZonePos::BottomRight);
            // with 2 out of the way, get back 1 in proper position
            self.cycle(one, 0, 0, ZonePos::BottomRight, ZonePos::TopRight);
        }
        /* Get 2 to just under 1 */
        start_zone = self._prep_start_zone_vertical(two).0;
        while start_zone > 1 {
            self.cycle(two, start_zone, 0, ZonePos::TopLeft, ZonePos::TopRight);
            start_zone -= 1;
        }
        self.cycle(two, 1, 0, ZonePos::TopLeft, ZonePos::TopRight);
        /* Complete the top right pair */
        self.cycle(two, 0, 0, ZonePos::BottomRight, ZonePos::TopRight);
    }

    pub(crate) fn cycle(
        &mut self,
        tile: u32,
        zone_row: usize,
        zone_col: usize,
        blank_target: ZonePos,
        tile_target: ZonePos,
    ) {
        let b_coords = {
            let (r, c) = blank_target.to_coords();
            (zone_row + r, zone_col + c)
        };
        let t_coords = {
            let (r, c) = tile_target.to_coords();
            (zone_row + r, zone_col + c)
        };
        while self.get_blank() != b_coords || self.find_pos(tile) != t_coords {
            let dir = match (self.get_blank().0 - zone_row, self.get_blank().1 - zone_col) {
                (0, 0) => Move::Right,
                (0, 1) => Move::Down,
                (1, 0) => Move::Up,
                (1, 1) => Move::Left,
                (_, _) => panic!("Oh no, out of bounds !!"),
            };
            self.perform_move(dir);
        }
    }

    pub(crate) fn _prep_start_zone_vertical(&mut self, tile: u32) -> ZoneOrigin {
        let (tile_row, tile_col) = self.find_pos(tile);
        while self.get_blank().0 > tile_row + 1 {
            self.perform_move(Move::Up);
        }
        while self.get_blank().0 + 1 < tile_row {
            self.perform_move(Move::Down);
        }
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
    fn test_puzzle_3x2() {
        let grid = vec![vec![2, 1], vec![3, 5], vec![4, 0]];
        let mut p = Puzzle::new(grid);
        p._solve_top_right_pair();
        assert_eq!(p.grid[0..2], vec![1, 2]);
        p._solve_3x2();
        assert!(p.is_solved());
    }
}
