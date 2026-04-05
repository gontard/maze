## Why

The maze game lacks time pressure — players can take as long as they want, which reduces tension and replayability. Adding a visible countdown timer creates urgency and a lose condition, making the game more engaging.

## What Changes

- Add a status bar rendered above the maze showing a countdown timer (elapsed / max), right-aligned to the maze width
- Compute max allowed time from the maze's solution path length (BFS shortest path × 1.5 seconds per step)
- Add a `Lost` game status — when time expires, the game ends with a loss message
- Switch the game loop from blocking `event::read()` to `event::poll()` so the timer updates continuously even without player input
- Color the timer based on urgency: white (>25% remaining), yellow (10–25%), red (<10%)

## Capabilities

### New Capabilities
- `countdown-timer`: Visible countdown timer with time-based lose condition, urgency coloring, and path-length-based time scaling
- `maze-solving`: BFS solver on `Maze` that returns the shortest solution path length

### Modified Capabilities
- `game-loop`: Add poll-based ticking and timeout checking (new `Lost` status, `check_timeout()`)
- `terminal-rendering`: Render a status bar row above the maze with the colored timer

## Impact

- `maze.rs`: New `solve()` method
- `game.rs`: New `Lost` variant, `max_time_secs` field, `check_timeout()` method
- `renderer.rs`: Status bar rendering with ANSI color support (crossterm `SetForegroundColor`)
- `main.rs`: Compute max time, poll-based loop, handle `Lost` end state
