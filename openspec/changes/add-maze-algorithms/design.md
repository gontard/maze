# Design: Add Maze Algorithms

## Trait change

Add `name()` to `MazeGenerator`:

```rust
pub trait MazeGenerator {
    fn name(&self) -> &'static str;
    fn generate(&self, width: usize, height: usize, seed: Option<u64>, start: Option<(usize, usize)>) -> Maze;
}
```

## Kruskal's algorithm

Struct: `Kruskal` (unit struct, in `generator.rs`)

Algorithm:
1. Start with all walls. Every odd-aligned cell is its own set.
2. Collect all internal walls between adjacent cells.
3. Shuffle the wall list using the seeded rng.
4. For each wall: if cells on either side are in different sets, carve the wall and union the sets.

Requires a union-find data structure — private helper within `generator.rs` (~25 lines, rank + path compression).

Produces many short dead ends, branchy texture.

## Prim's algorithm

Struct: `Prim` (unit struct, in `generator.rs`)

Algorithm:
1. Start with all walls. Mark starting cell as visited, carve it.
2. Add walls adjacent to the starting cell to a frontier vec.
3. While frontier is non-empty:
   - Pick a random wall from the frontier (swap-remove for O(1)).
   - If it separates a visited cell from an unvisited cell, carve through and mark the new cell visited, add its new walls to the frontier.
   - Otherwise discard.

No extra data structures beyond a `Vec` frontier and a visited grid.

Produces shorter corridors with a radial growth pattern from the start.

## Algorithm selection (main.rs)

```rust
// One rng per floor, derived before generation
let algo_index = rng.random_range(0..3u32);
let (mut maze, algo_name) = match algo_index {
    0 => (RecursiveBacktracker.generate(...), RecursiveBacktracker.name()),
    1 => (Kruskal.generate(...), Kruskal.name()),
    _ => (Prim.generate(...), Prim.name()),
};
```

No trait objects, no dynamic dispatch. The trait provides the contract; the match provides the selection.

## Renderer change

`renderer::render()` gains an `algo_name: &str` parameter, displayed on the HUD alongside floor number and timer.

## Grid model

Both algorithms use the same odd-aligned cell/wall grid as `RecursiveBacktracker`:
- Cells at odd (x, y) coordinates
- Walls at even coordinates or between cells
- Outer border stays all walls

This means `carve_rooms()`, `place_exit()`, and `solve()` work unchanged.

## File changes

| File | Change |
|------|--------|
| `src/generator.rs` | Add `name()` to trait, implement `Kruskal` + `Prim`, add union-find helper |
| `src/main.rs` | Random algorithm selection per floor, pass name to renderer |
| `src/renderer.rs` | Display algorithm name on HUD |
