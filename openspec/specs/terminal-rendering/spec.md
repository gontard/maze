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
The system SHALL redraw the entire maze on each player move.

#### Scenario: Maze updates after move
- **WHEN** the player moves
- **THEN** the terminal displays the updated maze with the player at the new position

### Requirement: Tile-to-character mapping is centralized
The system SHALL define all tile-to-character mappings in the renderer module, so game logic never deals with display characters.

#### Scenario: Adding a new tile type
- **WHEN** a new `Tile` variant is added
- **THEN** only the renderer module needs a new character mapping to display it
