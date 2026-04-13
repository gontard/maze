## 1. Workspace restructure

- [ ] 1.1 Convert to Cargo workspace: root `Cargo.toml` with `maze-core`, `maze-terminal`, `maze-web` members
- [ ] 1.2 Create `maze-core` lib crate: move `game.rs`, `maze.rs`, `generator.rs` into it, expose public API via `lib.rs`
- [ ] 1.3 Create `maze-terminal` bin crate: move `main.rs` and `renderer.rs`, depend on `maze-core` and `crossterm`
- [ ] 1.4 Verify all 77 existing tests pass with `cargo test --workspace`

## 2. DrawCommand abstraction

- [ ] 2.1 Define `DrawCommand` enum and `Color` enum in `maze-core::render`
- [ ] 2.2 Implement `render_frame` function returning `Vec<DrawCommand>` (maze grid, player, status bar)
- [ ] 2.3 Write tests for `render_frame`: verify commands include Clear, wall rects, player, floor text, timer text
- [ ] 2.4 Migrate existing renderer tests to assert on `Vec<DrawCommand>` output

## 3. Timer refactor

- [ ] 3.1 Remove `start_time: Instant` from `GameState`, accept elapsed time as parameter in `check_timeout`
- [ ] 3.2 Update `maze-terminal` game loop to compute elapsed from `Instant` and pass it in
- [ ] 3.3 Update existing timer tests

## 4. Terminal backend

- [ ] 4.1 Rewrite `maze-terminal` renderer to consume `Vec<DrawCommand>` and emit crossterm calls
- [ ] 4.2 Verify terminal game plays correctly (manual test)

## 5. Web backend

- [ ] 5.1 Create `maze-web` lib crate with `wasm-bindgen`, `web-sys`, `js-sys` dependencies
- [ ] 5.2 Implement canvas renderer: map `DrawCommand` to canvas 2D API calls
- [ ] 5.3 Implement `requestAnimationFrame` game loop with elapsed time from `performance.now()`
- [ ] 5.4 Implement keyboard event listener: arrow keys, WASD, Escape → `Direction` / quit
- [ ] 5.5 Create WASM entry point: init game state, attach to `<canvas>`, start loop
- [ ] 5.6 Add `getrandom` WASM compatibility for `rand` crate

## 6. Web shell

- [ ] 6.1 Create `web/index.html` with `<canvas>` element and ES module WASM loader
- [ ] 6.2 Verify game works locally with `wasm-pack build --target web` and a local HTTP server

## 7. CI and deployment

- [ ] 7.1 Add GitHub Actions workflow: `cargo test --workspace` + `wasm-pack test --node -p maze-core` + `wasm-pack build`
- [ ] 7.2 Add GitHub Pages deployment step to the workflow
