## Context

The maze game generates pure corridor mazes using a recursive backtracker. We want to add rooms as a composable modifier that works with any generation algorithm. This requires extracting exit placement from the generator so modifiers can run between generation and exit placement.

## Goals / Non-Goals

**Goals:**
- Room modifier that carves rectangular open areas into any generated maze
- Clean generation pipeline: generate -> modify -> place exit
- Room parameters: count and size range
- Deterministic with seed
- Zero regressions in existing tests

**Non-Goals:**
- Rooms-first generation (roguelike dungeon style) — that's a separate algorithm, not a modifier
- Room-specific tile types or rendering (rooms are just open Path areas)
- Guaranteed minimum room count (if placement fails due to overlap/border constraints, fewer rooms is fine)

## Decisions

### 1. Extract exit placement from generate()

`MazeGenerator::generate()` will return a maze with a `Start` tile but the exit field set to the start position (placeholder). A new `Maze::place_exit()` method runs `farthest_reachable` and places the `Exit` tile.

**Why:** Modifiers change the grid topology. If exit is placed inside `generate()`, rooms carved afterward invalidate the "farthest point" property. Extracting exit placement lets the pipeline run modifiers first.

**Migration:** `farthest_reachable` moves from `generator.rs` to `maze.rs` as a private helper for `place_exit()`. The function signature stays the same.

### 2. Room modifier operates on &mut Maze

```rust
pub fn carve_rooms(maze: &mut Maze, count: usize, min_size: usize, max_size: usize, rng: &mut impl Rng)
```

Takes a mutable reference to the maze and carves rooms in place. This is a free function (not a trait) — modifiers don't need polymorphism yet.

**Why not a trait:** There's only one modifier. A trait would be premature abstraction. If we add more modifiers later, we can extract the pattern then.

### 3. Room placement strategy

1. Pick a random cell on odd coordinates (corridor grid intersections)
2. Pick random width and height from `[min_size, max_size]`, constrained to odd numbers
3. Check that the rectangle fits within the border (1-cell margin from edges)
4. Carve all cells in the rectangle to `Path` (skip `Start` tiles)
5. Repeat for `count` attempts (not guaranteed placements)

**Why odd coordinates and sizes:** The maze grid uses odd coordinates for paths and even for walls. Aligning rooms to this grid keeps the structure clean.

**Why attempts not guarantees:** On small mazes, N rooms might not all fit. Treating `count` as "try N times" keeps the logic simple and avoids infinite retry loops.

### 4. Room carving only removes walls

The modifier sets `Wall` tiles to `Path`. It never converts `Path` to `Wall` or touches `Start`. This is the key invariant that preserves connectivity: removing walls from a perfect maze (all cells reachable, no loops) can only add connections, never remove them.

### 5. Pipeline in main.rs

```rust
let mut maze = generator.generate(41, 21, None, start_pos);
carve_rooms(&mut maze, 3, 3, 5, &mut rng);
maze.place_exit();
```

Room count and sizes can scale with floor number later, but start with fixed values.

### 6. Test strategy

**Phase 1 — Refactor (no behavior change):**
- Extract exit placement into `Maze::place_exit()`
- Move `farthest_reachable` to `maze.rs`
- Update `generate()` to not place exit
- Update `main.rs` to call `maze.place_exit()` after generate
- All existing tests updated and passing — same assertions, just calling `place_exit()` in test setup where needed

**Phase 2 — Add room modifier:**
- New tests for `carve_rooms`:
  - Only converts Wall->Path, never Path->Wall
  - Border remains all walls
  - Maze remains solvable after carving
  - All path cells still reachable
  - Deterministic with same seed
  - Room count: no more rooms than attempts
  - Works on mazes from different generators (once we have them)
- Integration test: full pipeline generate->carve_rooms->place_exit produces valid maze

## Risks / Trade-offs

- **Refactoring exit placement touches many tests**: This is the main risk. Mitigated by doing the refactor as a separate commit with no behavior change, so any test failure is a refactor bug, not a feature bug.
- **Rooms may trivialize short mazes**: On a 21x21 maze, a 5x5 room is significant. Could make paths too easy. Mitigated by keeping rooms small (3x3 to 5x5) and count low (2-3).
- **Exit distance may decrease**: Rooms create shortcuts. The exit might be closer (in steps) than in the pure corridor maze, meaning less time pressure. This is fine — `place_exit()` still picks the farthest point given the final topology.
