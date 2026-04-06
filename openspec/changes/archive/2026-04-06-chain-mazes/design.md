## Context

The maze game currently generates a single maze, plays it, and exits. The player always starts at (1,1) and the exit is placed at the bottom-right-most path cell. The game has a countdown timer derived from the solution path length. We want to chain mazes into an infinite tower where completing one floor immediately presents the next.

## Goals / Non-Goals

**Goals:**
- Infinite floor progression: each completed maze leads to the next
- Seamless "elevator" transition: player stays in place, maze changes around them
- Floor counter visible in the status bar
- Each floor gets a fresh timer
- Exit placement maximizes distance from start on every floor

**Non-Goals:**
- Increasing difficulty (larger mazes, tighter timers) — same parameters for now
- Score system or persistent high scores
- Transition animations between floors
- Pause functionality

## Decisions

### 1. Generator accepts custom start position

The `MazeGenerator::generate` signature gains an optional `start: Option<(usize, usize)>` parameter. When `None`, defaults to `(1,1)` for backward compatibility. The recursive backtracker carves from this position instead of hardcoded `(1,1)`.

**Why over changing the trait entirely:** The `Option` approach keeps the first-floor call simple (`None`) and makes the intent clear. No need for a builder pattern — this is the only parameter that varies.

### 2. Exit placement via BFS farthest point

Replace `find_bottom_right_path` with `farthest_reachable(grid, start) -> (usize, usize)`. This runs BFS from start, tracking distance, and returns the cell with the maximum distance.

**Why over keeping bottom-right:** When the start is at the old exit (which could be anywhere), a bottom-right heuristic might place the new exit right next to the start. BFS farthest point guarantees maximum traversal regardless of start position.

**Implementation:** Reuse the BFS pattern already in `Maze::solve()`. The farthest point is simply the last cell dequeued in a BFS traversal (or track max distance explicitly). Since the generator builds the grid before creating the `Maze` struct, this function operates on the raw `Vec<Vec<Tile>>` grid.

### 3. Level loop in main.rs, not in GameState

The outer level loop lives in `main()`. `GameState` gains a `level` field for display purposes but has no chaining logic. On `Won`, main generates a new maze and creates a fresh `GameState`.

**Why not in GameState:** GameState is a pure state machine (Playing/Won/Lost/Quit). Chaining is orchestration logic that involves the generator, renderer, and state together. Keeping it in main preserves separation of concerns.

### 4. Level tracked in GameState for rendering

`GameState` stores `level: usize` so the renderer can access it through the game state. The alternative (passing level as a separate param to render) would work but `GameState` is the natural home for "current game context."

**Reconsidered:** Actually, level is display context, not game state. Passing it as a separate parameter to the renderer (like `elapsed` and `max_time`) keeps GameState focused on game mechanics. The renderer already takes several params — one more is fine.

**Decision:** Pass `level` as a parameter to `render()`, do NOT add it to `GameState`.

### 5. Status bar layout

```
Floor 7                    ⏱ 00:15 / 00:30
```

"Floor N" left-aligned, timer right-aligned, padded with spaces to fill maze display width. The floor label uses no special coloring — timer urgency colors remain as-is.

### 6. Final message

On game end:
- Lost: "Time's up on floor N! You cleared N-1 floors."
- Quit: "Quit on floor N. You cleared N-1 floors."

When `N == 1` (failed/quit the first floor): "You cleared 0 floors" — honest and motivating.

## Risks / Trade-offs

- **Maze generation time**: Generating a new 41x21 maze is fast (<1ms), so no perceivable delay on floor transition. No risk here.
- **BFS for farthest point adds computation**: One extra BFS per floor. The maze is small (41x21 = 861 cells), so this is negligible.
- **Player position validity**: The new maze MUST have the start cell at the player's position. Since the generator carves from the start, this cell is always a path. No risk of the player landing in a wall.
- **Status bar width**: "Floor N" + timer must fit in maze display width. With a 41-wide maze, "Floor 999" (9 chars) + timer "⏱ 00:00 / 00:00" (17 chars) = 26 chars. Maze width is 41. Plenty of room even for very high floor numbers.
