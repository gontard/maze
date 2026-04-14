## MODIFIED Requirements

### Requirement: Maze generator trait
The system SHALL define a `MazeGenerator` trait with a method that accepts width, height, an optional random seed, and an optional start position, and returns a `Maze`. Generator internals SHALL use a named `MazeSetup` struct instead of a tuple for initialization state. Variable names in generators SHALL be descriptive (`room_w` not `rw`, `grid_ax` not `gax`).

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

## ADDED Requirements

### Requirement: Parameterized generator tests
Generator tests for properties shared across all algorithms (dimensions, reachability, solvability, border walls, determinism, custom start) SHALL be defined once via a macro and instantiated for each `MazeGenerator` implementation.

#### Scenario: All algorithms tested with same properties
- **WHEN** generator tests are run
- **THEN** RecursiveBacktracker, Kruskal, and Prim each pass the shared property suite

### Requirement: Remove unused import
The `use rand::prelude::IndexedRandom` import in generator.rs SHALL be removed as it is unused.

#### Scenario: Clean imports
- **WHEN** generator.rs is compiled
- **THEN** no unused import warnings are emitted
