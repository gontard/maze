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
            for (dx, dy) in [(0i32, -1i32), (0, 1), (-1, 0), (1, 0)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && ny >= 0 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if nx < self.width
                        && ny < self.height
                        && !visited[ny][nx]
                        && matches!(self.grid[ny][nx], Tile::Path | Tile::Start)
                    {
                        visited[ny][nx] = true;
                        queue.push_back((nx, ny));
                    }
                }
            }
        }

        let (ex, ey) = farthest;
        self.grid[ey][ex] = Tile::Exit;
        self.exit = (ex, ey);
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
            for (dx, dy) in [(0i32, -1i32), (0, 1), (-1, 0), (1, 0)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && ny >= 0 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if nx < self.width
                        && ny < self.height
                        && !visited[ny][nx]
                        && self.is_traversable(nx, ny)
                    {
                        visited[ny][nx] = true;
                        queue.push_back((nx, ny, dist + 1));
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
