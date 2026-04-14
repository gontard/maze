## ADDED Requirements

### Requirement: DrawCommand enum
The system SHALL define a `DrawCommand` enum in `maze-core` representing low-level rendering primitives. All rendering decisions SHALL be expressed as `Vec<DrawCommand>`. The variants are: `Clear`, `DrawChar`, and `DrawText`.

#### Scenario: Draw a character
- **WHEN** the renderer needs to draw a single character at a grid position
- **THEN** it emits `DrawCommand::DrawChar { x, y, ch, color }`

#### Scenario: Draw text
- **WHEN** the renderer needs to display text
- **THEN** it emits `DrawCommand::DrawText { x, y, text, color }`

#### Scenario: Clear the screen
- **WHEN** a new frame begins
- **THEN** the first command emitted SHALL be `DrawCommand::Clear`

### Requirement: Color representation
The system SHALL define a `Color` enum in `maze-core` that is platform-independent (no crossterm or web-sys types).

#### Scenario: Colors used in rendering
- **WHEN** the renderer produces draw commands
- **THEN** colors are expressed using the platform-independent `Color` enum

### Requirement: render_frame function
The system SHALL provide a `render_frame` function in `maze-core` that takes game state (maze, player position, level, elapsed time, max time) and returns `Vec<DrawCommand>`.

#### Scenario: Rendering a game frame
- **WHEN** `render_frame` is called with current game state
- **THEN** it returns a complete list of draw commands representing the full frame

#### Scenario: Frame includes status bar
- **WHEN** `render_frame` is called
- **THEN** the returned commands include floor number and countdown timer

#### Scenario: Frame includes player
- **WHEN** `render_frame` is called with a player position
- **THEN** the returned commands include the player drawn at that position

### Requirement: Cell coordinate system
The system SHALL use a grid-based coordinate system where (x, y) corresponds to maze tile positions (column, row). Backends translate grid coordinates to pixel or character positions.

#### Scenario: Coordinate mapping
- **WHEN** a `DrawCommand::DrawChar { x: 3, y: 5, .. }` is emitted
- **THEN** it represents maze tile at column 3, row 5
