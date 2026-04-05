## Why

Build a terminal-based maze game in Rust as a fun, self-contained project. The player navigates a procedurally generated maze to reach the exit, racing against the clock. The architecture should support future extensibility (new generation algorithms, tile types, fog of war) without requiring rewrites.

## What Changes

- New Rust project with `crossterm` for terminal interaction
- Procedural maze generation using recursive backtracker, behind a trait for easy algorithm swapping
- Grid-based maze with a `Tile` enum (Wall, Path, Start, Exit — extensible for future types like Tree, Rock, Water)
- 2-char-wide cell rendering for visually square proportions
- Player movement via arrow keys or WASD with wall collision blocking
- Timer tracking elapsed time from start, displayed on victory
- Game loop: input → update → render

## Capabilities

### New Capabilities
- `maze-generation`: Procedural maze generation behind a `MazeGenerator` trait, with recursive backtracker as the first implementation
- `game-loop`: Core game loop handling input, player movement, collision detection, win condition, and timer
- `terminal-rendering`: 2-char-wide tile-based rendering using crossterm, mapping Tile enum variants to Unicode characters

### Modified Capabilities

_(none — greenfield project)_

## Impact

- New Rust binary crate with `crossterm` and `rand` as dependencies
- Terminal raw mode required during gameplay (restored on exit/panic)
- Fixed maze size for MVP (no terminal size detection needed yet)
