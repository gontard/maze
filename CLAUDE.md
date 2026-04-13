# Maze

Rust maze game. Players navigate procedurally generated mazes with a countdown timer. Workspace with terminal and web (WASM/Canvas) frontends sharing core logic via `DrawCommand` abstraction.

## Structure

- `maze-core` — lib: game logic, maze generation, render commands (no IO)
- `maze-terminal` — bin: crossterm terminal frontend
- `maze-web` — cdylib: WASM + Canvas frontend, deployed to GitHub Pages

## Pre-commit checks

```bash
cargo fmt --all
RUSTFLAGS="-D warnings" cargo build --workspace
cargo test --workspace
```
