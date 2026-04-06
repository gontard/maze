## ADDED Requirements

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

### Requirement: Maze structure
The system SHALL represent a maze as a 2D grid of `Tile` enum values with exactly one `Start` tile and one `Exit` tile.

#### Scenario: Maze has start and exit
- **WHEN** a maze is generated
- **THEN** the grid contains exactly one `Start` tile and exactly one `Exit` tile

#### Scenario: Maze is solvable
- **WHEN** a maze is generated
- **THEN** there EXISTS a path of adjacent `Path`/`Start`/`Exit` tiles connecting `Start` to `Exit`

### Requirement: Tile types
The system SHALL define a `Tile` enum with at least the variants: `Wall`, `Path`, `Start`, `Exit`.

#### Scenario: Wall cells block movement
- **WHEN** a tile is `Wall`
- **THEN** it is not traversable

#### Scenario: Path cells allow movement
- **WHEN** a tile is `Path`, `Start`, or `Exit`
- **THEN** it is traversable

### Requirement: Recursive backtracker algorithm
The system SHALL implement the recursive backtracker (randomized DFS) algorithm as a `MazeGenerator`.

#### Scenario: All path cells are reachable
- **WHEN** a maze is generated with the recursive backtracker
- **THEN** every `Path`, `Start`, and `Exit` tile is reachable from every other such tile

### Requirement: Start placement in generator
The system SHALL place the `Start` tile at the specified start position (defaulting to (1,1)) during generation. The generator does NOT place the exit.

#### Scenario: Start is at specified position
- **WHEN** a maze is generated with start position (x, y)
- **THEN** the `Start` tile is at position (x, y) in the grid

#### Scenario: Start defaults to (1,1)
- **WHEN** a maze is generated with no start position specified
- **THEN** the `Start` tile is at position (1, 1) in the grid

#### Scenario: Generator returns maze without exit
- **WHEN** `generate()` is called
- **THEN** the returned maze has a `Start` tile but no `Exit` tile in the grid

### Requirement: Exit placement via place_exit
The system SHALL place the `Exit` tile via a separate `Maze::place_exit()` method at the farthest reachable point from the start.

#### Scenario: place_exit sets exit at farthest point
- **GIVEN** a maze with corridors (and optionally rooms) but no exit
- **WHEN** `maze.place_exit()` is called
- **THEN** the `Exit` tile is placed at the traversable cell with maximum BFS distance from start
- **AND** `maze.exit` is updated to that position

#### Scenario: place_exit works after room carving
- **GIVEN** a maze that has had rooms carved into it
- **WHEN** `maze.place_exit()` is called
- **THEN** the exit is at the farthest point considering the room shortcuts
