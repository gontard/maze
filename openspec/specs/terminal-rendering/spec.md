## ADDED Requirements

### Requirement: 2-char-wide cell rendering
The system SHALL render each maze tile as 2 characters wide to produce visually square cells in the terminal.

#### Scenario: Wall tile rendering
- **WHEN** a `Wall` tile is rendered
- **THEN** it displays as `██` (two full-block characters)

#### Scenario: Path tile rendering
- **WHEN** a `Path` tile is rendered
- **THEN** it displays as two space characters

#### Scenario: Start tile rendering
- **WHEN** the `Start` tile is rendered without the player on it
- **THEN** it displays as `S `

#### Scenario: Exit tile rendering
- **WHEN** the `Exit` tile is rendered
- **THEN** it displays as `E `

### Requirement: Player rendering
The system SHALL render the player position as `P ` overlaid on whatever tile the player occupies.

#### Scenario: Player displayed on maze
- **WHEN** the maze is rendered
- **THEN** the player's current position shows `P ` instead of the underlying tile

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

### Requirement: Tile-to-character mapping is centralized
The system SHALL define all tile-to-character mappings in the renderer module, so game logic never deals with display characters.

#### Scenario: Adding a new tile type
- **WHEN** a new `Tile` variant is added
- **THEN** only the renderer module needs a new character mapping to display it

### Requirement: Status bar rendering
The system SHALL render a status bar on the first row of the terminal, above the maze grid, showing the floor number and countdown timer.

#### Scenario: Status bar position
- **WHEN** the maze is rendered
- **THEN** the status bar occupies row 0 and the maze grid starts at row 1

#### Scenario: Status bar contains floor number
- **WHEN** the status bar is rendered on floor N
- **THEN** it displays "Floor N" left-aligned within the maze's display width

#### Scenario: Status bar contains timer
- **WHEN** the status bar is rendered
- **THEN** it displays the countdown timer right-aligned to the maze's display width
