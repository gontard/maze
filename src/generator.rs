use crate::maze::{Maze, Tile};
use rand::SeedableRng;
use rand::prelude::IndexedRandom;
use rand::rngs::StdRng;

pub trait MazeGenerator {
    fn generate(
        &self,
        width: usize,
        height: usize,
        seed: Option<u64>,
        start: Option<(usize, usize)>,
    ) -> Maze;
}

pub struct RecursiveBacktracker;

impl MazeGenerator for RecursiveBacktracker {
    fn generate(
        &self,
        width: usize,
        height: usize,
        seed: Option<u64>,
        start: Option<(usize, usize)>,
    ) -> Maze {
        // Ensure odd dimensions so walls/paths align on even/odd indices
        let width = if width % 2 == 0 { width + 1 } else { width };
        let height = if height % 2 == 0 { height + 1 } else { height };

        let (sx, sy) = start.unwrap_or((1, 1));

        let mut grid = vec![vec![Tile::Wall; width]; height];
        let mut rng = match seed {
            Some(s) => StdRng::seed_from_u64(s),
            None => StdRng::from_rng(&mut rand::rng()),
        };

        // Start carving from the start position
        grid[sy][sx] = Tile::Path;
        let mut stack = vec![(sx, sy)];

        while let Some(&(cx, cy)) = stack.last() {
            let mut neighbors = Vec::new();

            // Check 2-step neighbors (carving through walls)
            for (dx, dy) in [(0, -2i32), (0, 2), (-2i32, 0), (2, 0)] {
                let nx = cx as i32 + dx;
                let ny = cy as i32 + dy;
                if nx > 0
                    && nx < width as i32 - 1
                    && ny > 0
                    && ny < height as i32 - 1
                    && grid[ny as usize][nx as usize] == Tile::Wall
                {
                    neighbors.push((nx as usize, ny as usize));
                }
            }

            if neighbors.is_empty() {
                stack.pop();
            } else {
                let &(nx, ny) = neighbors.choose(&mut rng).unwrap();
                // Carve the wall between current and neighbor
                let wx = (cx + nx) / 2;
                let wy = (cy + ny) / 2;
                grid[wy][wx] = Tile::Path;
                grid[ny][nx] = Tile::Path;
                stack.push((nx, ny));
            }
        }

        // Place start tile
        grid[sy][sx] = Tile::Start;

        Maze {
            grid,
            width,
            height,
            start: (sx, sy),
            exit: (sx, sy),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn generates_maze_with_correct_dimensions() {
        let generator = RecursiveBacktracker;
        let maze = generator.generate(21, 21, Some(42), None);
        assert_eq!(maze.width, 21);
        assert_eq!(maze.height, 21);
        assert_eq!(maze.grid.len(), 21);
        assert_eq!(maze.grid[0].len(), 21);
    }

    #[test]
    fn has_exactly_one_start_and_one_exit() {
        let generator = RecursiveBacktracker;
        let mut maze = generator.generate(21, 21, Some(42), None);
        maze.place_exit();
        let mut starts = 0;
        let mut exits = 0;
        for row in &maze.grid {
            for tile in row {
                match tile {
                    Tile::Start => starts += 1,
                    Tile::Exit => exits += 1,
                    _ => {}
                }
            }
        }
        assert_eq!(starts, 1);
        assert_eq!(exits, 1);
    }

    #[test]
    fn start_is_at_1_1() {
        let generator = RecursiveBacktracker;
        let maze = generator.generate(21, 21, Some(42), None);
        assert_eq!(maze.start, (1, 1));
        assert_eq!(maze.grid[1][1], Tile::Start);
    }

    #[test]
    fn exit_is_farthest_from_start() {
        let generator = RecursiveBacktracker;
        let mut maze = generator.generate(21, 21, Some(42), None);
        maze.place_exit();
        // BFS from start to find the actual farthest point
        let distances = bfs_distances(&maze, maze.start);
        let exit_dist = distances[maze.exit.1][maze.exit.0].unwrap();
        // No traversable cell should be farther than the exit
        for y in 0..maze.height {
            for x in 0..maze.width {
                if let Some(d) = distances[y][x] {
                    assert!(
                        d <= exit_dist,
                        "cell ({x}, {y}) at distance {d} is farther than exit at distance {exit_dist}"
                    );
                }
            }
        }
    }

    #[test]
    fn maze_is_solvable() {
        let generator = RecursiveBacktracker;
        let mut maze = generator.generate(21, 21, Some(42), None);
        maze.place_exit();
        assert!(
            bfs_reachable(&maze, maze.start, maze.exit),
            "exit must be reachable from start"
        );
    }

    #[test]
    fn all_path_cells_reachable() {
        let generator = RecursiveBacktracker;
        let mut maze = generator.generate(21, 21, Some(42), None);
        maze.place_exit();
        let reachable = bfs_all_reachable(&maze, maze.start);
        for y in 0..maze.height {
            for x in 0..maze.width {
                if matches!(maze.grid[y][x], Tile::Path | Tile::Start | Tile::Exit) {
                    assert!(
                        reachable[y][x],
                        "cell ({x}, {y}) should be reachable from start"
                    );
                }
            }
        }
    }

    #[test]
    fn same_seed_produces_identical_maze() {
        let generator = RecursiveBacktracker;
        let maze1 = generator.generate(21, 21, Some(123), None);
        let maze2 = generator.generate(21, 21, Some(123), None);
        assert_eq!(maze1.grid, maze2.grid);
        assert_eq!(maze1.start, maze2.start);
        assert_eq!(maze1.exit, maze2.exit);
    }

    #[test]
    fn different_seeds_produce_different_mazes() {
        let generator = RecursiveBacktracker;
        let maze1 = generator.generate(21, 21, Some(1), None);
        let maze2 = generator.generate(21, 21, Some(2), None);
        assert_ne!(maze1.grid, maze2.grid);
    }

    #[test]
    fn even_dimensions_rounded_to_odd() {
        let generator = RecursiveBacktracker;
        let maze = generator.generate(20, 20, Some(42), None);
        assert_eq!(maze.width, 21);
        assert_eq!(maze.height, 21);
    }

    #[test]
    fn outer_border_is_all_walls() {
        let generator = RecursiveBacktracker;
        let maze = generator.generate(21, 21, Some(42), None);
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
    fn custom_start_position() {
        let generator = RecursiveBacktracker;
        let maze = generator.generate(21, 21, Some(42), Some((5, 5)));
        assert_eq!(maze.start, (5, 5));
        assert_eq!(maze.grid[5][5], Tile::Start);
    }

    #[test]
    fn custom_start_maze_is_solvable() {
        let generator = RecursiveBacktracker;
        let mut maze = generator.generate(21, 21, Some(42), Some((5, 5)));
        maze.place_exit();
        assert!(
            bfs_reachable(&maze, maze.start, maze.exit),
            "exit must be reachable from custom start"
        );
    }

    #[test]
    fn custom_start_all_cells_reachable() {
        let generator = RecursiveBacktracker;
        let mut maze = generator.generate(21, 21, Some(42), Some((5, 5)));
        maze.place_exit();
        let reachable = bfs_all_reachable(&maze, maze.start);
        for y in 0..maze.height {
            for x in 0..maze.width {
                if matches!(maze.grid[y][x], Tile::Path | Tile::Start | Tile::Exit) {
                    assert!(
                        reachable[y][x],
                        "cell ({x}, {y}) should be reachable from custom start"
                    );
                }
            }
        }
    }

    #[test]
    fn custom_start_exit_is_farthest() {
        let generator = RecursiveBacktracker;
        let mut maze = generator.generate(21, 21, Some(42), Some((5, 5)));
        maze.place_exit();
        let distances = bfs_distances(&maze, maze.start);
        let exit_dist = distances[maze.exit.1][maze.exit.0].unwrap();
        for y in 0..maze.height {
            for x in 0..maze.width {
                if let Some(d) = distances[y][x] {
                    assert!(
                        d <= exit_dist,
                        "cell ({x}, {y}) at distance {d} is farther than exit at distance {exit_dist}"
                    );
                }
            }
        }
    }

    #[test]
    fn same_seed_and_start_produces_identical_maze() {
        let generator = RecursiveBacktracker;
        let maze1 = generator.generate(21, 21, Some(123), Some((3, 7)));
        let maze2 = generator.generate(21, 21, Some(123), Some((3, 7)));
        assert_eq!(maze1.grid, maze2.grid);
        assert_eq!(maze1.start, maze2.start);
        assert_eq!(maze1.exit, maze2.exit);
    }

    // BFS helpers for tests
    fn bfs_reachable(maze: &Maze, from: (usize, usize), to: (usize, usize)) -> bool {
        let reachable = bfs_all_reachable(maze, from);
        reachable[to.1][to.0]
    }

    fn bfs_all_reachable(maze: &Maze, from: (usize, usize)) -> Vec<Vec<bool>> {
        let mut visited = vec![vec![false; maze.width]; maze.height];
        let mut queue = VecDeque::new();
        visited[from.1][from.0] = true;
        queue.push_back(from);

        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in [(0i32, -1i32), (0, 1), (-1, 0), (1, 0)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && ny >= 0 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if nx < maze.width
                        && ny < maze.height
                        && !visited[ny][nx]
                        && maze.is_traversable(nx, ny)
                    {
                        visited[ny][nx] = true;
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
        visited
    }

    fn bfs_distances(maze: &Maze, from: (usize, usize)) -> Vec<Vec<Option<usize>>> {
        let mut distances = vec![vec![None; maze.width]; maze.height];
        let mut queue = VecDeque::new();
        distances[from.1][from.0] = Some(0);
        queue.push_back(from);

        while let Some((x, y)) = queue.pop_front() {
            let dist = distances[y][x].unwrap();
            for (dx, dy) in [(0i32, -1i32), (0, 1), (-1, 0), (1, 0)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && ny >= 0 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if nx < maze.width
                        && ny < maze.height
                        && distances[ny][nx].is_none()
                        && maze.is_traversable(nx, ny)
                    {
                        distances[ny][nx] = Some(dist + 1);
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
        distances
    }
}
