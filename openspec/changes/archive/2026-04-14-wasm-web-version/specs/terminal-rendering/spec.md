## MODIFIED Requirements

### Requirement: Full maze redraw
The system SHALL consume `Vec<DrawCommand>` from `render_frame` and translate each command to crossterm output. The terminal renderer SHALL contain no rendering decisions — only mechanical mapping from draw commands to crossterm calls.

#### Scenario: Maze updates after move
- **WHEN** the player moves
- **THEN** the terminal backend calls `render_frame`, iterates the returned draw commands, and outputs corresponding crossterm calls

#### Scenario: FillRect mapping
- **WHEN** the backend receives `DrawCommand::FillRect { x, y, width, height, color }`
- **THEN** it outputs the appropriate crossterm colored block characters at the terminal position

#### Scenario: DrawText mapping
- **WHEN** the backend receives `DrawCommand::DrawText { x, y, text, color }`
- **THEN** it outputs the text via crossterm at the terminal position

#### Scenario: Clear mapping
- **WHEN** the backend receives `DrawCommand::Clear`
- **THEN** it clears the terminal screen via crossterm
