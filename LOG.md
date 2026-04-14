# Development Log

## Session 2026-04-04: Maze Game MVP

### What we built

A terminal-based maze game in Rust. The player navigates a procedurally generated maze to reach the exit, timed.

### Architecture

Four modules with clear separation:

- **maze.rs** - `Tile` enum (Wall, Path, Start, Exit) and `Maze` struct with grid, dimensions, start/exit positions, helper methods (`tile_at`, `is_traversable`)
- **generator.rs** - `MazeGenerator` trait + `RecursiveBacktracker` implementation using randomized DFS. Seeded RNG for reproducibility. Carves paths on odd-indexed cells.
- **game.rs** - `GameState` with player position, timer (`Instant`), status enum. Movement with wall collision, win detection on exit, quit via q/Esc.
- **renderer.rs** - 1-char-per-cell rendering via crossterm. `#` walls, space paths, `@` player, `S` start, `E` exit. Uses `\r\n` for raw mode compatibility.

### Key decisions made during exploration

- **Rust + crossterm** for terminal interaction
- **Recursive backtracker** behind a trait for easy algorithm swapping
- **Cell grid model** (walls are full cells, not thin lines between cells)
- **1-char-per-cell rendering** after iterating through several approaches (2-char `██`, `##`, `[]`, mixed Unicode). Full-block `█` renders as double-width in Warp terminal, causing alignment issues. Single ASCII chars were the most reliable.
- **Maze size**: 41x21 (wider than tall for better terminal proportions)

### Workflow

Used OpenSpec for structured planning (proposal, design, specs, tasks), then TDD for implementation. 32 tests covering maze structure, generation solvability, seed reproducibility, rendering, movement, collision, and win condition.

### Commits

1. `89c0243` - Initial commit with OpenSpec artifacts
2. `6134ea2` - Full implementation (all modules, tests, game loop)
3. `094c60a` - Archive change, sync specs to main

### What's next

Potential features discussed during exploration: fog of war, new tile types (trees, rocks, water), different generation algorithms (Kruskal's, Prim's), dynamic sizing, color, difficulty levels.

## Session 2026-04-05: Countdown Timer

### What we built

A visible countdown timer with a time-based lose condition. The timer displays above the maze in a status bar, updates continuously, and changes color as time runs out.

### Changes

- **maze.rs** - Added `Maze::solve()` BFS method returning shortest path length (`Option<usize>`)
- **game.rs** - New `GameStatus::Lost` variant, `max_time_secs` field, `check_timeout()` method that transitions to `Lost` when time expires
- **renderer.rs** - Status bar row above the maze with right-aligned timer (`⏱ MM:SS / MM:SS`). Color urgency via crossterm: cyan (>25% remaining), dark yellow (10-25%), red (<10%). Helper functions `format_timer`, `timer_urgency`, `urgency_color`.
- **main.rs** - Poll-based game loop (`event::poll(100ms)` instead of blocking `event::read()`), computes max time from solution path length × 0.375s, handles `Lost` status with timeout message

### Key decisions made during exploration

- **Status bar above maze** (dedicated row) rather than overlaying on maze row 0
- **Max time from solution path length** (BFS shortest path × multiplier) rather than fixed time or area-based
- **Poll-based loop** for continuous timer updates even when idle, rather than a separate rendering thread
- **Time multiplier tuned to 0.375s/step** — started at 1.5s but was too generous, divided by 4 after playtesting
- **Cyan for normal timer** — white was barely visible on terminal; dark yellow replaced yellow for the same reason

### Workflow

OpenSpec explore mode for design discussion, then propose for artifacts, TDD for implementation. 49 tests total (17 new: 3 solver, 5 game timer, 9 renderer).

### Commits

1. `bda001e` - Add countdown timer with time-based lose condition
2. `537cb1b` - Archive add-timer change and sync specs

### What's next

