## Context

The maze game currently blocks on `event::read()` — the loop only advances on key presses. There is no visible timer during play; elapsed time is only shown after winning. We need continuous rendering for a live countdown and a time-based lose condition.

## Goals / Non-Goals

**Goals:**
- Show a countdown timer that updates in real-time, even when the player is idle
- Derive max time from the maze's shortest solution path length (× 1.5s per step)
- End the game when time runs out
- Provide visual urgency via color changes as time dwindles

**Non-Goals:**
- Pause/resume functionality
- Configurable time multiplier via CLI args (hardcoded 1.5s for now)
- Animated or flashing timer effects beyond color changes

## Decisions

### Poll-based game loop instead of blocking read
Switch from `event::read()` to `event::poll(Duration::from_millis(100))` + `event::read()`. The 100ms poll interval gives smooth timer updates (~10 fps) without burning CPU.

**Alternative considered**: Spawning a separate thread for timer rendering. Rejected — adds threading complexity for no real benefit when poll works cleanly with crossterm.

### BFS solver on Maze struct
Add `Maze::solve() -> Option<usize>` that returns the shortest path length via BFS. This lives on `Maze` rather than in the generator because it's a property of the maze itself, and the generator tests already use similar BFS logic.

**Alternative considered**: Having the generator return the path length. Rejected — the recursive backtracker doesn't naturally produce the shortest path (it produces a spanning tree). A separate BFS is cleaner and more accurate.

### Time formula: path_length × 1.5 seconds
The BFS shortest path gives the minimum number of steps. Players explore dead ends and think, so 1.5s per step provides reasonable headroom. For a typical 41×21 maze with ~100-step solution, this gives ~150s (2.5 min).

### Status bar as a separate row above the maze
Render one line above the maze grid containing the right-aligned timer. The renderer shifts the maze down by 1 row (`MoveTo(0, 1)` instead of `MoveTo(0, 0)`). Timer text is right-padded to `maze.width` display characters.

**Alternative considered**: Overlaying timer to the right of row 0. Rejected — user preferred a dedicated status bar for cleanliness.

### Color urgency via crossterm SetForegroundColor
- White: > 25% time remaining
- Yellow: 10–25% remaining
- Red: < 10% remaining

Applied only to the timer text on the status bar. Uses `crossterm::style::SetForegroundColor` and `ResetColor`.

### GameStatus::Lost variant
Added alongside existing `Playing`, `Won`, `Quit`. The `check_timeout()` method on `GameState` transitions from `Playing` to `Lost` when elapsed exceeds max. `move_player` already no-ops on non-`Playing` states, so `Lost` is handled for free.

## Risks / Trade-offs

- [100ms poll granularity] → Timer display may be up to 100ms stale. Acceptable for a game timer showing MM:SS format.
- [Hardcoded 1.5s multiplier] → May be too easy or too hard depending on maze complexity. Can be tuned later or exposed as a CLI arg.
- [BFS runs once at startup] → Negligible cost even for large mazes. Not a performance concern.
