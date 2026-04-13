## ADDED Requirements

### Requirement: requestAnimationFrame game loop
The web game loop SHALL use `requestAnimationFrame` to drive frame rendering. Each frame SHALL call `render_frame` from `maze-core` and paint the resulting draw commands to the canvas.

#### Scenario: Frame rendering
- **WHEN** `requestAnimationFrame` fires
- **THEN** the system updates elapsed time, calls `render_frame`, and paints the result

#### Scenario: Game over stops loop
- **WHEN** game status is Won, Lost, or Quit
- **THEN** the `requestAnimationFrame` loop stops

### Requirement: Keyboard input
The web game loop SHALL listen for `keydown` events and translate them to `Direction` values consumed by `GameState::move_player`.

#### Scenario: Arrow key mapping
- **WHEN** the user presses ArrowUp, ArrowDown, ArrowLeft, or ArrowRight
- **THEN** the system calls `move_player` with the corresponding `Direction`

#### Scenario: WASD key mapping
- **WHEN** the user presses W, A, S, or D
- **THEN** the system calls `move_player` with the corresponding `Direction`

#### Scenario: Quit key
- **WHEN** the user presses Escape
- **THEN** the system calls `quit` on the game state

### Requirement: Timer uses browser time
The web game loop SHALL use `performance.now()` (via `web-sys`) for elapsed time tracking instead of `std::time::Instant`.

#### Scenario: Time tracking
- **WHEN** the game is running in the browser
- **THEN** elapsed time is computed from `performance.now()` and passed to `render_frame`
