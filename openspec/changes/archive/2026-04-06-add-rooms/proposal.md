## Why

Every maze is pure corridors — narrow 1-tile paths with walls between them. Adding rooms (rectangular open areas) breaks up the claustrophobia, creates landmarks for navigation, and adds a new dimension to gameplay under time pressure. Rooms give the player "I've been here before" recognition and force multi-exit decisions.

## What Changes

- A room modifier carves rectangular open areas into an already-generated maze, running as a post-processing step after any algorithm
- Exit placement moves out of `generate()` into a separate post-processing step, so modifiers can run between generation and exit placement without invalidating distances
- The generation pipeline becomes: algorithm generates corridors -> room modifier carves rooms -> exit placed at farthest point from start
- Room carving only removes walls (never adds them), so connectivity is preserved by construction

## Capabilities

### New Capabilities
- `room-modifier`: Post-processing step that carves rectangular rooms into a generated maze grid, parameterized by room count and size range

### Modified Capabilities
- `maze-generation`: Exit placement extracted from `generate()` into a standalone `place_exit()` function. `generate()` returns a maze with `Start` but no `Exit`. Exit is placed after all modifiers run.

## Impact

- `generator.rs`: `generate()` no longer places exit. `farthest_reachable` and exit placement extracted into public functions (or moved to `maze.rs`).
- `maze.rs`: Gains `place_exit(&mut self)` method and room carving logic (or a new `modifier.rs` module).
- `main.rs`: Pipeline changes from `generator.generate()` to `generate() -> carve_rooms() -> place_exit()`.
- Existing generator tests need updates: tests asserting exit properties move to integration-level tests that run the full pipeline.
- New tests for room modifier: border integrity, no path-to-wall conversion, determinism, solvability preserved.
