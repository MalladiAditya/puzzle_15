15 Puzzle in Rust

Using a mechanical approach where the only move is a counter-clockwise cycle in a 2x2 block. This approach resembles what a normal person would do by hand.
At a high level, the steps are as follows:
- Fix the top left corner (tile 1)
- Fill up the rest of the top row excluding the last pair (tile 2)
- Handle the remaining two tiles as a pair
- Repeat above 2 steps with analogous modifications to fill up the leftmost column
- Now we have a smaller grid left to solve, so we have our recursive step here
- The 2x2 block is the base case, which is solved by simply cycling in one direction