Potential features: fog of war, difficulty levels (maze size + time scaling), pause/resume, configurable time multiplier via CLI args.

## Session 2026-04-06: Maze Chaining (Infinite Tower)

### What we built

Infinite floor progression — when the player reaches the exit, a new maze generates around them and they continue on the next floor. The game becomes an endless tower climb until timeout or quit.

### Changes

- **generator.rs** - `MazeGenerator::generate` now accepts optional start position (`Option<(usize, usize)>`). Replaced `find_bottom_right_path` with `farthest_reachable()` — BFS from start to find the cell with maximum distance, maximizing challenge per floor. Carving starts from the provided position instead of hardcoded (1,1).
- **renderer.rs** - `render()` and `render_to_string()` take a `level: usize` parameter. Status bar now shows "Floor N" left-aligned and timer right-aligned, padded to maze width. `build_status_bar` updated for the two-element layout.
- **main.rs** - Outer level loop wrapping the game loop. Tracks `level` counter and `start_pos`. On `Won`: generates new maze with `start = previous exit`, fresh `GameState`, increments level. On `Lost`/`Quit`: breaks out and shows floor count ("You cleared N floors").

### Key decisions made during exploration

- **Farthest reachable point for exit** rather than bottom-right heuristic — when start varies across floors, bottom-right might place exit adjacent to start. BFS farthest guarantees maximum traversal.
- **"Elevator" transition** — no animation, maze simply changes around the player. Mental model: square tower, player takes elevator to new floor.
- **Level in render params, not GameState** — level is display context, not game mechanics. Keeps GameState focused on movement/timer/status.
- **Timer resets per floor** with fresh time budget from each floor's solution path length.
- **Same maze dimensions** (41x21) on all floors — no difficulty progression for now.

### Workflow

OpenSpec explore mode for design discussion, then propose for all artifacts, TDD for implementation. 56 tests total (7 new: farthest exit verification, custom start position, floor label rendering).

### Commits

1. `56c2302` - Add infinite maze chaining with floor progression
2. `f866675` - Archive chain-mazes change and sync specs

### What's next

Potential features: increasing difficulty per floor (larger mazes, tighter timers), score system, fog of war, pause/resume.

## Session 2026-04-06: Room Carving Modifier

### What we built

A composable room modifier that carves rectangular open areas into any generated maze. Rooms break up pure corridor layouts, creating landmarks and multi-exit decision points under time pressure.

### Changes

- **maze.rs** - New `Maze::place_exit()` method (BFS farthest-reachable, extracted from generator). New `Maze::carve_rooms()` method that carves random odd-aligned rectangles, converting walls to paths. Only removes walls — connectivity preserved by construction.
- **generator.rs** - `RecursiveBacktracker::generate()` no longer places exit. Returns maze with `Start` tile only. Removed `farthest_reachable` (moved to `maze.rs`).
- **main.rs** - Pipeline changed to `generate() -> carve_rooms(3, 3, 5) -> place_exit()`. Added `rand` imports for room RNG.

### Key decisions made during exploration

- **Room modifier (composable)** rather than rooms-first algorithm — rooms layer on top of any generation algorithm, keeping algorithm variety orthogonal to room carving.
- **Exit placement extracted from generator** — modifiers change grid topology, so exit must be placed after all modifiers run. `place_exit()` became a separate `Maze` method.
- **Rooms only remove walls** — key invariant that guarantees connectivity. Removing walls from a perfect maze can only add paths, never block them.
- **Odd-aligned coordinates and sizes** — matches the maze grid structure where paths are on odd indices and walls on even.
- **Count as attempts, not guarantees** — if a room doesn't fit (border constraint), it's skipped. Avoids infinite retry loops on small mazes.

### Workflow

OpenSpec explore mode for design discussion (algorithm comparison, room approaches, pipeline design), then propose for all artifacts, TDD for implementation. Refactored exit placement first as a safe commit (56 tests passing), then wrote 7 failing room tests, then implemented. 63 tests total (7 new: wall-to-path only, border integrity, solvability, reachability, determinism, wall count reduction, zero-count no-op).

