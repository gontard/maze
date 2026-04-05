## ADDED Requirements

### Requirement: Shortest path solver
The system SHALL provide a method on `Maze` that computes the shortest path length from start to exit using BFS.

#### Scenario: Solvable maze
- **WHEN** `solve()` is called on a maze with a valid path from start to exit
- **THEN** it returns `Some(n)` where `n` is the number of steps in the shortest path

#### Scenario: Trivially short maze
- **WHEN** `solve()` is called on a maze where start is adjacent to exit
- **THEN** it returns `Some(1)`

#### Scenario: Unsolvable maze
- **WHEN** `solve()` is called on a maze with no path from start to exit
- **THEN** it returns `None`
