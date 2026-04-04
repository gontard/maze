use std::time::Instant;

use crate::maze::Maze;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    Playing,
    Won,
    Quit,
}

pub struct GameState {
    pub player: (usize, usize),
    pub status: GameStatus,
    pub start_time: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl GameState {
    pub fn new(start: (usize, usize)) -> Self {
        Self {
            player: start,
            status: GameStatus::Playing,
            start_time: Instant::now(),
        }
    }

    pub fn move_player(&mut self, direction: Direction, maze: &Maze) {
        if self.status != GameStatus::Playing {
            return;
        }

        let (x, y) = self.player;
        let (nx, ny) = match direction {
            Direction::Up if y > 0 => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left if x > 0 => (x - 1, y),
            Direction::Right => (x + 1, y),
            _ => return,
        };

        if maze.is_traversable(nx, ny) {
            self.player = (nx, ny);
            if self.player == maze.exit {
                self.status = GameStatus::Won;
            }
        }
    }

    pub fn quit(&mut self) {
        self.status = GameStatus::Quit;
    }

    pub fn elapsed_secs(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::Tile;

    fn test_maze() -> Maze {
        // 5x5 maze with a simple path:
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
        Maze {
            grid,
            width: 5,
            height: 5,
            start: (1, 1),
            exit: (3, 3),
        }
    }

    #[test]
    fn initial_position_is_start() {
        let maze = test_maze();
        let state = GameState::new(maze.start);
        assert_eq!(state.player, (1, 1));
        assert_eq!(state.status, GameStatus::Playing);
    }

    #[test]
    fn move_to_open_cell() {
        let maze = test_maze();
        let mut state = GameState::new(maze.start);
        state.move_player(Direction::Right, &maze);
        assert_eq!(state.player, (2, 1));
    }

    #[test]
    fn move_blocked_by_wall() {
        let maze = test_maze();
        let mut state = GameState::new(maze.start);
        // Up from (1,1) is wall at (1,0)
        state.move_player(Direction::Up, &maze);
        assert_eq!(state.player, (1, 1));
    }

    #[test]
    fn move_blocked_by_boundary() {
        let maze = test_maze();
        let mut state = GameState::new(maze.start);
        // Left from (1,1) is wall at (0,1) — but also test boundary
        state.move_player(Direction::Left, &maze);
        assert_eq!(state.player, (1, 1)); // Wall blocks it
    }

    #[test]
    fn move_down_blocked_by_wall() {
        let maze = test_maze();
        let mut state = GameState::new(maze.start);
        // Down from (1,1) is wall at (1,2)
        state.move_player(Direction::Down, &maze);
        assert_eq!(state.player, (1, 1));
    }

    #[test]
    fn win_condition_on_exit() {
        let maze = test_maze();
        let mut state = GameState::new(maze.start);
        // Navigate: right, right, right (to 3,1), down (to 3,2), down (to 3,3 = exit)
        // Wait — (3,1) is Path, (3,2) is Path, (3,3) is Exit
        state.move_player(Direction::Right, &maze); // (2,1)
        state.move_player(Direction::Right, &maze); // (3,1)
        state.move_player(Direction::Down, &maze); // (3,2)
        state.move_player(Direction::Down, &maze); // (3,3) = exit
        assert_eq!(state.player, (3, 3));
        assert_eq!(state.status, GameStatus::Won);
    }

    #[test]
    fn cannot_move_after_win() {
        let maze = test_maze();
        let mut state = GameState::new(maze.start);
        state.move_player(Direction::Right, &maze);
        state.move_player(Direction::Right, &maze);
        state.move_player(Direction::Down, &maze);
        state.move_player(Direction::Down, &maze);
        assert_eq!(state.status, GameStatus::Won);

        // Try to move after winning
        state.move_player(Direction::Left, &maze);
        assert_eq!(state.player, (3, 3)); // Still at exit
    }

    #[test]
    fn quit_sets_status() {
        let maze = test_maze();
        let mut state = GameState::new(maze.start);
        state.quit();
        assert_eq!(state.status, GameStatus::Quit);
    }

    #[test]
    fn cannot_move_after_quit() {
        let maze = test_maze();
        let mut state = GameState::new(maze.start);
        state.quit();
        state.move_player(Direction::Right, &maze);
        assert_eq!(state.player, (1, 1)); // Didn't move
    }
}