### Commits

1. `0b9f92b` - Refactor: extract exit placement from generator into Maze::place_exit()
2. `fd77fab` - Add failing tests for carve_rooms room modifier (TDD red phase)
3. `2cb5a7d` - Implement carve_rooms room modifier
4. `8a7e7fb` - Integrate room carving into game pipeline
5. `62d30d5` - Archive add-rooms change and sync specs

### What's next

Potential features discussed during exploration: multiple generation algorithms (Kruskal's, Sidewinder — contrasting maze textures), fog of war, minimap, maze size scaling per floor, score system, animated wall dissolve.

## Session 2026-04-07: Multiple Maze Algorithms

### What we built

Two new maze generation algorithms — Kruskal's and Prim's — with random per-floor selection. Each floor now independently picks one of three algorithms (uniform random), giving visual and gameplay variety across the tower climb.

### Changes

- **generator.rs** - Added `Kruskal` and `Prim` structs implementing `MazeGenerator`. Extracted shared `init_maze()` and `finish_maze()` helpers from `RecursiveBacktracker` to avoid duplication. Added private `UnionFind` data structure for Kruskal's. 14 new tests (7 per algorithm: dimensions, reachability, solvability, borders, determinism, custom start).
- **main.rs** - Replaced single `RecursiveBacktracker` with random selection via `rng.random_range(0..3)` match. Moved RNG creation before the floor loop so it persists across floors.
- **CLAUDE.md** - New project-level config with `RUSTFLAGS="-D warnings"` pre-commit check (warnings-as-errors for CI readiness without blocking local dev).

### Key decisions made during exploration

- **Option C (no trait objects, no enum wrapper)** — the `MazeGenerator` trait provides the contract, a simple match provides the selection. No dynamic dispatch needed for 3 algorithms picked once per floor.
- **No algorithm name on HUD** — explored displaying "Floor N — Algorithm" but "Recursive Backtracker" was too long for the 41-char status bar. Dropped the feature rather than abbreviating.
- **Uniform random selection** — no weighting or difficulty progression for now. Keeps it simple, can revisit later.
- **Warnings-as-errors via RUSTFLAGS** — caught an unused `name()` method after removing HUD display. Added `RUSTFLAGS="-D warnings"` as a pre-commit check rather than `#[deny(warnings)]` in code, which would be painful during TDD red phases.

### Algorithm characteristics

- **Recursive Backtracker** — long winding corridors, few dead ends, high "river" factor
- **Kruskal's** — many short dead ends, branchy organic texture, uses union-find
- **Prim's** — shorter corridors, radial growth pattern from start, frontier-based

### Workflow

OpenSpec explore mode for algorithm comparison and design (Option A/B/C trade-offs), then propose for artifacts, TDD for implementation. 77 tests total (14 new generator tests + removed 2 unused name tests, net +12 from previous 63, then +2 from CLAUDE.md session).

### Commits

1. `c326116` - Implement Kruskal's and Prim's maze algorithms with random per-floor selection
2. `09640c1` - Add project CLAUDE.md with warnings-as-errors pre-commit check

### What's next

Potential features: algorithm name display (shorter names or wider maze), difficulty progression (algorithm weighting by floor), more algorithms (Eller's, Wilson's, Sidewinder), fog of war, minimap, score system.

## Session 2026-04-13: WASM Web Version

### What we built

A browser-playable version of the maze game compiled to WASM, rendered on HTML5 Canvas with monospace characters. Restructured the project into a Cargo workspace with a shared `DrawCommand` rendering abstraction consumed by both terminal and web backends.

### Architecture

Three-crate workspace:

- **maze-core** (lib) — game logic, maze generation, `DrawCommand`-based rendering. Zero platform dependencies. All rendering decisions live here.
- **maze-terminal** (bin) — crossterm frontend. Thin translator: `DrawCommand` → crossterm calls.
- **maze-web** (cdylib) — WASM/Canvas frontend. Thin translator: `DrawCommand` → Canvas 2D API calls. `requestAnimationFrame` game loop, `keydown` event listeners.

The `DrawCommand` enum is the key abstraction:
- `Clear` — new frame
- `DrawChar { x, y, ch, color }` — single character at grid position (tiles, player)
- `DrawText { x, y, text, color }` — text string (status bar labels)
- `FillRect { x, y, width, height, color }` — filled rectangle (available but unused currently)

### Key decisions made during exploration

- **Workspace over feature flags** — platform-specific code spans entire modules (renderer, entry point, event handling). Separate crates make it impossible to accidentally pull crossterm into WASM. Each crate compiles independently.
- **Low-level DrawCommand** — maximizes test coverage at core level. Backends are mechanical translators with zero logic.
- **Timer externalized from GameState** — removed `start_time: Instant` (not available in WASM). Game loop passes `elapsed_secs: f64` into `check_timeout()`. Terminal uses `Instant::now()`, web uses `performance.now()`.
- **Character-based Canvas rendering** — initially tried colored rectangles (`FillRect`), but lost the rogue-like aesthetic. Switched to `DrawChar` with monospace font on Canvas, preserving the `#`, `@`, `E`, `S` characters.
- **Cell proportions 10x18px** — monospace terminal characters have ~0.6:1 width:height ratio. Square tiles made the web version too wide.
- **wasm-pack with `--target web`** — ES module output, no bundler needed. Simple HTML shell loads WASM directly.
- **No Playwright in CI** — WASM build compiling is the smoke test. Canvas layer is thin enough to trust by inspection.

### Changes

- **Cargo.toml** — workspace root with 3 members
- **maze-core/src/render.rs** — new module: `DrawCommand`, `Color` enums, `render_frame()` function, `format_timer()`, `timer_urgency()`, `tile_char()`. 15 tests.
- **maze-core/src/game.rs** — removed `start_time: Instant` field, `check_timeout()` now takes `elapsed_secs` parameter, removed `elapsed_secs()` method.
- **maze-terminal/src/renderer.rs** — rewritten to consume `Vec<DrawCommand>` from core.
- **maze-terminal/src/main.rs** — computes elapsed from `Instant` and passes to `check_timeout()` and `render_frame()`.
- **maze-web/src/lib.rs** — WASM entry point: game state, `requestAnimationFrame` loop, keyboard handler, floor progression.
- **maze-web/src/renderer.rs** — Canvas painter: `DrawChar` → `fill_text()`, `DrawText` → `fill_text()`, `Clear` → `fill_rect()` black.
- **web/index.html** — minimal HTML shell with `<canvas>` and ES module WASM loader.
- **.github/workflows/ci.yml** — CI: `cargo test --workspace`, `wasm-pack test --node`, `wasm-pack build`, GitHub Pages deploy on main.

### Workflow

OpenSpec explore mode for design discussion (WASM feasibility, architecture options, testing strategy), then propose for all artifacts (proposal, 5 specs, design, 23 tasks), then implementation. Iterated on web rendering after first playtest — switched from colored rectangles to character-based rendering with correct proportions. 75 tests total (15 new render tests, timer tests updated for new API, old terminal renderer tests replaced by core DrawCommand tests).

### Commits

1. `cce6b09` - Add WASM web version change proposal with specs, design, and tasks
2. `4d08790` - Restructure into workspace with WASM web target
3. `2a64985` - Fix web rendering: character-based display, proportions, and timer

### What's next

Potential features from Obsidian backlog: secret passages, monsters with weapons/spells, inventory system. Open questions from design: canvas tile size tuning, game-over screen for web, floor interstitial screen.
