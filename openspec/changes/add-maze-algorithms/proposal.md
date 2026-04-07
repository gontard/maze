# Add Maze Algorithms (Kruskal's & Prim's)

## Problem
Every floor uses the same Recursive Backtracker algorithm, producing long winding corridors with a consistent feel. Floors lack variety.

## Solution
Add two new maze generation algorithms — Kruskal's and Prim's — and randomly select one of the three per floor. Display the algorithm name on the HUD so the player knows which type of maze they're navigating.

## Approach
- Implement `Kruskal` and `Prim` as new structs with `impl MazeGenerator`
- Add `fn name(&self) -> &'static str` to the `MazeGenerator` trait
- In the floor loop, randomly pick an algorithm (uniform over 3) using a match — no trait objects, no enum wrapper (Option C from exploration)
- Pass the algorithm name through to the renderer
- Room carving, exit placement, and game logic are untouched

## Non-goals
- Weighted/progressive algorithm selection (uniform random for now)
- Difficulty balancing across algorithms
- Any changes to room carving or exit placement
