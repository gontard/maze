## Context

The maze project has a workspace with three crates: maze-core (lib), maze-terminal (bin), maze-web (cdylib). After the initial build-out, a code review surfaced dead code, duplication, an unsafe block, and naming issues. The codebase is small (~900 lines of production code), making this an ideal time to clean up before adding features.

## Goals / Non-Goals

**Goals:**
- Eliminate all duplicated logic between maze-terminal and maze-web
- Remove dead code and unused imports
- Remove the unsafe block in maze-web
- Reduce BFS boilerplate via a neighbors iterator
- Improve naming clarity throughout generator code
- Cut ~100 lines of copy-pasted generator tests via parameterization

**Non-Goals:**
- Changing the flat `Vec<Vec<Tile>>` grid to a 1D `Vec<Tile>` (valuable but a larger refactor)
- Adding new game features
- Changing rendering behavior or game mechanics

## Decisions

### 1. Floor generation moves to maze-core as a function, not a struct

Add a `pub fn generate_floor(rng: &mut StdRng, start_pos: Option<(usize, usize)>) -> (Maze, f64)` in a new `maze-core/src/floor.rs`. This matches the existing signature in maze-web's `Game::generate_floor` and avoids over-abstracting.

**Alternative**: A `FloorGenerator` struct with configurable dimensions/room params. Rejected — YAGNI, a plain function is sufficient since all call sites use the same parameters.

### 2. `Maze::neighbors()` returns a small fixed-size iterator

```rust
pub fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)>
```

Returns only traversable neighbors. Used in `place_exit`, `solve`, and test helpers. Yields 0–4 items via `ArrayVec` or a filtered array.

**Alternative**: Return all in-bounds neighbors regardless of traversability. Rejected — every call site filters for traversability, so bake it in.

### 3. Keep `width`/`height` as fields but make them private with accessors

Making them fully derived (`grid.len()`, `grid[0].len()`) would require changing every access across three crates. Instead, keep them as fields but don't expose them as `pub` — add `pub fn width(&self)` / `pub fn height(&self)` methods. This is a future step; for now, leave fields public but note the inconsistency.

**Decision revised**: Actually, just keep `pub` fields for now. The sync risk is low because only generators construct `Maze` values. Removing the redundancy isn't worth the churn in this change.

### 4. Remove `FillRect` entirely

No code emits it. Both frontends handle it defensively. Remove the variant and all match arms. If needed later, it can be re-added.

### 5. Replace unsafe in maze-web with destructuring

The borrow issue is that `g.state.move_player(dir, &g.maze)` borrows `g` mutably (for `state`) and immutably (for `maze`) simultaneously through `RefCell::borrow_mut()`. Fix by destructuring:

```rust
let Game { ref maze, ref mut state, .. } = *g;
state.move_player(dir, maze);
```

### 6. Generator test parameterization via a macro

Use a macro that generates a test module per algorithm:

```rust
macro_rules! generator_tests {
    ($name:ident, $gen:expr) => { mod $name { ... } }
}
generator_tests!(recursive_backtracker, RecursiveBacktracker);
generator_tests!(kruskal, Kruskal);
generator_tests!(prim, Prim);
```

**Alternative**: A `#[test_case]` crate. Rejected — a simple macro avoids a new dependency.

### 7. Replace `init_maze` 5-tuple with a named struct

```rust
struct MazeSetup {
    width: usize,
    height: usize,
    start: (usize, usize),
    grid: Vec<Vec<Tile>>,
    rng: StdRng,
}
```

This is internal to generator.rs, not pub.

## Risks / Trade-offs

- **Removing `FillRect` is a breaking API change** → Acceptable since no external consumers exist; this is an internal project.
- **Neighbors iterator adds a method to `Maze`** → Low risk, purely additive.
- **Test macro may be harder to debug** → Mitigated by keeping the macro body simple and ensuring test names include the algorithm name.
- **Floor generation extraction couples maze-core to algorithm selection** → Acceptable; maze-core already owns all three generators.
