use crate::maze::{Maze, Tile};
use rand::prelude::IndexedRandom;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{RngExt, SeedableRng};

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
        let (width, height, (sx, sy), mut grid, mut rng) = init_maze(width, height, seed, start);

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

        finish_maze(grid, width, height, (sx, sy))
    }
}

fn init_maze(
    width: usize,
    height: usize,
    seed: Option<u64>,
    start: Option<(usize, usize)>,
) -> (usize, usize, (usize, usize), Vec<Vec<Tile>>, StdRng) {
    let width = if width % 2 == 0 { width + 1 } else { width };
    let height = if height % 2 == 0 { height + 1 } else { height };
    let (sx, sy) = start.unwrap_or((1, 1));
    let grid = vec![vec![Tile::Wall; width]; height];
    let rng = match seed {
        Some(s) => StdRng::seed_from_u64(s),
        None => StdRng::from_rng(&mut rand::rng()),
    };
    (width, height, (sx, sy), grid, rng)
}

fn finish_maze(
    mut grid: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    start: (usize, usize),
) -> Maze {
    grid[start.1][start.0] = Tile::Start;
    Maze {
        grid,
        width,
        height,
        start,
        exit: start,
    }
}

// --- Union-Find for Kruskal's ---

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return false;
        }
        if self.rank[ra] < self.rank[rb] {
            self.parent[ra] = rb;
        } else if self.rank[ra] > self.rank[rb] {
            self.parent[rb] = ra;
        } else {
            self.parent[rb] = ra;
            self.rank[ra] += 1;
        }
        true
    }
}

// --- Kruskal's Algorithm ---

pub struct Kruskal;

impl MazeGenerator for Kruskal {
    fn generate(
        &self,
        width: usize,
        height: usize,
        seed: Option<u64>,
        start: Option<(usize, usize)>,
    ) -> Maze {
        let (width, height, (sx, sy), mut grid, mut rng) = init_maze(width, height, seed, start);

        // Number of cells in the odd-aligned grid
        let cols = width / 2;
        let rows = height / 2;
        let cell_id = |cx: usize, cy: usize| cy * cols + cx;

        let mut uf = UnionFind::new(rows * cols);

        // Collect all internal walls between adjacent cells
        let mut walls: Vec<((usize, usize), (usize, usize))> = Vec::new();
        for cy in 0..rows {
            for cx in 0..cols {
                if cx + 1 < cols {
                    walls.push(((cx, cy), (cx + 1, cy)));
                }
                if cy + 1 < rows {
                    walls.push(((cx, cy), (cx, cy + 1)));
                }
            }
        }

        walls.shuffle(&mut rng);

        for ((ax, ay), (bx, by)) in walls {
            if uf.union(cell_id(ax, ay), cell_id(bx, by)) {
                // Carve both cells and the wall between them
                let gax = ax * 2 + 1;
                let gay = ay * 2 + 1;
                let gbx = bx * 2 + 1;
                let gby = by * 2 + 1;
                grid[gay][gax] = Tile::Path;
                grid[gby][gbx] = Tile::Path;
                grid[(gay + gby) / 2][(gax + gbx) / 2] = Tile::Path;
            }
        }

        finish_maze(grid, width, height, (sx, sy))
    }
}

// --- Prim's Algorithm ---

pub struct Prim;

