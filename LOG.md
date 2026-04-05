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
