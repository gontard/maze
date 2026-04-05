## 1. Project Setup

- [x] 1.1 Initialize Rust project with `cargo init` and add `crossterm` and `rand` dependencies
- [x] 1.2 Create module structure: `maze.rs`, `generator.rs`, `game.rs`, `renderer.rs`

## 2. Maze Data Model

- [x] 2.1 Define `Tile` enum (`Wall`, `Path`, `Start`, `Exit`) and `Maze` struct (grid, width, height, start, exit positions)
- [x] 2.2 Add helper methods: `Maze::tile_at(x, y)`, `Maze::is_traversable(x, y)`

## 3. Maze Generation

- [x] 3.1 Define `MazeGenerator` trait with `fn generate(width, height, seed) -> Maze`
- [x] 3.2 Implement `RecursiveBacktracker` using randomized DFS — carve paths on odd-indexed cells
- [x] 3.3 Place `Start` at (1,1) and `Exit` near bottom-right corner
- [x] 3.4 Test: generated maze is solvable (path exists from start to exit)
- [x] 3.5 Test: same seed produces identical maze

## 4. Terminal Rendering

- [x] 4.1 Implement tile-to-character mapping (Wall→`██`, Path→`  `, Start→`S `, Exit→`E `, Player→`P `)
- [x] 4.2 Implement full maze draw function using crossterm cursor positioning
- [x] 4.3 Test: rendered output width is 2× grid width

## 5. Game Loop

- [x] 5.1 Implement `GameState` struct (player position, timer via `Instant`, game status enum)
- [x] 5.2 Implement player movement with wall collision blocking and boundary checks
- [x] 5.3 Implement win detection (player reaches `Exit` tile)
- [x] 5.4 Implement quit handling (`q` and `Esc` keys)
- [x] 5.5 Test: movement blocked by walls, allowed on path/start/exit
- [x] 5.6 Test: win condition triggers on exit tile

## 6. Terminal Management

- [x] 6.1 Enable raw mode and hide cursor on game start
- [x] 6.2 Restore terminal state on normal exit (win/quit)
- [x] 6.3 Set panic hook to restore terminal state before unwinding

## 7. Main Integration

- [x] 7.1 Wire up main: generate maze → init game state → run loop (input → update → render)
- [x] 7.2 Display victory message with elapsed time on win
- [x] 7.3 Manual playtesting and polish
