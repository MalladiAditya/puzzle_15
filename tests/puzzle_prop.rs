use proptest::prelude::*;
use puzzle_15::puzzle::*;

fn make_grid(tiles: Vec<u32>, rows: usize, cols: usize) -> Vec<Vec<u32>> {
    assert!(tiles.len() == rows * cols, "Tile count should match");
    tiles.chunks(cols).map(|row| row.to_vec()).collect()
}

fn puzzle_stategy(rows: usize, cols: usize) -> impl Strategy<Value = Vec<u32>> {
    proptest::strategy::Just((0..(rows as u32 * cols as u32)).collect::<Vec<_>>()).prop_shuffle()
    // .prop_filter("must be solvable", move |tiles| {
    //     let p = Puzzle::new(make_grid(tiles.clone(), rows, cols));
    //     p.is_solvable()
    // })
}

proptest! {
  #![proptest_config(ProptestConfig::with_cases(1000))]
  #[test]
  fn solves_4x4(puzzle in puzzle_stategy(4, 4)) {
    let mut p = Puzzle::new(make_grid(puzzle.clone(), 4, 4));
    if p.is_solvable() {
      prop_assert!(p.solve());
    }
  }
  #[test]
  fn solves_5x5(puzzle in puzzle_stategy(5, 5)) {
    let mut p = Puzzle::new(make_grid(puzzle.clone(), 5, 5));
    if p.is_solvable() {
      prop_assert!(p.solve());
    }
  }
  #[test]
  fn solves_3x3(puzzle in puzzle_stategy(3, 3)) {
    let mut p = Puzzle::new(make_grid(puzzle.clone(), 3, 3));
    // dbg!(&p);
    if p.is_solvable() {
      prop_assert!(p.solve()); }
  }
  #[test]
  fn solves_2x2(puzzle in puzzle_stategy(2, 2)) {
    let mut p = Puzzle::new(make_grid(puzzle.clone(), 2, 2));
    if p.is_solvable() {
      prop_assert!(p.solve());
    }
  }
}
