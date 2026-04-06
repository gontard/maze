# Development Log

## Session 2026-04-04: Maze Game MVP

### What we built

A terminal-based maze game in Rust. The player navigates a procedurally generated maze to reach the exit, timed.

### Architecture

Four modules with clear separation:

- **maze.rs** - `Tile` enum (Wall, Path, Start, Exit) and `Maze` struct with grid, dimensions, start/exit positions, helper methods (`tile_at`, `is_traversable`)
- **generator.rs** - `MazeGenerator` trait + `RecursiveBacktracker` implementation using randomized DFS. Seeded RNG for reproducibility. Carves paths on odd-indexed cells.
- **game.rs** - `GameState` with player position, timer (`Instant`), status enum. Movement with wall collision, win detection on exit, quit via q/Esc.
- **renderer.rs** - 1-char-per-cell rendering via crossterm. `#` walls, space paths, `@` player, `S` start, `E` exit. Uses `\r\n` for raw mode compatibility.

### Key decisions made during exploration

- **Rust + crossterm** for terminal interaction
- **Recursive backtracker** behind a trait for easy algorithm swapping
- **Cell grid model** (walls are full cells, not thin lines between cells)
- **1-char-per-cell rendering** after iterating through several approaches (2-char `██`, `##`, `[]`, mixed Unicode). Full-block `█` renders as double-width in Warp terminal, causing alignment issues. Single ASCII chars were the most reliable.
- **Maze size**: 41x21 (wider than tall for better terminal proportions)

### Workflow

Used OpenSpec for structured planning (proposal, design, specs, tasks), then TDD for implementation. 32 tests covering maze structure, generation solvability, seed reproducibility, rendering, movement, collision, and win condition.

### Commits

1. `89c0243` - Initial commit with OpenSpec artifacts
2. `6134ea2` - Full implementation (all modules, tests, game loop)
3. `094c60a` - Archive change, sync specs to main

### What's next

Potential features discussed during exploration: fog of war, new tile types (trees, rocks, water), different generation algorithms (Kruskal's, Prim's), dynamic sizing, color, difficulty levels.

## Session 2026-04-05: Countdown Timer

### What we built

A visible countdown timer with a time-based lose condition. The timer displays above the maze in a status bar, updates continuously, and changes color as time runs out.

### Changes

- **maze.rs** - Added `Maze::solve()` BFS method returning shortest path length (`Option<usize>`)
- **game.rs** - New `GameStatus::Lost` variant, `max_time_secs` field, `check_timeout()` method that transitions to `Lost` when time expires
- **renderer.rs** - Status bar row above the maze with right-aligned timer (`⏱ MM:SS / MM:SS`). Color urgency via crossterm: cyan (>25% remaining), dark yellow (10-25%), red (<10%). Helper functions `format_timer`, `timer_urgency`, `urgency_color`.
- **main.rs** - Poll-based game loop (`event::poll(100ms)` instead of blocking `event::read()`), computes max time from solution path length × 0.375s, handles `Lost` status with timeout message

### Key decisions made during exploration

- **Status bar above maze** (dedicated row) rather than overlaying on maze row 0
- **Max time from solution path length** (BFS shortest path × multiplier) rather than fixed time or area-based
- **Poll-based loop** for continuous timer updates even when idle, rather than a separate rendering thread
- **Time multiplier tuned to 0.375s/step** — started at 1.5s but was too generous, divided by 4 after playtesting
- **Cyan for normal timer** — white was barely visible on terminal; dark yellow replaced yellow for the same reason

### Workflow

OpenSpec explore mode for design discussion, then propose for artifacts, TDD for implementation. 49 tests total (17 new: 3 solver, 5 game timer, 9 renderer).

### Commits

1. `bda001e` - Add countdown timer with time-based lose condition
2. `537cb1b` - Archive add-timer change and sync specs

### What's next

Potential features: fog of war, difficulty levels (maze size + time scaling), pause/resume, configurable time multiplier via CLI args.

## Session 2026-04-06: Maze Chaining (Infinite Tower)

### What we built

Infinite floor progression — when the player reaches the exit, a new maze generates around them and they continue on the next floor. The game becomes an endless tower climb until timeout or quit.

### Changes

- **generator.rs** - `MazeGenerator::generate` now accepts optional start position (`Option<(usize, usize)>`). Replaced `find_bottom_right_path` with `farthest_reachable()` — BFS from start to find the cell with maximum distance, maximizing challenge per floor. Carving starts from the provided position instead of hardcoded (1,1).
- **renderer.rs** - `render()` and `render_to_string()` take a `level: usize` parameter. Status bar now shows "Floor N" left-aligned and timer right-aligned, padded to maze width. `build_status_bar` updated for the two-element layout.
- **main.rs** - Outer level loop wrapping the game loop. Tracks `level` counter and `start_pos`. On `Won`: generates new maze with `start = previous exit`, fresh `GameState`, increments level. On `Lost`/`Quit`: breaks out and shows floor count ("You cleared N floors").

### Key decisions made during exploration

- **Farthest reachable point for exit** rather than bottom-right heuristic — when start varies across floors, bottom-right might place exit adjacent to start. BFS farthest guarantees maximum traversal.
- **"Elevator" transition** — no animation, maze simply changes around the player. Mental model: square tower, player takes elevator to new floor.
- **Level in render params, not GameState** — level is display context, not game mechanics. Keeps GameState focused on movement/timer/status.
- **Timer resets per floor** with fresh time budget from each floor's solution path length.
- **Same maze dimensions** (41x21) on all floors — no difficulty progression for now.

### Workflow

OpenSpec explore mode for design discussion, then propose for all artifacts, TDD for implementation. 56 tests total (7 new: farthest exit verification, custom start position, floor label rendering).

### Commits

1. `56c2302` - Add infinite maze chaining with floor progression
2. `f866675` - Archive chain-mazes change and sync specs

### What's next

Potential features: increasing difficulty per floor (larger mazes, tighter timers), score system, fog of war, pause/resume.
