## Context

Greenfield Rust project — no existing code. The game runs entirely in the terminal using `crossterm` for raw input and screen control. The architecture must support swapping maze generation algorithms and adding new tile types without structural changes.

## Goals / Non-Goals

**Goals:**
- Playable terminal maze game with timer-based win condition
- Clean separation between generation, game logic, and rendering
- Algorithm-swappable maze generation via trait
- Extensible tile system for future content (trees, rocks, water)

**Non-Goals:**
- Dynamic maze sizing / terminal size detection
- Fog of war or partial visibility
- Multiple levels or progression
- Sound, color themes, or configuration files
- Networking or multiplayer

## Decisions

### D1: Module structure

Four modules with clear responsibilities:

| Module | Responsibility |
|--------|---------------|
| `maze` | `Tile` enum, `Maze` struct (grid + start/exit positions) |
| `generator` | `MazeGenerator` trait + `RecursiveBacktracker` implementation |
| `game` | `GameState` (player position, timer, status), movement logic, win detection |
| `renderer` | Tile-to-character mapping, screen drawing via crossterm |

**Why**: Each concern is isolated. Adding a new algorithm means adding a struct that implements `MazeGenerator`. Adding a new tile means extending the `Tile` enum and the renderer's mapping.

### D2: Maze representation — cell grid, not wall grid

The maze is a 2D grid where each cell is a `Tile`. Walls are cells, paths are cells. This is simpler than a "thin wall" model where walls live between cells.

A typical generated maze on a grid of width W, height H:
- Odd-indexed rows/columns are potential path cells
- Even-indexed rows/columns are wall cells
- The generator carves paths by converting wall cells to path cells

**Why over thin-wall model**: Simpler data structure, simpler rendering (1:1 cell-to-display mapping), easier collision detection (just check `grid[y][x]`).

**Trade-off**: Uses more memory (walls take full cells) and mazes are slightly coarser. Acceptable for terminal-sized mazes.

### D3: Rendering — 2-char-wide cells

Each tile renders as 2 characters wide to compensate for terminal characters being taller than wide. This makes cells appear roughly square.

Mapping (MVP):
- `Wall` → `██`
- `Path` → `  `
- `Start` → `S `
- `Exit` → `E `
- Player overlay → `P `

The renderer owns this mapping. Game logic never deals with character widths.

### D4: Input handling — crossterm in raw mode

`crossterm` provides:
- Raw mode (keypresses without Enter)
- `KeyEvent` reading for arrows and WASD
- Cursor positioning for efficient redraws
- Cross-platform support

On game start, enable raw mode. On exit (win or quit), restore terminal state. Use a `Drop` guard or explicit cleanup to handle panics.

### D5: Timer — `std::time::Instant`

Start an `Instant` when the game begins. On reaching the exit, compute elapsed duration. No external crate needed.

### D6: Random number generation

Use `rand` crate for the recursive backtracker's random neighbor selection. Accept an optional seed for reproducible mazes (useful for testing).

## Risks / Trade-offs

- **Terminal compatibility** → `crossterm` abstracts most differences; full-block characters (`█`) work on all modern terminals. Mitigation: keep character mapping in one place for easy swapping.
- **Panic in raw mode leaves terminal broken** → Mitigation: use a cleanup guard (set a panic hook that restores terminal state before unwinding).
- **Fixed size may not fit small terminals** → Mitigation: choose a conservative default (e.g., 21x21 grid = 42 columns wide). Non-goal to fix for MVP.
