## ADDED Requirements

### Requirement: Floor generation function
The system SHALL provide a `generate_floor` function in maze-core that randomly selects a maze algorithm, generates a maze, carves rooms, places the exit, and computes the time budget.

#### Scenario: Generate a floor with no prior position
- **WHEN** `generate_floor` is called with `start_pos: None`
- **THEN** a maze is returned with start at (1, 1), rooms carved, exit placed, and a positive time budget

#### Scenario: Generate a floor with a prior exit position
- **WHEN** `generate_floor` is called with `start_pos: Some((ex, ey))`
- **THEN** the returned maze has its start at `(ex, ey)`

#### Scenario: Time budget is derived from solution path length
- **WHEN** `generate_floor` returns `(maze, max_time_secs)`
- **THEN** `max_time_secs` equals `maze.solve().unwrap() as f64 * 0.375`

#### Scenario: Deterministic with same RNG state
- **WHEN** `generate_floor` is called twice with identically-seeded RNGs and the same start_pos
- **THEN** both calls return identical mazes and time budgets

### Requirement: Frontends delegate to shared floor generation
maze-terminal and maze-web SHALL call `maze_core::floor::generate_floor` instead of duplicating algorithm selection, room carving, and time budget logic.

#### Scenario: Terminal uses shared generation
- **WHEN** maze-terminal generates a new floor
- **THEN** it calls `generate_floor` from maze-core

#### Scenario: Web uses shared generation
- **WHEN** maze-web generates a new floor
- **THEN** it calls `generate_floor` from maze-core
