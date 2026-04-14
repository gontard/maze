## 1. Dead Code Removal

- [x] 1.1 Remove `DrawCommand::FillRect` variant from `maze-core/src/render.rs`
- [x] 1.2 Remove `FillRect` match arms from `maze-terminal/src/renderer.rs` and `maze-web/src/renderer.rs`
- [x] 1.3 ~~Remove `use rand::prelude::IndexedRandom` from `maze-core/src/generator.rs`~~ SKIPPED: import is actually used by `choose()`

## 2. Maze Neighbors Iterator

- [x] 2.1 Add `Maze::neighbors(x, y) -> impl Iterator<Item = (usize, usize)>` method to `maze-core/src/maze.rs`
- [x] 2.2 Write tests for `neighbors()`: interior cell, wall-adjacent, corner, out-of-bounds
- [x] 2.3 Refactor `Maze::place_exit()` to use `self.neighbors()`
- [x] 2.4 Refactor `Maze::solve()` to use `self.neighbors()`
- [x] 2.5 Refactor test BFS helpers (`bfs_all_reachable`, `bfs_distances`) to use `maze.neighbors()`

## 3. Generator Naming Cleanup

- [x] 3.1 Replace `init_maze` 5-tuple return with a `MazeSetup` struct in `generator.rs`
- [x] 3.2 Rename cryptic variables in `carve_rooms`: `rw`→`room_w`, `rh`→`room_h`, `rx`→`room_x`, `ry`→`room_y`
- [x] 3.3 Rename cryptic grid-coordinate variables in Kruskal/Prim: `gax`→`grid_ax`, `gay`→`grid_ay`, etc.

## 4. Parameterize Generator Tests

- [x] 4.1 Create `generator_tests!` macro with shared property tests (dimensions, reachability, solvability, borders, determinism, custom start)
- [x] 4.2 Instantiate macro for RecursiveBacktracker, Kruskal, and Prim
- [x] 4.3 Remove the duplicated per-algorithm test functions
- [x] 4.4 ~~Remove duplicate `bfs_all_reachable` helper~~ Kept minimal copies in both test modules — extracting to shared test utility not worth the complexity for 10 lines

## 5. Floor Generation Extraction

- [x] 5.1 Create `maze-core/src/floor.rs` with `pub fn generate_floor(rng: &mut StdRng, start_pos: Option<(usize, usize)>) -> (Maze, f64)`
- [x] 5.2 Write tests for `generate_floor`: returns solvable maze, correct time budget, deterministic with same seed
- [x] 5.3 Update `maze-terminal/src/main.rs` to call `maze_core::floor::generate_floor`
- [x] 5.4 Update `maze-web/src/lib.rs` to call `maze_core::floor::generate_floor`, remove `Game::generate_floor`

## 6. Remove Unsafe Block

- [x] 6.1 Replace unsafe raw-pointer borrow in `maze-web/src/lib.rs` keyboard handler with struct destructuring

## 7. Verify

- [x] 7.1 Run `cargo fmt --all && RUSTFLAGS="-D warnings" cargo build --workspace && cargo test --workspace` — all clean (99 tests pass)
