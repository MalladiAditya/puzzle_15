/* Solve the original 4x4 puzzle
 * Refine some of the methods in the base cases
 */
use crate::puzzle::*;
use std::cmp::min;

pub(crate) type ZoneOrigin = (usize, usize);

pub(crate) enum Axis {
    H,
    V,
}

impl Puzzle {
    pub fn solve(&mut self) -> bool {
        // check global solvability (only once)
        if self.row_offset == 0 && self.col_offset == 0 && !self.is_solvable() {
            return false;
        }
        let (rows, cols) = (self.rows - self.row_offset, self.cols - self.col_offset);
        if rows == 2 && cols == 2 {
            self.solve_2x2();
            return self.is_solved();
        }
        // Place top left corner
        self.top_left_corner();
        // dbg!(&self.grid);
        // Fillers for top row
        for i in 1..(self.cols - self.col_offset - 2) {
            self.top_row_filler(i);
        }
        // top right pair
        self.top_right_pair();
        // dbg!(&self.grid);
        // Fillers for left column
        for i in 1..(self.rows - self.row_offset - 2) {
            self.left_col_filler(i);
        }
        // Bottom left pair
        self.bottom_left_pair();
        // dbg!(&self.grid);

        // Solve smaller grid
        if rows > 2 {
            self.row_offset += 1;
            self.blank = (self.blank.0 - 1, self.blank.1);
        }
        if cols > 2 {
            self.col_offset += 1;
            self.blank = (self.blank.0, self.blank.1 - 1);
        }
        self.solve()
    }

    fn move_tile_to_zone(&mut self, tile: u32, target_zone: ZoneOrigin, order: (Axis, Axis)) {
        self.conduit_step(tile, target_zone, order.0, 0);
        // dbg!(&self.grid);
        self.conduit_step(tile, target_zone, order.1, 1);
        // dbg!(&self.grid);
    }

    fn conduit_step(&mut self, tile: u32, target_zone: ZoneOrigin, axis: Axis, step: usize) {
        let (target_row, target_col) = target_zone;
        let (mut tile_row, mut tile_col) = self.prep_start_zone(tile);
        let (cur_row, cur_col) = self.find_pos(tile);
        let zone_tile = self.get(tile_row, tile_col);
        let target_tile = self.get_tile(tile_row, tile_col);
        if tile > target_tile && zone_tile == target_tile {
            if cur_row == self.blank.0 {
                tile_row = min(tile_row + 1, self.rows - self.row_offset - 2);
            } else if cur_col == self.blank.1 {
                tile_col = min(tile_col + 1, self.cols - self.col_offset - 2);
            }
        }
        match axis {
            Axis::H => {
                tile_row = if step == 1 { target_row } else { tile_row };
                loop {
                    // println!("({}, {})", tile_row, tile_col);
                    if tile_col == target_col {
                        break;
                    } else if tile_col < target_col {
                        self.cycle(
                            tile,
                            tile_row,
                            tile_col,
                            ZonePos::BottomRight,
                            ZonePos::TopRight,
                        );
                        tile_col += 1;
                    } else {
                        self.cycle(
                            tile,
                            tile_row,
                            tile_col,
                            ZonePos::TopLeft,
                            ZonePos::BottomLeft,
                        );
                        tile_col -= 1;
                    }
                }
            }
            Axis::V => {
                tile_col = if step == 1 { target_col } else { tile_col };
                loop {
                    // println!("({}, {})", tile_row, tile_col);
                    if tile_row == target_row {
                        break;
                    } else if tile_row < target_row {
                        self.cycle(
                            tile,
                            tile_row,
                            tile_col,
                            ZonePos::BottomRight,
                            ZonePos::BottomLeft,
                        );
                        tile_row += 1;
                    } else {
                        self.cycle(
                            tile,
                            tile_row,
                            tile_col,
                            ZonePos::TopLeft,
                            ZonePos::TopRight,
                        );
                        tile_row -= 1;
                    }
                }
            }
        }
    }

    fn top_left_corner(&mut self) {
        // println!("start left corner");
        let (rows, cols) = (self.rows - self.row_offset, self.cols - self.col_offset);
        if rows <= 2 || cols <= 2 {
            return;
        }
        let tile = self.get_tile(0, 0);
        if self.get(0, 0) == tile {
            return;
        }
        self.move_tile_to_zone(tile, (0, 0), (Axis::H, Axis::V));
        self.cycle(tile, 0, 0, ZonePos::BottomLeft, ZonePos::TopLeft);
    }

    fn top_row_filler(&mut self, filler_pos: usize) {
        // println!("start top fillers");
        let tile = self.get_tile(0, filler_pos);
        if self.get(0, filler_pos) == tile {
            return;
        }
        // assert_eq!(self.grid[0], 1);
        // dbg!(&self.grid);
        self.move_tile_to_zone(tile, (0, filler_pos), (Axis::H, Axis::V));
        self.cycle(tile, 0, filler_pos, ZonePos::BottomLeft, ZonePos::TopLeft);
    }

