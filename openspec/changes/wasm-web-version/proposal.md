## Why

Make the maze game playable in a browser with zero server infrastructure. A WASM build on GitHub Pages lets anyone play without installing anything.

## What Changes

- Restructure into a Cargo workspace with 3 crates: `maze-core` (lib), `maze-terminal` (bin), `maze-web` (lib/WASM)
- Introduce a `DrawCommand` rendering abstraction in `maze-core` — all rendering logic produces `Vec<DrawCommand>`, both backends consume it
- **BREAKING**: `maze-terminal` replaces the current single-crate binary. The terminal renderer switches from direct crossterm output to consuming `DrawCommand`s
- Add `maze-web` crate: thin Canvas backend mapping `DrawCommand` to canvas API calls, `requestAnimationFrame` game loop, keyboard event listeners
- Add `wasm-pack` build for `maze-web`
- Add GitHub Pages deployment (static HTML shell + WASM binary)
- CI: `cargo test --workspace` + `wasm-pack test --node -p maze-core` + `wasm-pack build -p maze-web`

## Capabilities

### New Capabilities
- `draw-command`: Low-level rendering abstraction (`DrawCommand` enum) in maze-core. Both backends consume it. All rendering decisions live here — backends are dumb translators.
- `web-canvas-backend`: Thin WASM/Canvas backend mapping DrawCommands to HTML5 Canvas calls. No logic, no tests — thin enough to trust by inspection.
- `web-game-loop`: Browser game loop using `requestAnimationFrame` + keyboard event listeners, driving the same game state as the terminal version.
- `github-pages-deploy`: Static deployment of WASM build to GitHub Pages via GitHub Actions.

### Modified Capabilities
- `terminal-rendering`: Renderer switches from building crossterm output directly to consuming `Vec<DrawCommand>` from core.

## Impact

- **Project structure**: Single crate → Cargo workspace with 3 members
- **Dependencies**: New deps: `wasm-bindgen`, `web-sys`, `js-sys`, `wasm-pack` (build tool). `crossterm` and `rand` move to appropriate crates.
- **Existing tests**: 77 tests migrate to `maze-core` and `maze-terminal`. Core tests also run under WASM via `wasm-pack test --node`.
- **CI**: New steps for WASM build/test and GitHub Pages deployment.
- **No behavioral changes** to the terminal game.
