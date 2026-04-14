## Requirements

### Requirement: Neighbors iterator
The `Maze` struct SHALL provide a `neighbors(x, y)` method that returns an iterator over `(usize, usize)` coordinates of traversable adjacent cells (up, down, left, right).

#### Scenario: Interior cell with all traversable neighbors
- **WHEN** `neighbors(x, y)` is called on a cell surrounded by 4 traversable cells
- **THEN** the iterator yields exactly 4 coordinates

#### Scenario: Cell adjacent to walls
- **WHEN** `neighbors(x, y)` is called on a cell where 2 neighbors are walls
- **THEN** the iterator yields only the 2 traversable neighbor coordinates

#### Scenario: Corner cell at grid boundary
- **WHEN** `neighbors(0, 0)` is called
- **THEN** only in-bounds traversable neighbors are yielded (at most 2)

#### Scenario: Out-of-bounds input
- **WHEN** `neighbors(x, y)` is called with coordinates outside the grid
- **THEN** the iterator yields zero items

### Requirement: BFS methods use neighbors iterator
`Maze::place_exit()` and `Maze::solve()` SHALL use `self.neighbors(x, y)` instead of inline neighbor enumeration.

#### Scenario: place_exit uses neighbors
- **WHEN** `place_exit()` traverses the maze
- **THEN** it uses `self.neighbors()` for adjacency

#### Scenario: solve uses neighbors
- **WHEN** `solve()` traverses the maze
- **THEN** it uses `self.neighbors()` for adjacency