    fn top_right_pair(&mut self) {
        // println!("get top right");
        // dbg!(&self.grid);
        if self.rows - self.row_offset <= 2 {
            return;
        }
        let col_1 = self.cols - self.col_offset - 2;
        let col_2 = self.cols - self.col_offset - 1;
        let tile_1 = self.get_tile(0, col_1);
        let tile_2 = self.get_tile(0, col_2);
        if self.get(0, col_1) == tile_1 && self.get(0, col_2) == tile_2 {
            return;
        }
        // move tile_1 to top right
        self.move_tile_to_zone(tile_1, (0, col_1), (Axis::H, Axis::V));
        self.cycle(tile_1, 0, col_1, ZonePos::BottomRight, ZonePos::TopRight);
        // dbg!(&self.grid);
        // resolve pathological scenario - tile_2 @ (0, col_1)
        if self.get(0, col_1) == tile_2 {
            // banish tile_2 out of current zone
            self.cycle(tile_2, 0, col_1, ZonePos::BottomLeft, ZonePos::BottomRight);
            self.cycle(tile_2, 1, col_1, ZonePos::TopRight, ZonePos::BottomRight);
            // with tile_2 out of the way, get back tile_1 in proper position
            self.cycle(tile_1, 0, col_1, ZonePos::BottomRight, ZonePos::TopRight);
        }
        // dbg!(&self.grid);
        // move tile_2 under tile_1
        self.move_tile_to_zone(tile_2, (1, col_1), (Axis::H, Axis::V));
        self.cycle(tile_2, 1, col_1, ZonePos::TopLeft, ZonePos::TopRight);
        // complete the pair
        self.cycle(tile_2, 0, col_1, ZonePos::BottomRight, ZonePos::TopRight);
    }

    fn left_col_filler(&mut self, filler_pos: usize) {
        // println!("start left fillers");
        let tile = self.get_tile(filler_pos, 0);
        if self.get(filler_pos, 0) == tile {
            return;
        }
        self.move_tile_to_zone(tile, (filler_pos, 0), (Axis::V, Axis::H));
        self.cycle(tile, filler_pos, 0, ZonePos::TopRight, ZonePos::TopLeft);
    }

    fn bottom_left_pair(&mut self) {
        // println!("start bottom left");
        // dbg!(&self.grid);
        if self.cols - self.col_offset <= 2 {
            return;
        }
        let row_1 = self.rows - self.row_offset - 2;
        let row_2 = self.rows - self.row_offset - 1;
        let tile_1 = self.get_tile(row_1, 0);
        let tile_2 = self.get_tile(row_2, 0);
        if self.get(row_1, 0) == tile_1 && self.get(row_2, 0) == tile_2 {
            return;
        }
        // move tile_1 to bottom left
        self.move_tile_to_zone(tile_1, (row_1, 0), (Axis::V, Axis::H));
        // dbg!(&self.grid);
        self.cycle(tile_1, row_1, 0, ZonePos::BottomRight, ZonePos::BottomLeft);
        // resolve pathological scenario - tile_2 @ (row_1, 0)
        if self.get(row_1, 0) == tile_2 {
            // banish tile_2 out of current zone
            self.cycle(tile_2, row_1, 0, ZonePos::BottomRight, ZonePos::TopRight);
            self.cycle(tile_2, row_1, 1, ZonePos::TopLeft, ZonePos::TopRight);
            // with tile_2 out of the way, get back tile_1 in proper position
            self.cycle(tile_1, row_1, 0, ZonePos::BottomRight, ZonePos::BottomLeft);
        }
        // dbg!(&self.grid);
        // move tile_2 under tile_1
        self.move_tile_to_zone(tile_2, (row_1, 1), (Axis::V, Axis::H));
        // dbg!(&self.grid);
        self.cycle(tile_2, row_1, 1, ZonePos::TopLeft, ZonePos::BottomLeft);
        // complete the pair
        self.cycle(tile_2, row_1, 0, ZonePos::BottomRight, ZonePos::BottomLeft);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_4x4() {
        let grid = vec![
            vec![1, 2, 0, 11],
            vec![12, 7, 14, 9],
            vec![3, 13, 4, 6],
            vec![15, 10, 8, 5],
        ];
        let mut p = Puzzle::new(grid);
        assert!(p.is_solvable());
        assert!(p.solve());
        dbg!(&p.grid);
    }

    #[test]
    fn test_3x2() {
        let mut p = Puzzle::new(vec![vec![2, 1], vec![3, 5], vec![4, 0]]);
        assert!(p.solve());
    }

    #[test]
    fn test_2x3() {
        let mut p = Puzzle::new(vec![vec![4, 3, 2], vec![1, 0, 5]]);
        assert!(p.solve());
    }

    #[test]
    fn test_3x3() {
        let mut p = Puzzle::new(vec![vec![1, 8, 0], vec![2, 3, 7], vec![5, 4, 6]]);
        assert!(p.solve());
    }
}
