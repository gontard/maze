## Context

The maze game is a single Rust crate with clean separation between game logic (`game.rs`, `maze.rs`, `generator.rs`) and terminal display (`renderer.rs`, `main.rs`). The game logic has zero coupling to crossterm. This makes it a strong candidate for WASM compilation with minimal restructuring.

Current dependencies: `crossterm` (terminal I/O) and `rand` (maze generation).

## Goals / Non-Goals

**Goals:**
- Playable maze game in the browser via WASM + Canvas
- Shared rendering abstraction (`DrawCommand`) consumed by both terminal and web backends
- All rendering logic testable at the `maze-core` level
- Deploy to GitHub Pages with no server
- CI validates both targets

**Non-Goals:**
- Mobile/touch input support
- Sound, animations, or visual polish beyond parity with terminal version
- Multiplayer or server-side state
- Refactoring game logic (it's already clean)

## Decisions

### 1. Cargo workspace with 3 crates

```
maze/
├── Cargo.toml              (workspace root)
├── maze-core/
│   ├── Cargo.toml          (lib: rand)
│   └── src/
│       ├── lib.rs
│       ├── game.rs
│       ├── maze.rs
│       ├── generator.rs
│       └── render.rs       (DrawCommand + render_frame)
├── maze-terminal/
│   ├── Cargo.toml          (bin: maze-core, crossterm)
│   └── src/
│       ├── main.rs          (event loop, terminal setup)
│       └── renderer.rs      (DrawCommand → crossterm)
├── maze-web/
│   ├── Cargo.toml          (lib: maze-core, wasm-bindgen, web-sys, js-sys)
│   └── src/
│       ├── lib.rs           (wasm entry, rAF loop, key events)
│       └── renderer.rs      (DrawCommand → canvas)
└── web/
    └── index.html           (HTML shell with <canvas>)
```

**Why over feature flags:** The platform-specific code spans entire modules (renderer, entry point, event handling). Separate crates make it impossible to accidentally pull crossterm into WASM. Each crate compiles independently.

### 2. Low-level DrawCommand enum

```rust
pub enum DrawCommand {
    Clear,
    FillRect { x: usize, y: usize, width: usize, height: usize, color: Color },
    DrawText { x: usize, y: usize, text: String, color: Color },
}

pub enum Color {
    White,
    Red,
    Yellow,
    Green,
    // ... as needed
}
```

Coordinates are grid-based (tile positions). Backends translate to pixels or terminal characters.

**Why low-level over high-level (DrawWall, DrawPlayer):** Maximizes test coverage at the core level. Backends become mechanical translators with zero logic. Adding new visual elements never requires backend changes.

### 3. Timer abstraction

`GameState` currently uses `std::time::Instant` which doesn't work in WASM. Instead of making `GameState` platform-aware, the game loop passes elapsed time into the state from the outside.

- Terminal: computes elapsed from `Instant::now()`
- Web: computes elapsed from `performance.now()`
- `GameState` receives `elapsed_secs: f64`, no `Instant` field

This keeps `maze-core` free of platform dependencies.

### 4. wasm-pack for build tooling

Use `wasm-pack build --target web` to produce JS glue + WASM binary. No bundler needed — the `--target web` output works with native ES modules, loaded directly from `index.html`.

**Why not trunk or other tools:** wasm-pack is the most established tool for Rust→WASM. No bundler means fewer moving parts for a simple project.

### 5. GitHub Pages deployment

A GitHub Actions workflow:
1. Install wasm-pack
2. `cargo test --workspace`
3. `wasm-pack test --node -p maze-core`
4. `wasm-pack build --target web -p maze-web --out-dir ../web/pkg`
5. Deploy `web/` directory to GitHub Pages via `actions/deploy-pages`

No Playwright. The WASM build compiling is the smoke test for the web target.

## Risks / Trade-offs

- **[Risk] `rand` crate WASM compatibility** → `rand` works in WASM with `getrandom/js` feature. Need to enable it in `maze-core`'s `Cargo.toml` when building for WASM target.
  Mitigation: Add `getrandom = { version = "0.3", features = ["wasm_js"] }` as a dependency with `[target.'cfg(target_arch = "wasm32")'.dependencies]`.

- **[Risk] Terminal renderer regression** → Rewriting the terminal renderer to consume DrawCommands could introduce visual bugs.
  Mitigation: Existing 77 tests migrate and validate core rendering logic. Terminal-specific mapping is thin enough to verify visually.

- **[Trade-off] No web renderer tests** → We accept no automated testing of the canvas mapping layer.
  Mitigation: The layer is deliberately minimal — a match statement mapping DrawCommands to canvas calls. If it compiles, it likely works.

- **[Trade-off] Timer moved outside GameState** → Slightly changes the GameState API (removes `start_time` field, `elapsed_secs` becomes a parameter).
  Mitigation: All callers are in the game loop, easy to update. Tests become simpler (no need for sleep/mock time).

## Open Questions

- Exact canvas tile size in pixels (16x16? 24x24? Configurable?)
- Should the web version show a game-over screen, or just stop rendering?
- Floor progression: does the web version auto-advance like terminal, or show an interstitial?
