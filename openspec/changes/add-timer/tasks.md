## 1. Maze Solver

- [x] 1.1 Add `Maze::solve() -> Option<usize>` BFS method returning shortest path length
- [x] 1.2 Add tests: solvable maze, adjacent start/exit, unsolvable maze

## 2. Game State Timer

- [x] 2.1 Add `GameStatus::Lost` variant
- [x] 2.2 Add `max_time_secs: f64` field to `GameState` and update constructor
- [x] 2.3 Add `check_timeout(&mut self)` method that transitions to `Lost`
- [x] 2.4 Add tests: timeout triggers Lost, no movement after Lost, no timeout before max time

## 3. Renderer Status Bar

- [x] 3.1 Add `elapsed` and `max_time` parameters to `render()` and `render_to_string()`
- [x] 3.2 Render status bar row with right-aligned timer in `⏱ MM:SS / MM:SS` format
- [x] 3.3 Apply color urgency (white >25%, yellow 10-25%, red <10%) using crossterm colors
- [x] 3.4 Shift maze grid to start at row 1 instead of row 0
- [x] 3.5 Add tests: timer formatting, right-alignment, status bar present in output

## 4. Game Loop Integration

- [x] 4.1 Compute max time from `maze.solve()` path length × 1.5 in `main.rs`
- [x] 4.2 Switch from `event::read()` to `event::poll(100ms)` + `event::read()`
- [x] 4.3 Call `check_timeout()` each tick and pass elapsed/max_time to renderer
- [x] 4.4 Handle `GameStatus::Lost` at game exit with timeout message
