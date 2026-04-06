## Requirements

### Requirement: Room carving modifier
The system SHALL provide a `carve_rooms` function that carves rectangular open areas into an existing maze grid.

#### Scenario: Rooms only remove walls
- **GIVEN** a generated maze
- **WHEN** `carve_rooms` is applied
- **THEN** every tile that was `Path`, `Start`, or `Exit` before carving remains unchanged or stays `Path`
- **AND** only `Wall` tiles may be converted to `Path`

#### Scenario: Border integrity preserved
- **GIVEN** a generated maze with all-wall outer border
- **WHEN** `carve_rooms` is applied
- **THEN** the outer border remains all `Wall` tiles

#### Scenario: Maze remains solvable
- **GIVEN** a generated maze with exit placed
- **WHEN** `carve_rooms` is applied and exit is re-placed
- **THEN** the maze has a valid solution path from start to exit

#### Scenario: All cells reachable
- **GIVEN** a generated maze after room carving
- **WHEN** BFS is run from the start position
- **THEN** every `Path`, `Start`, and `Exit` tile is reachable

#### Scenario: Deterministic with seed
- **GIVEN** the same maze and the same RNG seed
- **WHEN** `carve_rooms` is applied twice
- **THEN** both results are identical

#### Scenario: Room count bounded by attempts
- **GIVEN** a maze and a room count parameter N
- **WHEN** `carve_rooms` is applied
- **THEN** at most N rooms are carved (fewer if placement constraints prevent it)
