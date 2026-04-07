## ADDED Requirements

### Requirement: MazeGenerator provides a name
The `MazeGenerator` trait SHALL include a `name()` method returning a `&'static str` identifying the algorithm.

#### Scenario: Each algorithm reports its name
- **WHEN** `name()` is called on a generator
- **THEN** it returns a human-readable algorithm name (e.g. "Recursive Backtracker", "Kruskal's", "Prim's")

### Requirement: Kruskal's algorithm
The system SHALL implement Kruskal's randomized algorithm as a `MazeGenerator`.

#### Scenario: Kruskal generates valid maze
- **WHEN** a `Kruskal` generator is invoked with width=21 and height=21
- **THEN** the returned `Maze` has a grid of 21x21 `Tile` cells
- **AND** all `Path`/`Start` cells are reachable from each other
- **AND** the outer border is all `Wall`

#### Scenario: Kruskal is deterministic with seed
- **WHEN** `Kruskal` is invoked twice with the same seed and start
- **THEN** both invocations return identical mazes

#### Scenario: Kruskal supports custom start
- **WHEN** `Kruskal` is invoked with start position (x, y)
- **THEN** the `Start` tile is at (x, y)

### Requirement: Prim's algorithm
The system SHALL implement Prim's randomized algorithm as a `MazeGenerator`.

#### Scenario: Prim generates valid maze
- **WHEN** a `Prim` generator is invoked with width=21 and height=21
- **THEN** the returned `Maze` has a grid of 21x21 `Tile` cells
- **AND** all `Path`/`Start` cells are reachable from each other
- **AND** the outer border is all `Wall`

#### Scenario: Prim is deterministic with seed
- **WHEN** `Prim` is invoked twice with the same seed and start
- **THEN** both invocations return identical mazes

#### Scenario: Prim supports custom start
- **WHEN** `Prim` is invoked with start position (x, y)
- **THEN** the `Start` tile is at (x, y)

### Requirement: Random algorithm selection per floor
The game loop SHALL randomly select one of the three algorithms (uniform distribution) for each floor.

#### Scenario: Algorithm varies across floors
- **WHEN** multiple floors are generated
- **THEN** each floor independently selects a random algorithm

### Requirement: Algorithm name displayed
The renderer SHALL display the algorithm name on the HUD alongside floor number and timer.

#### Scenario: HUD shows algorithm
- **WHEN** a floor is rendered
- **THEN** the algorithm name is visible to the player