impl MazeGenerator for Prim {
    fn generate(
        &self,
        width: usize,
        height: usize,
        seed: Option<u64>,
        start: Option<(usize, usize)>,
    ) -> Maze {
        let (width, height, (sx, sy), mut grid, mut rng) = init_maze(width, height, seed, start);

        let cols = width / 2;
        let rows = height / 2;

        // Convert start to cell coordinates
        let start_cx = (sx - 1) / 2;
        let start_cy = (sy - 1) / 2;

        let mut visited = vec![vec![false; cols]; rows];
        visited[start_cy][start_cx] = true;
        grid[start_cy * 2 + 1][start_cx * 2 + 1] = Tile::Path;

        // Frontier: walls between a visited and unvisited cell
        // Stored as (cell_a, cell_b) where cell_a is visited
        let mut frontier: Vec<((usize, usize), (usize, usize))> = Vec::new();

        let add_frontiers =
            |cx: usize,
             cy: usize,
             visited: &Vec<Vec<bool>>,
             frontier: &mut Vec<((usize, usize), (usize, usize))>| {
                for (dx, dy) in [(0i32, -1i32), (0, 1), (-1, 0), (1, 0)] {
                    let nx = cx as i32 + dx;
                    let ny = cy as i32 + dy;
                    if nx >= 0
                        && ny >= 0
                        && (nx as usize) < cols
                        && (ny as usize) < rows
                        && !visited[ny as usize][nx as usize]
                    {
                        frontier.push(((cx, cy), (nx as usize, ny as usize)));
                    }
                }
            };

        add_frontiers(start_cx, start_cy, &visited, &mut frontier);

        while !frontier.is_empty() {
            let idx = rng.random_range(0..frontier.len());
            let ((_, _), (bx, by)) = frontier.swap_remove(idx);

            if visited[by][bx] {
                continue;
            }

            // Find a visited neighbor to connect through
            let mut connect_from = None;
            for (dx, dy) in [(0i32, -1i32), (0, 1), (-1, 0), (1, 0)] {
                let nx = bx as i32 + dx;
                let ny = by as i32 + dy;
                if nx >= 0
                    && ny >= 0
                    && (nx as usize) < cols
                    && (ny as usize) < rows
                    && visited[ny as usize][nx as usize]
                {
                    connect_from = Some((nx as usize, ny as usize));
                    break;
                }
            }

            if let Some((ax, ay)) = connect_from {
                visited[by][bx] = true;
                let gax = ax * 2 + 1;
                let gay = ay * 2 + 1;
                let gbx = bx * 2 + 1;
                let gby = by * 2 + 1;
                grid[gby][gbx] = Tile::Path;
                grid[(gay + gby) / 2][(gax + gbx) / 2] = Tile::Path;

                add_frontiers(bx, by, &visited, &mut frontier);
            }
        }

        finish_maze(grid, width, height, (sx, sy))
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

    // --- Kruskal tests ---

    #[test]
    fn kruskal_generates_maze_with_correct_dimensions() {
        let generator = Kruskal;
        let maze = generator.generate(21, 21, Some(42), None);
        assert_eq!(maze.width, 21);
        assert_eq!(maze.height, 21);
    }

    #[test]
    fn kruskal_all_path_cells_reachable() {
        let generator = Kruskal;
        let mut maze = generator.generate(21, 21, Some(42), None);
        maze.place_exit();
        let reachable = bfs_all_reachable(&maze, maze.start);
        for y in 0..maze.height {
            for x in 0..maze.width {
                if matches!(maze.grid[y][x], Tile::Path | Tile::Start | Tile::Exit) {
                    assert!(
                        reachable[y][x],
                        "kruskal: cell ({x}, {y}) should be reachable from start"
                    );
                }
            }
        }
    }

    #[test]
    fn kruskal_maze_is_solvable() {
        let generator = Kruskal;
        let mut maze = generator.generate(21, 21, Some(42), None);
        maze.place_exit();
        assert!(
            bfs_reachable(&maze, maze.start, maze.exit),
            "kruskal: exit must be reachable from start"
        );
    }

    #[test]
    fn kruskal_outer_border_is_all_walls() {
        let generator = Kruskal;
        let maze = generator.generate(21, 21, Some(42), None);
        for x in 0..maze.width {
            assert_eq!(maze.grid[0][x], Tile::Wall);
            assert_eq!(maze.grid[maze.height - 1][x], Tile::Wall);
        }
        for y in 0..maze.height {
            assert_eq!(maze.grid[y][0], Tile::Wall);
            assert_eq!(maze.grid[y][maze.width - 1], Tile::Wall);
        }
    }

    #[test]
    fn kruskal_same_seed_produces_identical_maze() {
        let generator = Kruskal;
        let maze1 = generator.generate(21, 21, Some(123), None);
        let maze2 = generator.generate(21, 21, Some(123), None);
        assert_eq!(maze1.grid, maze2.grid);
    }

    #[test]
    fn kruskal_custom_start_position() {
        let generator = Kruskal;
        let maze = generator.generate(21, 21, Some(42), Some((5, 5)));
        assert_eq!(maze.start, (5, 5));
        assert_eq!(maze.grid[5][5], Tile::Start);
    }

    #[test]
    fn kruskal_custom_start_all_cells_reachable() {
        let generator = Kruskal;
        let mut maze = generator.generate(21, 21, Some(42), Some((5, 5)));
        maze.place_exit();
        let reachable = bfs_all_reachable(&maze, maze.start);
        for y in 0..maze.height {
            for x in 0..maze.width {
                if matches!(maze.grid[y][x], Tile::Path | Tile::Start | Tile::Exit) {
                    assert!(
                        reachable[y][x],
                        "kruskal: cell ({x}, {y}) should be reachable from custom start"
                    );
                }
            }
        }
    }

    // --- Prim tests ---

    #[test]
    fn prim_generates_maze_with_correct_dimensions() {
        let generator = Prim;
        let maze = generator.generate(21, 21, Some(42), None);
        assert_eq!(maze.width, 21);
        assert_eq!(maze.height, 21);
    }

    #[test]
    fn prim_all_path_cells_reachable() {
        let generator = Prim;
        let mut maze = generator.generate(21, 21, Some(42), None);
        maze.place_exit();
        let reachable = bfs_all_reachable(&maze, maze.start);
        for y in 0..maze.height {
            for x in 0..maze.width {
                if matches!(maze.grid[y][x], Tile::Path | Tile::Start | Tile::Exit) {
                    assert!(
                        reachable[y][x],
                        "prim: cell ({x}, {y}) should be reachable from start"
                    );
                }
            }
        }
    }

    #[test]
    fn prim_maze_is_solvable() {
        let generator = Prim;
        let mut maze = generator.generate(21, 21, Some(42), None);
        maze.place_exit();
        assert!(
            bfs_reachable(&maze, maze.start, maze.exit),
            "prim: exit must be reachable from start"
        );
    }

    #[test]
    fn prim_outer_border_is_all_walls() {
        let generator = Prim;
        let maze = generator.generate(21, 21, Some(42), None);
        for x in 0..maze.width {
            assert_eq!(maze.grid[0][x], Tile::Wall);
            assert_eq!(maze.grid[maze.height - 1][x], Tile::Wall);
        }
        for y in 0..maze.height {
            assert_eq!(maze.grid[y][0], Tile::Wall);
            assert_eq!(maze.grid[y][maze.width - 1], Tile::Wall);
        }
    }

    #[test]
    fn prim_same_seed_produces_identical_maze() {
        let generator = Prim;
        let maze1 = generator.generate(21, 21, Some(123), None);
        let maze2 = generator.generate(21, 21, Some(123), None);
        assert_eq!(maze1.grid, maze2.grid);
    }

    #[test]
    fn prim_custom_start_position() {
        let generator = Prim;
        let maze = generator.generate(21, 21, Some(42), Some((5, 5)));
        assert_eq!(maze.start, (5, 5));
        assert_eq!(maze.grid[5][5], Tile::Start);
    }

    #[test]
    fn prim_custom_start_all_cells_reachable() {
        let generator = Prim;
        let mut maze = generator.generate(21, 21, Some(42), Some((5, 5)));
        maze.place_exit();
        let reachable = bfs_all_reachable(&maze, maze.start);
        for y in 0..maze.height {
            for x in 0..maze.width {
                if matches!(maze.grid[y][x], Tile::Path | Tile::Start | Tile::Exit) {
                    assert!(
                        reachable[y][x],
                        "prim: cell ({x}, {y}) should be reachable from custom start"
                    );
                }
            }
        }
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
