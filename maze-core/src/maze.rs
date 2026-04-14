#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Path,
    Start,
    Exit,
}

pub struct Maze {
    pub grid: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
    pub start: (usize, usize),
    pub exit: (usize, usize),
}

impl Maze {
    pub fn tile_at(&self, x: usize, y: usize) -> Option<Tile> {
        if x < self.width && y < self.height {
            Some(self.grid[y][x])
        } else {
            None
        }
    }

    pub fn is_traversable(&self, x: usize, y: usize) -> bool {
        matches!(
            self.tile_at(x, y),
            Some(Tile::Path | Tile::Start | Tile::Exit)
        )
    }

    pub fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        const DELTAS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        DELTAS.into_iter().filter_map(move |(dx, dy)| {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && ny >= 0 {
                let (nx, ny) = (nx as usize, ny as usize);
                if self.is_traversable(nx, ny) {
                    return Some((nx, ny));
                }
            }
            None
        })
    }

    pub fn place_exit(&mut self) {
        use std::collections::VecDeque;

        let mut visited = vec![vec![false; self.width]; self.height];
        let mut queue = VecDeque::new();
        let (sx, sy) = self.start;
        visited[sy][sx] = true;
        queue.push_back((sx, sy));
        let mut farthest = self.start;

        while let Some((x, y)) = queue.pop_front() {
            farthest = (x, y);
            for (nx, ny) in self.neighbors(x, y) {
                if !visited[ny][nx] {
                    visited[ny][nx] = true;
                    queue.push_back((nx, ny));
                }
            }
        }

        let (ex, ey) = farthest;
        self.grid[ey][ex] = Tile::Exit;
        self.exit = (ex, ey);
    }

    pub fn carve_rooms(
        &mut self,
        count: usize,
        min_size: usize,
        max_size: usize,
        rng: &mut impl rand::Rng,
    ) {
        use rand::RngExt;

        for _ in 0..count {
            // Pick random odd-aligned room dimensions
            let room_w = {
                let half_min = min_size / 2;
                let half_max = max_size / 2;
                rng.random_range(half_min..=half_max) * 2 + 1
            };
            let room_h = {
                let half_min = min_size / 2;
                let half_max = max_size / 2;
                rng.random_range(half_min..=half_max) * 2 + 1
            };

            // Pick random odd-aligned top-left corner within border
            let max_x = (self.width - 1 - room_w) / 2;
            let max_y = (self.height - 1 - room_h) / 2;
            if max_x < 1 || max_y < 1 {
                continue;
            }
            let room_x = rng.random_range(1..=max_x) * 2 - 1;
            let room_y = rng.random_range(1..=max_y) * 2 - 1;

            // Carve: only convert Wall to Path
            for y in room_y..room_y + room_h {
                for x in room_x..room_x + room_w {
                    if x > 0
                        && x < self.width - 1
                        && y > 0
                        && y < self.height - 1
                        && self.grid[y][x] == Tile::Wall
                    {
                        self.grid[y][x] = Tile::Path;
                    }
                }
            }
        }
    }

    pub fn solve(&self) -> Option<usize> {
        use std::collections::VecDeque;

        let mut visited = vec![vec![false; self.width]; self.height];
        let mut queue = VecDeque::new();
        let (sx, sy) = self.start;
        visited[sy][sx] = true;
        queue.push_back((sx, sy, 0usize));

        while let Some((x, y, dist)) = queue.pop_front() {
            if (x, y) == self.exit {
                return Some(dist);
            }
            for (nx, ny) in self.neighbors(x, y) {
                if !visited[ny][nx] {
                    visited[ny][nx] = true;
                    queue.push_back((nx, ny, dist + 1));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::{MazeGenerator, RecursiveBacktracker};
    use rand::SeedableRng;
    use rand::rngs::StdRng;
    use std::collections::VecDeque;

    fn sample_maze() -> Maze {
        // 3x3 maze:
        // W W W
        // W S W
        // W E W
        let grid = vec![
            vec![Tile::Wall, Tile::Wall, Tile::Wall],
            vec![Tile::Wall, Tile::Start, Tile::Wall],
            vec![Tile::Wall, Tile::Exit, Tile::Wall],
        ];
        Maze {
            grid,
            width: 3,
            height: 3,
            start: (1, 1),
            exit: (1, 2),
        }
    }

    #[test]
    fn tile_equality() {
        assert_eq!(Tile::Wall, Tile::Wall);
        assert_ne!(Tile::Wall, Tile::Path);
    }

    #[test]
    fn maze_stores_dimensions() {
        let maze = sample_maze();
        assert_eq!(maze.width, 3);
        assert_eq!(maze.height, 3);
        assert_eq!(maze.start, (1, 1));
        assert_eq!(maze.exit, (1, 2));
    }

    #[test]
    fn tile_at_returns_correct_tile() {
        let maze = sample_maze();
        assert_eq!(maze.tile_at(0, 0), Some(Tile::Wall));
        assert_eq!(maze.tile_at(1, 1), Some(Tile::Start));
        assert_eq!(maze.tile_at(1, 2), Some(Tile::Exit));
    }

    #[test]
    fn tile_at_out_of_bounds_returns_none() {
        let maze = sample_maze();
        assert_eq!(maze.tile_at(3, 0), None);
        assert_eq!(maze.tile_at(0, 3), None);
        assert_eq!(maze.tile_at(99, 99), None);
    }

    #[test]
    fn is_traversable_for_path_start_exit() {
        let maze = sample_maze();
        assert!(maze.is_traversable(1, 1)); // Start
        assert!(maze.is_traversable(1, 2)); // Exit
    }

    #[test]
    fn is_traversable_false_for_wall() {
        let maze = sample_maze();
        assert!(!maze.is_traversable(0, 0)); // Wall
    }

    #[test]
    fn is_traversable_false_for_out_of_bounds() {
        let maze = sample_maze();
        assert!(!maze.is_traversable(99, 99));
    }

    #[test]
    fn neighbors_interior_cell_all_traversable() {
        // 5x5 maze with open center
        let grid = vec![
            vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
            vec![Tile::Wall, Tile::Path, Tile::Path, Tile::Path, Tile::Wall],
            vec![Tile::Wall, Tile::Path, Tile::Start, Tile::Path, Tile::Wall],
            vec![Tile::Wall, Tile::Path, Tile::Path, Tile::Path, Tile::Wall],
            vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
        ];
        let maze = Maze {
            grid,
            width: 5,
            height: 5,
            start: (2, 2),
            exit: (2, 2),
        };
        let mut ns: Vec<_> = maze.neighbors(2, 2).collect();
        ns.sort();
        assert_eq!(ns, vec![(1, 2), (2, 1), (2, 3), (3, 2)]);
    }

    #[test]
    fn neighbors_cell_adjacent_to_walls() {
        // sample_maze: Start at (1,1) has only Exit at (1,2) as neighbor
        let maze = sample_maze();
        let ns: Vec<_> = maze.neighbors(1, 1).collect();
        assert_eq!(ns, vec![(1, 2)]);
    }

    #[test]
    fn neighbors_corner_cell() {
        let maze = sample_maze();
        // (0,0) is a wall, but even if we ask neighbors of it, they're all walls
        let ns: Vec<_> = maze.neighbors(0, 0).collect();
        assert!(ns.is_empty());
    }

    #[test]
    fn neighbors_out_of_bounds() {
        let maze = sample_maze();
        let ns: Vec<_> = maze.neighbors(99, 99).collect();
        assert!(ns.is_empty());
    }

    #[test]
    fn solve_adjacent_start_exit() {
        // sample_maze has start=(1,1) and exit=(1,2), adjacent
        let maze = sample_maze();
        assert_eq!(maze.solve(), Some(1));
    }

    #[test]
    fn solve_longer_path() {
        // 5x5 maze with a 4-step shortest path:
        // W W W W W
        // W S P P W
        // W W W P W
        // W P P E W
        // W W W W W
        let grid = vec![
            vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
            vec![Tile::Wall, Tile::Start, Tile::Path, Tile::Path, Tile::Wall],
            vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Path, Tile::Wall],
            vec![Tile::Wall, Tile::Path, Tile::Path, Tile::Exit, Tile::Wall],
            vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
        ];
        let maze = Maze {
            grid,
            width: 5,
            height: 5,
            start: (1, 1),
            exit: (3, 3),
        };
        // Shortest path: (1,1)->(2,1)->(3,1)->(3,2)->(3,3) = 4 steps
        assert_eq!(maze.solve(), Some(4));
    }

    fn generated_maze(seed: u64) -> Maze {
        let generator = RecursiveBacktracker;
        generator.generate(21, 21, Some(seed), None)
    }

    fn bfs_all_reachable(maze: &Maze, from: (usize, usize)) -> Vec<Vec<bool>> {
        let mut visited = vec![vec![false; maze.width]; maze.height];
        let mut queue = VecDeque::new();
        visited[from.1][from.0] = true;
        queue.push_back(from);

        while let Some((x, y)) = queue.pop_front() {
            for (nx, ny) in maze.neighbors(x, y) {
                if !visited[ny][nx] {
                    visited[ny][nx] = true;
                    queue.push_back((nx, ny));
                }
            }
        }
        visited
    }

    #[test]
    fn carve_rooms_only_converts_walls_to_paths() {
        let mut maze = generated_maze(42);
        let before: Vec<Vec<Tile>> = maze.grid.clone();
        let mut rng = StdRng::seed_from_u64(99);
        maze.carve_rooms(5, 3, 5, &mut rng);

        for y in 0..maze.height {
            for x in 0..maze.width {
                let old = before[y][x];
                let new = maze.grid[y][x];
                match old {
                    Tile::Wall => assert!(
                        new == Tile::Wall || new == Tile::Path,
                        "wall at ({x}, {y}) became {new:?}"
                    ),
                    Tile::Path | Tile::Start => assert_eq!(
                        old, new,
                        "non-wall tile at ({x}, {y}) changed from {old:?} to {new:?}"
                    ),
                    Tile::Exit => {} // no exit placed yet
                }
            }
        }
    }

    #[test]
    fn carve_rooms_preserves_border() {
        let mut maze = generated_maze(42);
        let mut rng = StdRng::seed_from_u64(99);
        maze.carve_rooms(5, 3, 5, &mut rng);

        for x in 0..maze.width {
            assert_eq!(maze.grid[0][x], Tile::Wall, "top border at x={x}");
            assert_eq!(
                maze.grid[maze.height - 1][x],
                Tile::Wall,
                "bottom border at x={x}"
            );
        }
        for y in 0..maze.height {
            assert_eq!(maze.grid[y][0], Tile::Wall, "left border at y={y}");
            assert_eq!(
                maze.grid[y][maze.width - 1],
                Tile::Wall,
                "right border at y={y}"
            );
        }
    }

    #[test]
    fn carve_rooms_maze_remains_solvable() {
        let mut maze = generated_maze(42);
        let mut rng = StdRng::seed_from_u64(99);
        maze.carve_rooms(5, 3, 5, &mut rng);
        maze.place_exit();
        assert!(
            maze.solve().is_some(),
            "maze must be solvable after room carving"
        );
    }

    #[test]
    fn carve_rooms_all_cells_reachable() {
        let mut maze = generated_maze(42);
        let mut rng = StdRng::seed_from_u64(99);
        maze.carve_rooms(5, 3, 5, &mut rng);
        maze.place_exit();
        let reachable = bfs_all_reachable(&maze, maze.start);
        for y in 0..maze.height {
            for x in 0..maze.width {
                if matches!(maze.grid[y][x], Tile::Path | Tile::Start | Tile::Exit) {
                    assert!(
                        reachable[y][x],
                        "cell ({x}, {y}) should be reachable after room carving"
                    );
                }
            }
        }
    }

    #[test]
    fn carve_rooms_deterministic_with_same_seed() {
        let mut maze1 = generated_maze(42);
        let mut rng1 = StdRng::seed_from_u64(99);
        maze1.carve_rooms(5, 3, 5, &mut rng1);

        let mut maze2 = generated_maze(42);
        let mut rng2 = StdRng::seed_from_u64(99);
        maze2.carve_rooms(5, 3, 5, &mut rng2);

        assert_eq!(maze1.grid, maze2.grid);
    }

    #[test]
    fn carve_rooms_changes_some_walls_to_paths() {
        let mut maze = generated_maze(42);
        let walls_before: usize = maze
            .grid
            .iter()
            .flatten()
            .filter(|t| **t == Tile::Wall)
            .count();
        let mut rng = StdRng::seed_from_u64(99);
        maze.carve_rooms(5, 3, 5, &mut rng);
        let walls_after: usize = maze
            .grid
            .iter()
            .flatten()
            .filter(|t| **t == Tile::Wall)
            .count();
        assert!(
            walls_after < walls_before,
            "room carving should remove some walls"
        );
    }

    #[test]
    fn carve_rooms_zero_count_changes_nothing() {
        let mut maze = generated_maze(42);
        let before = maze.grid.clone();
        let mut rng = StdRng::seed_from_u64(99);
        maze.carve_rooms(0, 3, 5, &mut rng);
        assert_eq!(maze.grid, before);
    }

    #[test]
    fn solve_unsolvable_maze() {
        // Start and exit completely walled off from each other
        let grid = vec![
            vec![Tile::Wall, Tile::Wall, Tile::Wall],
            vec![Tile::Wall, Tile::Start, Tile::Wall],
            vec![Tile::Wall, Tile::Wall, Tile::Wall],
            vec![Tile::Wall, Tile::Exit, Tile::Wall],
            vec![Tile::Wall, Tile::Wall, Tile::Wall],
        ];
        let maze = Maze {
            grid,
            width: 3,
            height: 5,
            start: (1, 1),
            exit: (1, 3),
        };
        assert_eq!(maze.solve(), None);
    }
}
