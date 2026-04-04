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
}
