## ADDED Requirements

### Requirement: Status bar rendering
The system SHALL render a status bar on the first row of the terminal, above the maze grid.

#### Scenario: Status bar position
- **WHEN** the maze is rendered
- **THEN** the status bar occupies row 0 and the maze grid starts at row 1

#### Scenario: Status bar contains timer
- **WHEN** the status bar is rendered
- **THEN** it displays the countdown timer right-aligned to the maze's display width
