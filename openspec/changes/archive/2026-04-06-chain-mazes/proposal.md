## Why

The game ends after a single maze. Chaining mazes into an infinite tower creates a replayable experience where each floor is a fresh challenge. The player rides an "elevator" — staying in place while a new maze forms around them.

## What Changes

- Generator accepts a custom start position instead of hardcoding (1,1), enabling the old exit to become the new start
- Exit placement changes from "bottom-right-most path cell" to "farthest reachable point from start" via BFS, maximizing challenge per floor
- Game loop wraps in an outer level loop: on win, generate next floor; on loss/quit, end game
- Timer resets each floor with a fresh time budget based on that floor's solution path length
- Status bar adds "Floor N" label left-aligned, timer stays right-aligned
- Final message reports how many floors the player cleared

## Capabilities

### New Capabilities
- `maze-chaining`: Progression through infinite floors — generating the next maze when the current one is solved, with start/exit position handoff between floors

### Modified Capabilities
- `maze-generation`: Start position becomes parameterized; exit placement changes from bottom-right heuristic to farthest-reachable-point BFS
- `game-loop`: Outer level loop added; GameState tracks current level; Won status triggers next floor instead of game end
- `terminal-rendering`: Status bar shows floor number left-aligned alongside the existing timer
- `countdown-timer`: Timer resets per floor (no change to timer mechanics, just lifecycle)

## Impact

- `generator.rs`: `generate()` signature changes (new start param), `find_bottom_right_path` replaced
- `game.rs`: `GameState` gains `level` field, constructor signature changes
- `renderer.rs`: `render()` / `render_to_string()` gain `level` param, status bar layout changes
- `main.rs`: Outer loop added, maze regeneration logic, final message updated
- Existing tests in all four modules will need updates for new signatures
