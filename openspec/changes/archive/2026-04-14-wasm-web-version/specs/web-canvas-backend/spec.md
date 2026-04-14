## ADDED Requirements

### Requirement: Canvas draw command consumer
The `maze-web` crate SHALL consume `Vec<DrawCommand>` and map each command to HTML5 Canvas 2D API calls. This layer SHALL contain no rendering logic — only mechanical translation.

#### Scenario: FillRect mapping
- **WHEN** the backend receives `DrawCommand::FillRect { x, y, width, height, color }`
- **THEN** it calls `canvas_ctx.fill_rect()` with pixel coordinates derived from the grid position and a configured tile size

#### Scenario: DrawText mapping
- **WHEN** the backend receives `DrawCommand::DrawText { x, y, text, color }`
- **THEN** it calls `canvas_ctx.fill_text()` at the corresponding pixel position

#### Scenario: Clear mapping
- **WHEN** the backend receives `DrawCommand::Clear`
- **THEN** it calls `canvas_ctx.clear_rect()` over the entire canvas

### Requirement: WASM entry point
The `maze-web` crate SHALL expose a wasm-bindgen entry point that initializes the game and starts the render loop.

#### Scenario: Page load
- **WHEN** the WASM module is loaded in the browser
- **THEN** it initializes game state, attaches to a `<canvas>` element, and begins rendering
