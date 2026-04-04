## ADDED Requirements

### Requirement: Maze generator trait
The system SHALL define a `MazeGenerator` trait with a method that accepts width, height, and an optional random seed, and returns a `Maze`.

#### Scenario: Generate maze with default algorithm
- **WHEN** a `RecursiveBacktracker` generator is invoked with width=21 and height=21
- **THEN** the returned `Maze` has a grid of 21x21 `Tile` cells

#### Scenario: Generate maze with seed for reproducibility
- **WHEN** a generator is invoked twice with the same width, height, and seed
- **THEN** both invocations return identical mazes

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

### Requirement: Start and exit placement
The system SHALL place the `Start` tile near the top-left and the `Exit` tile near the bottom-right of the maze.

#### Scenario: Start is in top-left region
- **WHEN** a maze is generated
- **THEN** the `Start` tile is at position (1, 1) in the grid

#### Scenario: Exit is in bottom-right region
- **WHEN** a maze is generated
- **THEN** the `Exit` tile is at the last traversable cell nearest to the bottom-right corner
