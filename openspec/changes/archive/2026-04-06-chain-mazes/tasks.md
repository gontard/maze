## 1. Generator: custom start and farthest exit

- [x] 1.1 Add `farthest_reachable(grid, start) -> (usize, usize)` function using BFS, replacing `find_bottom_right_path`
- [x] 1.2 Change `MazeGenerator::generate` trait and `RecursiveBacktracker` impl to accept optional start position parameter
- [x] 1.3 Carve maze from provided start position instead of hardcoded (1,1)
- [x] 1.4 Update generator tests: custom start position, farthest exit placement, backward compatibility with None

## 2. Renderer: floor number in status bar

- [x] 2.1 Add `level: usize` parameter to `render()` and `render_to_string()`
- [x] 2.2 Update status bar layout: "Floor N" left-aligned, timer right-aligned, padded to maze width
- [x] 2.3 Update `build_status_bar` test helper and all renderer tests for new signature and layout

## 3. Main: level loop and chaining

- [x] 3.1 Wrap game loop in outer level loop with level counter and player position tracking
- [x] 3.2 On Won: generate new maze with start = previous exit, create fresh GameState, increment level
- [x] 3.3 On Lost/Quit: break out of level loop
- [x] 3.4 Update final messages to include floor count ("You cleared N floors")
- [x] 3.5 Pass level to renderer calls
