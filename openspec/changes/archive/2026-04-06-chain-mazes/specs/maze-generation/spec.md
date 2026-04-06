## MODIFIED Requirements

### Requirement: Maze generator trait
The system SHALL define a `MazeGenerator` trait with a method that accepts width, height, an optional random seed, and an optional start position, and returns a `Maze`.

#### Scenario: Generate maze with default algorithm
- **WHEN** a `RecursiveBacktracker` generator is invoked with width=21 and height=21
- **THEN** the returned `Maze` has a grid of 21x21 `Tile` cells

#### Scenario: Generate maze with seed for reproducibility
- **WHEN** a generator is invoked twice with the same width, height, seed, and start position
- **THEN** both invocations return identical mazes

#### Scenario: Generate maze with custom start position
- **WHEN** a generator is invoked with start position (x, y)
- **THEN** the `Start` tile is placed at (x, y)
- **AND** the maze is carved starting from that position

#### Scenario: Generate maze with no start position
- **WHEN** a generator is invoked with no start position (None)
- **THEN** the `Start` tile is placed at (1, 1)

### Requirement: Start and exit placement
The system SHALL place the `Start` tile at the specified start position (defaulting to (1,1)) and the `Exit` tile at the farthest reachable point from the start.

#### Scenario: Start is at specified position
- **WHEN** a maze is generated with start position (x, y)
- **THEN** the `Start` tile is at position (x, y) in the grid

#### Scenario: Start defaults to (1,1)
- **WHEN** a maze is generated with no start position specified
- **THEN** the `Start` tile is at position (1, 1) in the grid

#### Scenario: Exit is farthest reachable point
- **WHEN** a maze is generated with start position S
- **THEN** the `Exit` tile is placed at the traversable cell with the maximum BFS distance from S
