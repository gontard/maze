use crate::maze::Maze;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    Playing,
    Won,
    Lost,
    Quit,
}

pub struct GameState {
    pub player: (usize, usize),
    pub status: GameStatus,
    pub max_time_secs: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl GameState {
    #[cfg(test)]
    pub fn new(start: (usize, usize)) -> Self {
        Self::new_with_max_time(start, f64::INFINITY)
    }

    pub fn new_with_max_time(start: (usize, usize), max_time_secs: f64) -> Self {
        Self {
            player: start,
            status: GameStatus::Playing,
            max_time_secs,
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

    pub fn check_timeout(&mut self, elapsed_secs: f64) {
        if self.status == GameStatus::Playing && elapsed_secs >= self.max_time_secs {
            self.status = GameStatus::Lost;
        }
    }

    pub fn quit(&mut self) {
        self.status = GameStatus::Quit;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::Tile;

    fn test_maze() -> Maze {
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
        state.move_player(Direction::Up, &maze);
        assert_eq!(state.player, (1, 1));
    }

    #[test]
    fn move_blocked_by_boundary() {
        let maze = test_maze();
        let mut state = GameState::new(maze.start);
        state.move_player(Direction::Left, &maze);
        assert_eq!(state.player, (1, 1));
    }

    #[test]
    fn move_down_blocked_by_wall() {
        let maze = test_maze();
        let mut state = GameState::new(maze.start);
        state.move_player(Direction::Down, &maze);
        assert_eq!(state.player, (1, 1));
    }

    #[test]
    fn win_condition_on_exit() {
        let maze = test_maze();
        let mut state = GameState::new(maze.start);
        state.move_player(Direction::Right, &maze);
        state.move_player(Direction::Right, &maze);
        state.move_player(Direction::Down, &maze);
        state.move_player(Direction::Down, &maze);
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
        state.move_player(Direction::Left, &maze);
        assert_eq!(state.player, (3, 3));
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
        assert_eq!(state.player, (1, 1));
    }

    #[test]
    fn new_with_max_time_sets_fields() {
        let maze = test_maze();
        let state = GameState::new_with_max_time(maze.start, 60.0);
        assert_eq!(state.max_time_secs, 60.0);
        assert_eq!(state.status, GameStatus::Playing);
    }

    #[test]
    fn check_timeout_does_not_trigger_before_max_time() {
        let maze = test_maze();
        let mut state = GameState::new_with_max_time(maze.start, 9999.0);
        state.check_timeout(0.0);
        assert_eq!(state.status, GameStatus::Playing);
    }

    #[test]
    fn check_timeout_triggers_lost() {
        let maze = test_maze();
        let mut state = GameState::new_with_max_time(maze.start, 10.0);
        state.check_timeout(10.0);
        assert_eq!(state.status, GameStatus::Lost);
    }

    #[test]
    fn cannot_move_after_lost() {
        let maze = test_maze();
        let mut state = GameState::new_with_max_time(maze.start, 10.0);
        state.check_timeout(10.0);
        assert_eq!(state.status, GameStatus::Lost);
        state.move_player(Direction::Right, &maze);
        assert_eq!(state.player, (1, 1));
    }

    #[test]
    fn check_timeout_no_effect_after_won() {
        let maze = test_maze();
        let mut state = GameState::new_with_max_time(maze.start, 10.0);
        state.move_player(Direction::Right, &maze);
        state.move_player(Direction::Right, &maze);
        state.move_player(Direction::Down, &maze);
        state.move_player(Direction::Down, &maze);
        assert_eq!(state.status, GameStatus::Won);
        state.check_timeout(999.0);
        assert_eq!(state.status, GameStatus::Won);
    }
}
