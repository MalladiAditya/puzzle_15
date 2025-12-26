/* Base case - solving the 2x2 grid */
/* setup
 * 3 numbers and 1 blank
 * if solvable, cycling either direction will eventually solve it
 *
 */

use crate::puzzle::Move;
use crate::puzzle::Puzzle;

impl Puzzle {
    pub(crate) fn is_solved_2x2(&self) -> bool {
        self.get(1, 1) == 0 && self.get(0, 0) < self.get(0, 1) && self.get(0, 1) < self.get(1, 0)
    }

    pub(crate) fn solve_2x2(&mut self) {
        while !self.is_solved_2x2() {
            self.cycle_2x2();
        }
    }

    fn cycle_2x2(&mut self) {
        let dir = match self.get_blank() {
            (0, 0) => Move::Right,
            (0, 1) => Move::Down,
            (1, 0) => Move::Up,
            (1, 1) => Move::Left,
            (_, _) => panic!("Oh no, out of bounds !!"),
        };
        self.perform_move(dir);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn permutations<T: Clone>(arr: Vec<T>) -> Vec<Vec<T>> {
        if arr.len() == 1 {
            return vec![arr];
        }

        let mut result = Vec::new();
        for i in 0..arr.len() {
            let mut rest = arr.clone();
            let elem = rest.remove(i);
            for mut perm in permutations(rest) {
                perm.insert(0, elem.clone());
                result.push(perm);
            }
        }
        result
    }

    #[test]
    fn test_solved() {
        let grid = vec![vec![1, 2], vec![3, 0]];
        let mut p = Puzzle::new(grid);
        assert!(p.is_solvable());
        assert!(p.is_solved());
        p.solve_2x2();
        assert!(p.is_solved());

        dbg!(p);
    }
    #[test]
    fn test_all() {
        let tiles = vec![0, 1, 2, 3];
        let perms = permutations(tiles);
        let mut pass = 0;
        for perm in perms {
            let grid = perm.chunks(2).map(|chunk| chunk.to_vec()).collect();
            let mut p = Puzzle::new(grid);
            if p.is_solvable() {
                p.solve_2x2();
                pass += if p.is_solved() { 1 } else { 0 };
            }
        }
        assert_eq!(pass, 12);
    }
}
