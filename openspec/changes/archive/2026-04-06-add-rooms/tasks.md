## 1. Refactor: extract exit placement from generator

- [x] 1.1 Add `place_exit(&mut self)` method to `Maze` that runs BFS farthest-reachable and places the `Exit` tile
- [x] 1.2 Move `farthest_reachable` from `generator.rs` to `maze.rs` as a private helper
- [x] 1.3 Update `RecursiveBacktracker::generate()` to return maze without exit (no `Exit` tile in grid, `exit` field set to `start`)
- [x] 1.4 Update `main.rs` to call `maze.place_exit()` after `generate()`
- [x] 1.5 Update all existing generator tests to call `place_exit()` where they assert exit properties
- [x] 1.6 Update all existing maze tests if needed (no changes needed — hand-built test mazes unaffected)
- [x] 1.7 Run full test suite — all tests must pass with no behavior change

## 2. Add room carving modifier

- [x] 2.1 Write tests for `carve_rooms`: wall-to-path only, border integrity, solvability, all cells reachable, determinism, room count bound
- [x] 2.2 Implement `carve_rooms(maze: &mut Maze, count: usize, min_size: usize, max_size: usize, rng: &mut impl Rng)`
- [x] 2.3 Run room modifier tests — all must pass

## 3. Integrate into game pipeline

- [x] 3.1 Update `main.rs` pipeline: `generate()` -> `carve_rooms()` -> `place_exit()`
- [x] 3.2 Run full test suite and play-test
