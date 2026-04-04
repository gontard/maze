## 1. Project Setup

- [ ] 1.1 Initialize Rust project with `cargo init` and add `crossterm` and `rand` dependencies
- [ ] 1.2 Create module structure: `maze.rs`, `generator.rs`, `game.rs`, `renderer.rs`

## 2. Maze Data Model

- [ ] 2.1 Define `Tile` enum (`Wall`, `Path`, `Start`, `Exit`) and `Maze` struct (grid, width, height, start, exit positions)
- [ ] 2.2 Add helper methods: `Maze::tile_at(x, y)`, `Maze::is_traversable(x, y)`

## 3. Maze Generation

- [ ] 3.1 Define `MazeGenerator` trait with `fn generate(width, height, seed) -> Maze`
- [ ] 3.2 Implement `RecursiveBacktracker` using randomized DFS — carve paths on odd-indexed cells
- [ ] 3.3 Place `Start` at (1,1) and `Exit` near bottom-right corner
- [ ] 3.4 Test: generated maze is solvable (path exists from start to exit)
- [ ] 3.5 Test: same seed produces identical maze

## 4. Terminal Rendering

- [ ] 4.1 Implement tile-to-character mapping (Wall→`██`, Path→`  `, Start→`S `, Exit→`E `, Player→`P `)
- [ ] 4.2 Implement full maze draw function using crossterm cursor positioning
- [ ] 4.3 Test: rendered output width is 2× grid width

## 5. Game Loop

- [ ] 5.1 Implement `GameState` struct (player position, timer via `Instant`, game status enum)
- [ ] 5.2 Implement player movement with wall collision blocking and boundary checks
- [ ] 5.3 Implement win detection (player reaches `Exit` tile)
- [ ] 5.4 Implement quit handling (`q` and `Esc` keys)
- [ ] 5.5 Test: movement blocked by walls, allowed on path/start/exit
- [ ] 5.6 Test: win condition triggers on exit tile

## 6. Terminal Management

- [ ] 6.1 Enable raw mode and hide cursor on game start
- [ ] 6.2 Restore terminal state on normal exit (win/quit)
- [ ] 6.3 Set panic hook to restore terminal state before unwinding

## 7. Main Integration

- [ ] 7.1 Wire up main: generate maze → init game state → run loop (input → update → render)
- [ ] 7.2 Display victory message with elapsed time on win
- [ ] 7.3 Manual playtesting and polish
